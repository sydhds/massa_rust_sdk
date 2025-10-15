use clap::Parser;
use massa_rust_web3::{
    Address, BUILDNET_URL, ReadOnlyCall, ReadOnlyResult, execute_read_only_call,
};
use std::str::FromStr;

const CONTRACT_ADDRESS: &str = "AS1AArefHYqYd9KB8wcCkvgesDap2teidRdwPJA22DpZqFxPUxuY";
const CALLER_ADDRESS: &str = "AU12NTxUbAFvHzrLH3XKwxkNgsjPqiAadnbthJz2v1TuNJEyWU2Cx";

#[derive(Debug, Clone, Parser)]
#[command(about = "read_only_call example", long_about = None)]
pub struct Args {
    #[arg(long = "contract", default_value = CONTRACT_ADDRESS, help = "Hello SC address")]
    pub contract_address: String,

    #[arg(long = "caller", default_value = CALLER_ADDRESS, help = "Wallet address that will call the SC")]
    pub caller_address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let sc_address = Address::from_str(args.contract_address.as_str())
        .expect("Please provide a valid SC address");
    println!("SC address: {:?}", sc_address);
    let caller_address = Address::from_str(args.caller_address.as_str())
        .expect("Please provide a valid address (caller address)");
    println!("Caller address: {:?}", caller_address);

    // Call hello() function from SC

    let read_function = "hello".to_string();
    // TODO: Builder pattern
    let read_params_ = ReadOnlyCall {
        max_gas: 4294167295, // FIXME: MAX_GAS ?
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
    println!("Read (function: `{}`): {:#?}", read_function, hello);

    let res = match &hello[0].result {
        ReadOnlyResult::Ok(res) => String::from_utf16_lossy(bytemuck::cast_slice(res.as_slice())),
        ReadOnlyResult::Error(e) => e.clone(),
    };

    println!("result as utf-16 string: {:?}", res);
}
