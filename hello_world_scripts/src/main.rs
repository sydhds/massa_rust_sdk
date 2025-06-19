use massa_rust_web3::get_status;

#[tokio::main]
async fn main() {
    let node_status = get_status().await.unwrap();
    println!("Node status: {}", node_status);
}
