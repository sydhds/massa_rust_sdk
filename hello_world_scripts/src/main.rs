use std::path::{PathBuf};
use std::str::FromStr;
use dotenv::dotenv;
// internal
use massa_rust_web3::{
    BUILDNET_URL,
    deploy_smart_contract,
    KeyPair,
};

#[tokio::main]
async fn main() {

    dotenv().expect("Failed to load .env file");
    let pkey = std::env::var("PRIVATE_KEY").unwrap();
    let keypair = KeyPair::from_str(pkey.as_str()).unwrap();

    let wasm_path = PathBuf::from("target/wasm32-unknown-unknown/release/main.wasm");
    println!("Deploying: {:?}", wasm_path);

    let sc_address = deploy_smart_contract(
        BUILDNET_URL,
        &keypair,
        wasm_path.as_path(),
    ).await;

    println!("SC address: {:?}", sc_address);
}
