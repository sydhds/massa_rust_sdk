use std::path::{PathBuf};
use std::str::FromStr;
use dotenv::dotenv;
// internal
use massa_rust_web3::{BUILDNET_URL, deploy_smart_contract, KeyPair, DeployerArgs};

#[tokio::main]
async fn main() {

    dotenv().expect("Failed to load .env file");
    let pkey = std::env::var("PRIVATE_KEY").unwrap();
    let keypair = KeyPair::from_str(pkey.as_str()).unwrap();

    let wasm_path = PathBuf::from("target/wasm32-unknown-unknown/release/main.wasm");
    println!("Deploying: {:?}", wasm_path);

    let deploy_args = DeployerArgs {
        // Our smart contract constructor uses the blockchain storage to store data, need to pay for it
        coins: Some(10000000u64), // == 0.01 Massa
        ..Default::default()
    };

    let sc_address = deploy_smart_contract(
        BUILDNET_URL,
        &keypair,
        wasm_path.as_path(),
        deploy_args,
    ).await;

    println!("SC address: {:?}", sc_address);
}
