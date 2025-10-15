# Massa rust sdk

A rust sdk to develop smart contract for the [Massa blockchain](https://www.massa.net).

This sdk is still in development and very WIP, so expect breaking changes. You have been warned!

Early birds && developers are welcome to contribute. Please have a look at the [issues](https://github.com/sydhds/massa_rust_sdk/issues) and the [roadmap](https://github.com/sydhds/massa_rust_sdk/milestones).

## Architecture

Crates:

* [massa_rust_sc](massa_rust_sc): Helpers for Rust-written smart contract
* [massa_sc_runner](massa_sc_runner): Unit tests runner (Rust-written unit tests in smart contracts)
* [massa_rust_web3](massa_rust_web3): Crate for interacting with smart contract (aka JsonRPC & Grpc)

Examples:

* [hello_world](hello_world): Example of Rust smart contract
* [hello_world_scripts](hello_world_scripts): Example of scripts to interact with the hello_world smart contract example

## Quickstart

### Build (with nightly)

* RUSTFLAGS="-Ctarget-cpu=mvp -Ctarget-feature=+bulk-memory" cargo +nightly build -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown -p hello_world --release

Tested with `cargo 1.90.0-nightly (eabb4cd92 2025-07-09)`

### Build (without nightly)

* cargo build -p hello_world --target=wasm32-unknown-unknown --release
* ./build_post.sh

### Unit tests

* cargo +1.88 build -p massa_sc_runner
* RUSTFLAGS="-Ctarget-cpu=mvp -Ctarget-feature=+bulk-memory" cargo +nightly test -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown -p hello_world

Or manually:

* RUSTFLAGS="-Ctarget-cpu=mvp -Ctarget-feature=+bulk-memory" cargo +nightly test -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown -p hello_world --no-run
* cargo run -p massa_sc_runner -- target/wasm32-unknown-unknown/debug/deps/hello_world-XXXXX.wasm

Note: 
* Require Rust 1.88 (< Rust 1.89) to build massa_sc_runner until this bug is fixed: https://github.com/wasmerio/wasmer/issues/5610 and released. 
* Debug build of massa_sc_runner is used to run the tests. See [config.toml](.cargo/config.toml) for more details.

### Deploy

* cp -v .env.example .env
* \_\_EDIT\_\_ .env (with wallet info)
* cargo run --bin hello_world_scripts
* Debug:
  * RUST_LOG=debug cargo run --bin hello_world_scripts

### Other scripts

* cargo run -p hello_world_scripts --example get_operations -- \_\_OPERATION_ID_STRING\_\_ get_op.log

## Developing Massa smart contracts in Rust

Writing a dev book (similar to [the Rust book](https://github.com/rust-lang/book)) is planned, but in the meantime, you can refer to the following documentation & examples:

### Documentation (Smart contract development)

* cargo doc -p massa_rust_sc --target wasm32-unknown-unknown

### JsonRPC & Grpc call documentation

* cargo doc -p massa_rust_web3

### Smart contract examples 

* [hello_world](hello_world): use events && blockchain storage 

### JsonRPC examples

* [hello_world_scripts](hello_world_scripts): 
  * deploy hello_world SC
  * example - read_only_call: call hello() function from hello_world SC
  * example - get_status: call JsonRPC get_status function
  * example - get_operations: call JsonRPC get_operations function

## Support Our Mission 💎

We are not affiliated with [Massa Labs](https://www.massa.net) or [Massa Foundation](https://massa.foundation/).

### Why Your Support Matters

- Your continued support helps us provide regular updates and remain independent, so we can fulfill our mission
- Sustained funding is key to quickly releasing new features requested by you and other community members

Please also leave [a star](https://github.com/sydhds/massa_rust_sdk) on GitHub if you like this project. It provides additional motivation to keep going.

**A big thank you to all current and past sponsors, whose generous support has been and continues to be essential to the success of the project!**

[View Sponsors ›](SPONSORS.md)

### How to support us

* Crypto donations
  * BTC (network: BITCOIN): 1Gtq8FDSqw4VWxEhFDkMkfdGDa6sxJiaqf
  * Ethereum (+ USDT/ERC20, network: Ethereum): 0xa9d24971ee2ece918a4e5ed930da33e71a901bf9
  * Massa: AU1NGhiqutBytHdUE38bGeNYh9XnCzCCWdcvM6aTimceebKvJhpG

## Sdk developers

This section is for developers that want to hack on the massa rust sdk code itself.

## massa_rust_sdk

* RUSTFLAGS="-Ctarget-cpu=mvp -Ctarget-feature=+bulk-memory" cargo +nightly test -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown -p massa_rust_sc

## massa_rust_web3

* cargo test -p massa_rust_web3

## Others

* Clippy
  * cargo clippy -p massa_rust_sc --target wasm32-unknown-unknown
  * cargo clippy -p hello_world --target wasm32-unknown-unknown
  * cargo clippy -p massa_rust_web3 
  * cargo clippy -p massa_sc_runner
  * cargo clippy -p hello_world_scripts
* Fmt 
  * cargo fmt
