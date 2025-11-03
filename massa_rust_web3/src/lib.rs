mod deploy;
mod jsonrpc_common;
#[cfg(feature = "jsonrpsee")]
mod jsonrpc;
#[cfg(feature = "reqwest")]
mod jsonrpc_reqwest;
mod deploy_sc;


// exports
pub use jsonrpc_common::MassaJsonRpc;
#[cfg(feature = "jsonrpsee")]
pub use jsonrpc::MassaRpcClient;
#[cfg(feature = "reqwest")]
pub use jsonrpc_reqwest::MassaRpcClient;
pub use deploy_sc::{deploy_smart_contract, DeployerArgs};
// massa re exports
pub use massa_signature::KeyPair;
pub use massa_models::{
    operation::OperationId,
    address::Address,
};
pub use massa_api_exports::{
    node::NodeStatus,
    address::AddressInfo,
    execution::{ReadOnlyCall, ReadOnlyResult, ReadOnlyBytecodeExecution},
};

pub const BUILDNET_URL: &str = "https://buildnet.massa.net/api/v2";
pub const BUILDNET_CHAINID: u64 = 77658366;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use massa_models::address::Address;
    use super::*;

    #[tokio::test]
    async fn test_get_status() {

        let client = MassaRpcClient::new(BUILDNET_URL);
        let node_status = client.get_status().await.unwrap(); //  get_status(BUILDNET_URL).await.unwrap();
        println!("{}", "#".repeat(20));
        println!("Node status: {}", node_status);
        assert_eq!(node_status.chain_id, BUILDNET_CHAINID);
    }

    #[tokio::test]
    async fn test_get_addresses() {

        let addresses = vec![
            Address::from_str("AU1Yvq49utdezr496dHbRj3TMjqsCh2awggjfGraHoddE7XfEkpY").unwrap(),
        ];
        let client = MassaRpcClient::new(BUILDNET_URL);
        let addresses_info = client.get_addresses(addresses.clone()).await.unwrap();
        println!("{}", "#".repeat(20));
        println!("Addresses info: {:?}", addresses_info);
        assert_eq!(addresses_info.len(), 1);
        assert_eq!(addresses_info[0].address, addresses[0]);
        assert_eq!(addresses_info[0].thread, 9);
        // assert_eq!(node_status.chain_id, BUILDNET_CHAINID);
    }
}