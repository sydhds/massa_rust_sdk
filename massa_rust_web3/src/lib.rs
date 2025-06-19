use jsonrpsee::{
    core::{client::ClientT},
    http_client::HttpClientBuilder,
    rpc_params,
};
use jsonrpsee::core::client;
use massa_api_exports::node::NodeStatus;

pub async fn get_status() -> Result<NodeStatus, client::Error> { 

    let client = HttpClientBuilder::default()
        .build("https://buildnet.massa.net/api/v2")
        .unwrap();
    let params = rpc_params![];
    client.request("get_status", params).await 
}

