mod interface;

use std::io::Write;
// std
use std::path::PathBuf;
// third-party
use massa_sc_runtime::{Compiler, CondomLimits, GasCosts, Interface, RuntimeModule, run_function};
use tempfile::NamedTempFile;
// internal
use interface::MassaScRunnerInterface;

const UNIT_TEST_PREFIX: &str = "__MASSA_RUST_SDK_UNIT_TEST";

const GAS_COSTS_FILE : &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/abi_gas_costs.json"));

fn main() {

    // TODO: debug!
    println!("args: {:?}", std::env::args());
    // println!("Should run with wasm now...");
    let wasm_file = std::env::args().nth(1).unwrap();
    println!("Wasm file: {wasm_file}");

    let limit = u64::MAX;

    // Load gas costs
    let mut temp_file = NamedTempFile::new().expect("Cannot create temp file");
    temp_file.write_all(GAS_COSTS_FILE.as_bytes()).expect("Cannot write to temp file");
    temp_file.flush().expect("Cannot flush temp file");
    let temp_path = temp_file.path().to_path_buf();
    // Note: GasCosts can only be initialized from a file :-/
    let gas_costs = GasCosts::new(temp_path).expect("Failed to load gas costs");

    // let module = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/wasm/basic_func.wasm"));
    let bytecode = std::fs::read(wasm_file).unwrap();

    // List wasm functions
    // Note: cannot access Wasmer module (hidden in RuntimeModule struct from massa-sc-runtime)
    //       so we need to do it manually

    let unit_test_functions = get_wasm_functions(bytecode.as_slice());

    for f in unit_test_functions {

        println!("Running unit test: {f}");

        let exec_limits = CondomLimits::default();
        let interface: Box<dyn Interface> = Box::new(MassaScRunnerInterface::default());

        let runtime_module = RuntimeModule::new(
            bytecode.as_slice(),
            gas_costs.clone(),
            Compiler::SP,
            exec_limits.clone(),
        )
            .unwrap();

        let res = run_function(
            &*interface,
            runtime_module,
            f.as_str(),
            &[],
            limit,
            gas_costs.clone(),
            exec_limits.clone(),
        );

        println!("wasm vm res: {res:?}");
    }
}

fn get_wasm_functions(wasm_content: &[u8]) -> Vec<String> {

    use wasmer::{Engine, Module, Store, ExternType};

    let engine = Engine::default();
    let store = Store::new(engine);
    let module = Module::new(&store, wasm_content).unwrap();

    module
        .exports()
        .filter_map(|export| {
            if let ExternType::Function(_f) = export.ty() && export.name().starts_with(UNIT_TEST_PREFIX) {
                Some(export.name().to_string())
            } else {
                None
            }
        })
        .collect()
}