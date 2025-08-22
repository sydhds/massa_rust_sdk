mod interface;

// std
use std::path::PathBuf;
// third-party
use massa_sc_runtime::{Compiler, CondomLimits, GasCosts, Interface, RuntimeModule, run_function};
// internal
use interface::MassaScRunnerInterface;

const UNIT_TEST_PREFIX: &str = "__MASSA_RUST_SDK_UNIT_TEST";

fn main() {
    // TODO: debug!
    println!("args: {:?}", std::env::args());
    // println!("Should run with wasm now...");
    let wasm_file = std::env::args().nth(1).unwrap();
    println!("Wasm file: {wasm_file}");

    let limit = u64::MAX;

    // let gas_costs = GasCosts::default();
    let gas_costs = GasCosts::new(PathBuf::from(
        "massa_sc_runner/resources/abi_gas_costs.json",
    ))
    .expect("Failed to load gas costs");

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
            if let ExternType::Function(f) = export.ty() && export.name().starts_with(UNIT_TEST_PREFIX) {
                Some(f.name().to_string())
            } else {
                None
            }
        })
        .collect()
}