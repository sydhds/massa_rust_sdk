use dotenv::dotenv;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{Level, debug, info};
use tracing_subscriber::{EnvFilter, fmt};
// internal
use massa_rust_web3::{BUILDNET_URL, DeployerArgs, KeyPair, deploy_smart_contract};

#[tokio::main]
async fn main() {
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(Level::INFO) // default if `RUST_LOG` is not set
        .with_env_filter(EnvFilter::from_default_env()) // RUST_LOG env var
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    dotenv().expect("Failed to load .env file");
    let pkey = std::env::var("PRIVATE_KEY").unwrap();
    let keypair = KeyPair::from_str(pkey.as_str()).unwrap();

    let wasm_path = PathBuf::from("target/wasm32-unknown-unknown/release/main.wasm");
    info!("Deploying: {:?}", wasm_path);

    let deploy_args = DeployerArgs {
        // Our smart contract constructor uses the blockchain storage to store data, need to pay for it
        coins: Some(10000000u64), // == 0.01 Massa
        ..Default::default()
    };

    debug!("deployer arguments: {:?}", deploy_args);

    let sc_address =
        deploy_smart_contract(BUILDNET_URL, &keypair, wasm_path.as_path(), deploy_args).await;

    info!("SC address: {:?}", sc_address);
}
