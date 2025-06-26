mod deploy;

use std::collections::VecDeque;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
// third-party
use jsonrpsee::core::{Serialize, client};
use jsonrpsee::tokio::io::AsyncReadExt;
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder, rpc_params, tokio};
use serde::Deserialize;
// massa
use massa_api_exports::node::NodeStatus;
use massa_api_exports::operation::{OperationInfo, OperationInput};
use massa_models::operation::SecureShareOperation;
// massa re exports
pub use massa_api_exports::execution::{ExecuteReadOnlyResponse, ReadOnlyCall, ReadOnlyResult};
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
pub use massa_signature::KeyPair;
// internal
use crate::deploy::DEPLOYER_BYTECODE;

pub const BUILDNET_URL: &str = "https://buildnet.massa.net/api/v2";
pub const BUILDNET_CHAINID: u64 = 77658366;

pub async fn get_status(url: impl AsRef<str>) -> Result<NodeStatus, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;
    let params = rpc_params![];
    client.request("get_status", params).await
}

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

    // let keypair = KeyPair::generate(0).unwrap();
    // let operation = create_execute_sc_op_with_too_much_gas(&keypair, 10);

    let operation: SecureShareOperation = {
        /*
        let op = OperationType::ExecuteSC {
            data: Vec::new(),
            max_gas: (u32::MAX - 1) as u64,
            max_coins: Amount::default(),
            datastore: Datastore::default(),
        };
        let content = Operation {
            fee: Amount::default(),
            op,
            expire_period,
        };
        */

        // const CHAINID_BUILDNET: u64 = 77658366;
        Operation::new_verifiable(operation, OperationSerializer::new(), keypair, BUILDNET_CHAINID).unwrap()
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

const MAX_GAS_EXECUTE: u64 = 3980167295;
const MAX_GAS_DEPLOYMENT: u64 = 3980167295;
const PERIOD_TO_LIVE_DEFAULT: u64 = 9;

pub async fn deploy_smart_contract(
    url: impl AsRef<str> + Clone,
    key_pair: &KeyPair,
    smart_contract: &Path,
) -> Result<Address, client::Error> {
    // TODO: no unwrap
    let mut fs = tokio::fs::File::open(smart_contract).await.unwrap();
    let mut file_content = Vec::new();
    // TODO: no unwrap
    fs.read_to_end(&mut file_content).await.unwrap();

    //
    // TODO: function populateDatastore
    //       node_modules/@massalabs/massa-web3/dist/cmd/smartContracts/deployerUtils.js

    let ds = {
        const CONTRACTS_NUMBER_KEY: [u8; 1] = [0u8];
        let mut ds = Datastore::default();

        ds.insert(CONTRACTS_NUMBER_KEY.to_vec(), 1u64.to_le_bytes().to_vec());
        // TODO: deploy 1+ SC
        // massa-web3 function 'contractKey'
        ds.insert(1u64.to_le_bytes().to_vec(), file_content);
        // massa-web3 function 'argsKey'
        // TODO: Args
        let mut argsKey = 1u64.to_le_bytes().to_vec();
        // FIXME
        argsKey.extend_from_slice(&[1, 0, 0, 0, 0]);
        // XXX: Args encoded 'Massa'
        ds.insert(argsKey, [
            5,   0,   0,  0, 77,
            97, 115, 115, 97
        ].to_vec());
        // massa-web3 function 'coinsKey'
        let mut coinsKey = 1u64.to_le_bytes().to_vec();
        // FIXME
        coinsKey.extend_from_slice(&[1, 0, 0, 0, 1]);
        // FIXME:
        ds.insert(coinsKey, [
            128, 150, 152, 0,
            0,   0,   0, 0
        ].to_vec());

        ds
    };

    println!("ds: {:?}", ds
        .iter()
        .filter(|(k, v)| {
            **k != 1u64.to_le_bytes().to_vec() 
        }).collect::<Vec<_>>()
    );
    println!("ds len: {:?}", ds.len());

    let op = OperationType::ExecuteSC {
        data: DEPLOYER_BYTECODE.to_vec(),
        max_gas: MAX_GAS_DEPLOYMENT,
        // max_coins: Amount::from_str("897450289").unwrap(), // TODO
        max_coins: Amount::from_raw(897450289), // TODO
        datastore: ds,
    };

    // println!("op: {}", op);

    // node_modules/@massalabs/massa-web3/dist/cmd/client/publicAPI.js
    // function fetchPeriod
    let status = get_status(url.clone()).await.unwrap();
    let last_slot = status.last_slot.unwrap();
    println!("last_slot: {}", last_slot);
    println!("period to live: {}", PERIOD_TO_LIVE_DEFAULT);
    let expire_period = last_slot.period + PERIOD_TO_LIVE_DEFAULT;

    let content = Operation {
        fee: Amount::from_str("0.01").unwrap(),
        op,
        expire_period,
    };

    println!("content fee: {:?} - raw: {}", content.fee, content.fee.to_raw());
    println!("content ex period: {:?}", content.expire_period);
    // panic!();

    let op_id = send_operations(url.clone(), content, key_pair).await?;
    println!("operation ids: {:?}", op_id);
    println!("operation ids: {:?}", op_id[0]);

    // FIXME: wait is final
    // println!("Wait...");
    // tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    // println!("awaited...");

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

    println!("elapsed time: {:?}", start.elapsed());
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

    println!("events: {:#?}", events);
    
    const EVENT_CONSTRUCTOR_ADDRESS_PREFIX: &str = "Contract deployed at address: ";
    let addr: Vec<Address> = events
        .iter()
        .filter_map(|event| {
            if event.data.starts_with(EVENT_CONSTRUCTOR_ADDRESS_PREFIX) {
                let addr_str = &event.data[EVENT_CONSTRUCTOR_ADDRESS_PREFIX.len()..];
                Some(Address::from_str(addr_str).unwrap())
            } else {
                None
            }
        })
        .collect();

    // FIXME: no unwrap
    Ok(addr.get(0).unwrap().clone())
}
