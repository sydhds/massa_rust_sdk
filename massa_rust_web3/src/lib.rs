mod deploy;

use std::collections::VecDeque;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
// third-party
use jsonrpsee::core::{Serialize, client};
use jsonrpsee::tokio::io::AsyncReadExt;
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder, rpc_params, tokio};
use serde::Deserialize;
use tracing::debug;
// massa
use massa_api_exports::node::NodeStatus;
use massa_api_exports::operation::{OperationInfo, OperationInput};
use massa_models::operation::SecureShareOperation;
// massa re exports
use massa_api_exports::execution::ReadOnlyBytecodeExecution;
pub use massa_api_exports::execution::{ExecuteReadOnlyResponse, ReadOnlyCall, ReadOnlyResult};
use massa_models::datastore::DatastoreSerializer;
pub use massa_models::{
    address::Address,
    amount::Amount,
    config::CHAINID,
    datastore::Datastore,
    execution::EventFilter,
    operation::{Operation, OperationId, OperationSerializer, OperationType},
    output_event::SCOutputEvent,
    secure_share::SecureShareContent,
    slot::Slot,
};
use massa_serialization::{SerializeError, Serializer};
pub use massa_signature::KeyPair;
// use reqwest::Url;
// internal
use crate::deploy::DEPLOYER_BYTECODE;

pub const BUILDNET_URL: &str = "https://buildnet.massa.net/api/v2";
pub const BUILDNET_CHAINID: u64 = 77658366;

/*
trait MassaJsonRpc {

    type Error;

    async fn post<P, R, E: std::error::Error>(&self, url: impl AsRef<str>, params: P) -> Result<R, E>;

    fn empty_params<P>() -> P;

    async fn get_status<P, E: std::error::Error>(&self, url: impl AsRef<str>) -> Result<NodeStatus, E> {
        // let params = Self::empty_params();
        self.post(url, Self::empty_params()).await
    }
}
*/

pub async fn get_status(url: impl AsRef<str>) -> Result<NodeStatus, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;
    let params = rpc_params![];
    client.request("get_status", params).await
}

/*
pub async fn get_status_reqwest(url: impl AsRef<str>) -> Result<NodeStatus, client::Error> {
    // let client = HttpClientBuilder::default().build(url)?;
    // let params = rpc_params![];
    // client.request("get_status", params).await
    let client = reqwest::Client::new();
    let res_0 = client.post(Url::from_str(url.as_ref()).unwrap())
        // .json()
        .header("Content-Type", "application/json")
        .body("")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("resp text: {:?}", res_0);
    let res = client.post(Url::from_str(url.as_ref()).unwrap())
        // .json()
        .send()
        .await
        .unwrap()
        .json::<NodeStatus>()
        .await
        .unwrap();
    Ok(res)
}
*/

pub async fn get_filtered_sc_output_event(
    url: impl AsRef<str>,
    event_filter: EventFilter,
) -> Result<Vec<SCOutputEvent>, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;
    client
        .request("get_filtered_sc_output_event", rpc_params![event_filter])
        .await
}

/// Shortcut for `get_filtered_sc_output_event`
pub async fn get_events(
    url: impl AsRef<str>,
    event_filter: EventFilter,
) -> Result<Vec<SCOutputEvent>, client::Error> {
    get_filtered_sc_output_event(url, event_filter).await
}

/*
#[derive(Debug, Clone)]
pub struct ReadOnlyCallParams(pub ReadOnlyCall);

impl Default for ReadOnlyCallParams {
    fn default() -> Self {
        // TODO: Zero address
        Self(ReadOnlyCall {
            max_gas: 0,
            target_address: Address::from_str(
                "AU12fZLkHnLED3okr8Lduyty7dz9ZKkd24xMCc2JJWPcdmfn2eUEx",
            )
            .unwrap(),
            target_function: "".to_string(),
            parameter: vec![],
            caller_address: None,
            coins: None,
            fee: None,
        })
    }
}

impl Deref for ReadOnlyCallParams {
    type Target = ReadOnlyCall;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
*/

/// Lightweight version of `ExecuteReadOnlyResponse` (Massa struct)
///
/// execute_read_only_call response deserialization will fail without this
/// Might be related to Massa issue: https://github.com/massalabs/massa/issues/4775
/// massa-web3 (js) has its own formatter formatReadOnlyExecuteSCParams to circumvent this
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecuteReadOnlyResponseLw {
    pub executed_at: Slot,
    pub result: ReadOnlyResult,
    pub output_events: VecDeque<SCOutputEvent>,
    pub gas_cost: u64,
}

pub async fn execute_read_only_call(
    url: impl AsRef<str>,
    read_params: Vec<ReadOnlyCall>,
) -> Result<Vec<ExecuteReadOnlyResponseLw>, client::Error> {
    // Note: Massa issue: https://github.com/massalabs/massa/issues/4775

    let client = HttpClientBuilder::default().build(url)?;

    // response type: Result<Vec<ExecuteReadOnlyResponseLw>, _>
    client
        .request("execute_read_only_call", rpc_params![read_params])
        .await
}

pub async fn execute_read_only_bytecode(
    url: impl AsRef<str>,
    read_params: Vec<ReadOnlyBytecodeExecution>,
) -> Result<Vec<ExecuteReadOnlyResponseLw>, client::Error> {
    // Note: Massa issue: https://github.com/massalabs/massa/issues/4775

    let client = HttpClientBuilder::default().build(url)?;
    client
        .request("execute_read_only_bytecode", rpc_params![read_params])
        .await
}

pub async fn get_operations(
    url: impl AsRef<str>,
    operation_ids: Vec<OperationId>,
) -> Result<Vec<OperationInfo>, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;
    let params = rpc_params![operation_ids];
    let response: Vec<OperationInfo> = client.request("get_operations", params).await?;

    Ok(response)
}

pub async fn send_operations(
    url: impl AsRef<str>,
    operation: Operation,
    keypair: &KeyPair,
) -> Result<Vec<OperationId>, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;

    let operation: SecureShareOperation = {
        Operation::new_verifiable(
            operation,
            OperationSerializer::new(),
            keypair,
            BUILDNET_CHAINID,
        )
        .unwrap()
    };

    let input: OperationInput = OperationInput {
        creator_public_key: keypair.get_public_key(),
        signature: operation.signature,
        serialized_content: operation.serialized_data,
    };

    let response: Result<Vec<OperationId>, _> = client
        .request("send_operations", rpc_params![vec![input]])
        .await;

    response
}

#[derive(thiserror::Error, Debug)]
pub enum DeployError {
    #[error("Cannot read file: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Client(#[from] client::Error),
    #[error("Unable to serialize the datastore (for gas estimation)")]
    Serialize(#[from] SerializeError),
    #[error("Unable to estimate gas cost, response: {0:?}")]
    InvalidGasEstimation(Box<Vec<ExecuteReadOnlyResponseLw>>),
    #[error("Max gas == {0:?} but should be > {1} && < {2}")]
    Gas(u64, u64, u64),
    #[error("Invalid address retrieved from events: {0}")]
    InvalidAddress(String),
    #[error("Unable to retrieve the address of the deployed smart contract from events")]
    AddressNotFound,
}

#[derive(Debug, Clone, Default)]
pub struct DeployerArgs {
    /// Arguments for the deployed smart contract constructor function
    /// Default to: no arguments
    pub constructor_arguments: Option<Vec<u8>>, // TODO: Args struct
    /// Coins (used to pay for storage when the deployed smart contract constructor is called)
    /// Only required if the smart contract constructor stores data in its storage.
    /// Default to: 0
    pub coins: Option<u64>,
    /// Fee for the deploy transaction (If None, use the minimal fee fetched from the rpc)
    pub fee: Option<Amount>,
    /// Max gas to use for the deploy transaction (If None, gas will be estimated)
    pub max_gas: Option<u64>,
}

#[allow(clippy::collapsible_if)]
#[allow(clippy::manual_range_contains)]
pub async fn deploy_smart_contract(
    url: impl AsRef<str> + Clone,
    key_pair: &KeyPair,
    smart_contract: &Path,
    args: DeployerArgs,
) -> Result<Address, DeployError> {
    // From: node_modules/@massalabs/massa-web3/dist/cmd/smartContracts/constants.d.ts
    // const MAX_GAS_CALL: u64 = 4294167295;

    const MIN_GAS_CALL: u64 = 2100000;
    const MAX_GAS_EXECUTE: u64 = 3980167295;
    // const MAX_GAS_DEPLOYMENT: u64 = 3980167295;
    const PERIOD_TO_LIVE_DEFAULT: u64 = 9;

    // Read the smart contract we want to deploy
    let mut file_content = Vec::new();
    let mut fs = tokio::fs::File::open(smart_contract).await?;
    fs.read_to_end(&mut file_content).await?;
    let file_content_len = file_content.len();

    //
    // TODO: function populateDatastore
    //       node_modules/@massalabs/massa-web3/dist/cmd/smartContracts/deployerUtils.js

    // /home/sydh/dev/perso/massa_rust_sdk/massa-hello-world/node_modules/@massalabs/massa-web3/dist/cmd/provider/jsonRpcProvider/jsonRpcProvider.js

    // Datastore for ExecuteSC operation
    // require the following keys:
    // * CONTRACTS_NUMBER_KEY: the number of contracts we want to deploy
    // And for each contract, we want to deploy:
    // * contract_key: the contract bytecode we want to deploy
    // * args_key: the arguments for the constructor ??
    // * coins_key: the coins we want to pay for the deployment ??
    let ds = {
        let mut ds = Datastore::default();

        // Defined as: ```const CONTRACTS_NUMBER_KEY = new Uint8Array([0]);```
        const CONTRACTS_NUMBER_KEY: [u8; 1] = [0u8];
        ds.insert(CONTRACTS_NUMBER_KEY.to_vec(), 1u64.to_le_bytes().to_vec());

        // massa-web3 function 'contractKey' (in node_modules/@massalabs/massa-web3/dist/cmd/smartContracts/deployerUtils.js)
        // 1u64 -> SC index that we want to deploy (starting at 1)
        let contract_key = 1u64.to_le_bytes().to_vec();
        ds.insert(contract_key, file_content);

        // massa-web3 function 'argsKey'
        // 1u64 -> SC index that we want to deploy (starting at 1)
        // + a Uint8Array of length 1: [0] (so [1, 0, 0, 0] for the size + [0] for the value
        let mut args_key = 1u64.to_le_bytes().to_vec();
        args_key.extend_from_slice(&[1, 0, 0, 0, 0]);
        // Arguments expected by the deployed smart contract constructor function
        // XXX: Args encoded 'Massa'
        /*
        ds.insert(argsKey, [
            5,   0,   0,  0, 77,
            97, 115, 115, 97
        ].to_vec());
        */
        ds.insert(args_key, args.constructor_arguments.unwrap_or_default());

        // massa-web3 function 'coinsKey'
        // 1u64 -> SC index that we want to deploy (starting at 1)
        // + a Uint8Array of length 1: [1] (so [1, 0, 0, 0] for the size + [1] for the value
        let mut coins_key = 1u64.to_le_bytes().to_vec();
        coins_key.extend_from_slice(&[1, 0, 0, 0, 1]);
        // Coins as u64 (serialized as LE bytes)
        /*
        ds.insert(coins_key, [
            128, 150, 152, 0,
            0,   0,   0, 0
        ].to_vec());
        */
        // let coins_value = 10000000u64.to_le_bytes().to_vec();
        let coins_value = args.coins.unwrap_or(0u64).to_le_bytes().to_vec();
        ds.insert(coins_key, coins_value);

        ds
    };

    // println!("ds: {:?}", ds
    //     .iter()
    //     .filter(|(k, _v)| {
    //         **k != 1u64.to_le_bytes().to_vec()
    //     }).collect::<Vec<_>>()
    // );
    // println!("ds len: {:?}", ds.len());

    // max_coins
    // == Max amount of coins allowed to be spent by the execution
    let max_coins: Amount = {
        // node_modules/@massalabs/massa-web3/dist/cmd/basicElements/storage.js
        const ACCOUNT_SIZE_BYTES: u64 = 10;

        // == Amount::from_str("0.0001").unwrap();
        const STORAGE_BYTE_COST: Amount = Amount::from_raw(100000);

        let account_cost = STORAGE_BYTE_COST
            .checked_mul_u64(ACCOUNT_SIZE_BYTES)
            .unwrap();

        STORAGE_BYTE_COST
            .checked_mul_u64(file_content_len as u64)
            .unwrap()
            .checked_add(account_cost)
            .unwrap()
            .checked_add(Amount::from_raw(args.coins.unwrap_or(0u64)))
            .unwrap()
    };
    debug!("max coins: {:?}", max_coins);
    debug!("max coins: {:?}", max_coins.to_raw());
    //

    // node_modules/@massalabs/massa-web3/dist/cmd/client/publicAPI.js
    // function fetchPeriod
    let status = get_status(url.clone()).await?;
    // Note: get_status should always return a valid last_slot
    let last_slot = status.last_slot.expect("get_status last_slot is None");
    debug!("last_slot: {}", last_slot);
    debug!("period to live: {}", PERIOD_TO_LIVE_DEFAULT);
    let expire_period = last_slot.period + PERIOD_TO_LIVE_DEFAULT;

    let minimal_fee = status.minimal_fees;
    if let Some(fee) = args.fee {
        if fee < minimal_fee {
            // TODO: warn!
            println!("Fee is too low: {} (minimal fee: {})", fee, minimal_fee);
        }
    }

    // max_gas

    let max_gas = {
        let max_gas = match args.max_gas {
            Some(max_gas) => max_gas,
            None => {
                debug!("Estimating gas cost...");
                let ds_serializer = DatastoreSerializer::new();
                let mut buffer = Vec::new();
                ds_serializer.serialize(&ds, &mut buffer)?;

                let read_params = vec![ReadOnlyBytecodeExecution {
                    max_gas: MAX_GAS_EXECUTE,
                    bytecode: DEPLOYER_BYTECODE.to_vec(),
                    address: Some(Address::from_public_key(&key_pair.get_public_key())),
                    operation_datastore: Some(buffer),
                    fee: Some(args.fee.unwrap_or(minimal_fee)),
                }];

                let res = execute_read_only_bytecode(url.clone(), read_params).await?;
                debug!("Estimating gas cost: res: {:?}", res);
                if let Some(res) = res.first() {
                    // TODO: massa-web3 use a 20% margin for gas estimation,
                    // but this is working for deployment?
                    res.gas_cost
                } else {
                    return Err(DeployError::InvalidGasEstimation(Box::new(res)));
                }
            }
        };

        if max_gas < MIN_GAS_CALL || max_gas > MAX_GAS_EXECUTE {
            return Err(DeployError::Gas(max_gas, MIN_GAS_CALL, MAX_GAS_EXECUTE));
        }

        max_gas
    };

    //

    // Execute Deployer SC that will deploy our smart contract

    // https://docs.massa.net/docs/learn/operation-format-execution#executesc-operation-payload
    let op = OperationType::ExecuteSC {
        data: DEPLOYER_BYTECODE.to_vec(),
        max_gas,
        max_coins,
        datastore: ds,
    };

    // println!("op: {}", op);

    let content = Operation {
        // fee: Amount::from_str("0.01").unwrap(), // FIXME
        fee: args.fee.unwrap_or(minimal_fee),
        op,
        expire_period,
    };

    debug!(
        "content fee: {:?} - raw: {}",
        content.fee,
        content.fee.to_raw()
    );
    debug!("content ex period: {:?}", content.expire_period);
    // panic!();

    let op_id = send_operations(url.clone(), content, key_pair).await?;
    debug!("operation ids: {:?}", op_id);
    debug!("operation ids: {:?}", op_id[0]);

    // FIXME: wait is final
    // println!("Wait...");
    // tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    // println!("awaited...");

    // TODO: separate deploy from events retrieval?
    // TODO: rework waiting loop

    /*
    let mut c = 0;
    let start = std::time::Instant::now();
    loop {
        let status = get_operations(url.clone(), op_id.clone()).await;
        // println!("status: {:?}", status);
        if let Ok(status) = status {
            if status.len() > 0 {

                if status[0].op_exec_status.is_some() || status[0].is_operation_final == Some(true) {
                    println!("exec done or is_final: {}", status[0]);
                    break;
                }

                /*
                // println!("operation status: {:?}", status);
                if let Some(exec_info) = status[0].op_exec_status {
                    match exec_info {
                        true => {
                            println!("exec_info: {:?}", exec_info);
                            println!("Exiting Ok");
                            break
                        },
                        false => panic!("failed op: {:?}", status),
                    }
                }
                */
            }

        }
        // println!("Waiting for status...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        c += 1;
        if c >= 30 {
            println!("Exiting timeout");
            break;
        }
    }
    */

    const DEFAULT_WAIT_TIMEOUT_MS: Duration = Duration::from_millis(60000);
    const DEFAULT_WAIT_PERIOD_MS: Duration = Duration::from_millis(500);

    let start = std::time::Instant::now();
    loop {
        let status = get_operations(url.clone(), op_id.clone()).await;
        if let Ok(status) = status {
            if !status.is_empty() {
                if status[0].op_exec_status.is_some() || status[0].is_operation_final == Some(true)
                {
                    // println!("exec done or is_final: {}", status[0]);
                    break;
                }
            }
        }

        tokio::time::sleep(DEFAULT_WAIT_PERIOD_MS).await;
        if start.elapsed() > DEFAULT_WAIT_TIMEOUT_MS {
            // println!("Exiting timeout");
            break;
        }
    }

    // println!("elapsed time: {:?}", start.elapsed());
    let event_filter = EventFilter {
        start: None,
        end: None,
        emitter_address: None,
        original_caller_address: None,
        original_operation_id: Some(op_id[0]),
        // is_final: Some(true),
        is_final: None,
        is_error: None,
    };
    let events = get_events(url, event_filter).await?;

    debug!("events: {:#?}", events);

    const EVENT_CONSTRUCTOR_ADDRESS_PREFIX: &str = "Contract deployed at address: ";
    let addr: Vec<&str> = events
        .iter()
        .filter_map(|event| {
            if event.data.starts_with(EVENT_CONSTRUCTOR_ADDRESS_PREFIX) {
                let addr_str = &event.data[EVENT_CONSTRUCTOR_ADDRESS_PREFIX.len()..];
                Some(addr_str)
            } else {
                None
            }
        })
        .collect();

    let addr_1 = addr.first().ok_or(DeployError::AddressNotFound)?;
    Address::from_str(addr_1).map_err(|_e| DeployError::InvalidAddress(addr_1.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_status() {
        let node_status = get_status(BUILDNET_URL).await.unwrap();
        println!("{}", "#".repeat(20));
        println!("Node status: {}", node_status);
        assert_eq!(node_status.chain_id, BUILDNET_CHAINID);
    }
}
