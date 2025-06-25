use std::collections::VecDeque;
use std::path::Path;
use std::str::FromStr;
use dotenv::dotenv;
// third-party
use serde::Deserialize;
// internal
use massa_rust_web3::{Address, BUILDNET_URL, EventFilter, ReadOnlyCall, ReadOnlyResult, SCOutputEvent, Slot, execute_read_only_call, get_events, get_status, deploy_smart_contract, KeyPair};

const CONTRACT_ADDRESS: &str = "AS1AArefHYqYd9KB8wcCkvgesDap2teidRdwPJA22DpZqFxPUxuY";
const CALLER_ADDRESS: &str = "AU12NTxUbAFvHzrLH3XKwxkNgsjPqiAadnbthJz2v1TuNJEyWU2Cx";

#[derive(Clone, Debug, Deserialize)]
struct MyResp {
    pub executed_at: Slot,
    pub result: ReadOnlyResult,
    pub output_events: VecDeque<SCOutputEvent>,
    pub gas_cost: u64,
}

#[tokio::main]
async fn main() {
    let sc_address = Address::from_str(CONTRACT_ADDRESS).unwrap();
    println!("SC address: {:?}", sc_address);
    let caller_address = Address::from_str(CALLER_ADDRESS).unwrap();
    println!("Caller address: {:?}", caller_address);

    /*
    let node_status = get_status(BUILDNET_URL).await.unwrap();
    println!("{}", "#".repeat(20));
    println!("Node status: {}", node_status);
    */

    /*
    let event_filter = EventFilter {
        start: None,
        end: None,
        emitter_address: Some(sc_address),
        original_caller_address: None,
        original_operation_id: None,
        is_final: None,
        is_error: None,
    };
    let events = get_events(BUILDNET_URL, event_filter).await.unwrap();

    println!("{}", "#".repeat(20));
    println!("Events for SC ({}): {:#?}", sc_address, events);
    */

    /*
    let read_function = "hello".to_string();
    // TODO: Builder pattern for ReadOnlyCallParams
    // TODO: define MAX_GAS const
    let read_params_ = ReadOnlyCall {
        max_gas: 4294167295,
        target_address: sc_address,
        target_function: read_function.clone(),
        parameter: vec![],
        caller_address: Some(caller_address),
        coins: None,
        // fee: Some(Amount::from_str("0.01").unwrap()),
        // TODO / FIXME: should always be None otherwise got error
        //               Runtime error: spending address AUXXXXX... not found
        fee: None,
    };

    let hello = execute_read_only_call(BUILDNET_URL, vec![read_params_])
        .await
        .unwrap();
    println!("{}", "#".repeat(20));
    println!("Read (function: {}): {:#?}", read_function, hello);
    */

    dotenv().unwrap();
    let pkey = std::env::var("PRIVATE_KEY").unwrap();
    let keypair = KeyPair::from_str(pkey.as_str()).unwrap();

    let sc_address = deploy_smart_contract(
        BUILDNET_URL,
        &keypair,
        Path::new("target/wasm32-unknown-unknown/release/main.wasm")
    ).await;
    
    println!("SC address: {:?}", sc_address);
}
