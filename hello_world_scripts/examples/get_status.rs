use massa_rust_web3::{
    BUILDNET_URL,
    get_status,
};

#[tokio::main]
async fn main() {
    let node_status = get_status(BUILDNET_URL).await.unwrap();
    println!("{}", "#".repeat(20));
    println!("Node status: {}", node_status);
}