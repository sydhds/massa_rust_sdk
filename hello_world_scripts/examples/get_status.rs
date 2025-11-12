use massa_rust_web3::{BUILDNET_URL, MassaJsonRpc, MassaRpcClient};

#[tokio::main]
async fn main() {
    let client = MassaRpcClient::new(BUILDNET_URL);
    let node_status = client.get_status().await.unwrap();
    println!("{}", "#".repeat(20));
    println!("Node status: {}", node_status);
    println!("{}", "#".repeat(35));
}
