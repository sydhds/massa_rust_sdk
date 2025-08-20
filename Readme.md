# Massa rust sdk

A rust sdk to develop smart contract for the [Massa blockchain](www.massa.net).

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

* cargo test --target=wasm32-unknown-unknown -p hello_world --no-run
* cargo run -p massa_sc_runner -- target/wasm32-unknown-unknown/debug/deps/hello_world-XXXXX.wasm

## Deploy

* cp -v .env.example .env
* \_\_EDIT\_\_ .env (with wallet info)
* cargo run --bin hello_world_scripts

### Other scripts

* cargo run -p hello_world_scripts --example get_operations -- \_\_OPERATION_ID_STRING\_\_ get_op.log
