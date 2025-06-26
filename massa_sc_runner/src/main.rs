mod interface;

use std::path::PathBuf;
use massa_sc_runtime::{run_function, Compiler, CondomLimits, GasCosts, Interface, RuntimeModule};
use interface::MassaScRunnerInterface;

fn main() {
    
    println!("args: {:?}", std::env::args());
    // println!("Should run with wasm now...");
    let wasm_file = std::env::args().nth(1).unwrap();
    println!("Should run wasm file: {}", wasm_file);

    let function = "test_unit_xax";
    let limit = u64::MAX;

    // let gas_costs = GasCosts::default();
    let gas_costs = GasCosts::new(PathBuf::from("massa_sc_runner/resources/abi_gas_costs.json"))
        .expect("Failed to load gas costs");

    let exec_limits = CondomLimits::default();
    let interface: Box<dyn Interface> = Box::new(MassaScRunnerInterface::default());
    // let module = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/wasm/basic_func.wasm"));
    let bytecode = std::fs::read(wasm_file).unwrap();

    let runtime_module = RuntimeModule::new(
        bytecode.as_slice(),
        gas_costs.clone(),
        Compiler::SP,
        exec_limits.clone(),
    ).unwrap();

    let res = run_function(
        &*interface,
        runtime_module,
        function,
        &[],
        limit,
        gas_costs,
        exec_limits
    );
    
    println!("res: {:?}", res);
}
