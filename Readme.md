# Massa rust sdk

A rust sdk to develop smart contract for the [Massa blockchain](https://www.massa.net).

This sdk is still in development and very WIP, so expect breaking changes. You have been warned!
Rust developers are welcome to contribute. Please have a look at the issues and the roadmap.

## Architecture

* massa_rust_sc: Helpers for Rust-written smart contract
* massa_sc_runner: Unit tests runner (Rust-written unit tests in smart contracts)
* massa_rust_web3: Crate for interacting with smart contract (aka JsonRPC & Grpc)
* hello_world: Example of Rust smart contract
* hello_world_scripts: Example of scripts to interact with the hello_world smart contract example

## Quickstart

### Build

* cargo build -p hello_world --target=wasm32-unknown-unknown --release

### Post build

* ./build_post.sh

### Unit tests

* cargo build -p massa_sc_runner
* cargo test --target=wasm32-unknown-unknown -p hello_world

Or manually:

* cargo test --target=wasm32-unknown-unknown -p hello_world --no-run
* cargo run -p massa_sc_runner -- target/wasm32-unknown-unknown/debug/deps/hello_world-XXXXX.wasm

Note: 
* Require Rust 1.88 until this bug is fixed: https://github.com/wasmerio/wasmer/issues/5610 and released.
* Debug build of massa_sc_runner is used to run the tests. See [config.toml](.cargo/config.toml) for more details.

### Deploy

* cp -v .env.example .env
* \_\_EDIT\_\_ .env (with wallet info)
* cargo run --bin hello_world_scripts

### Other scripts

* cargo run -p hello_world_scripts --example get_operations -- \_\_OPERATION_ID_STRING\_\_ get_op.log

## Developing Massa smart contracts in Rust

Writing a handbook is planned, but in the meantime, you can refer to the following documentation & examples:

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