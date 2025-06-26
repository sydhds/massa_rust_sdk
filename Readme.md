# Massa rust sdk

A rust sdk to develop smart contract for [Massa blockchain](www.massa.net)

## Architecture

* massa_rust_sc: library for Rust smart contract dev for Massa
* massa_sc_runner: runner for unit tests in smart contract
* massa_rust_web3: library for interacting with smart contract (e.g. JsonRPC methods)
* hello_world: example of Rust smart contract
* hello_world_scripts: example of scripts to interact with hello_world smart contract

## hello_world smart contract

### Build

* cargo build -p hello_world --target=wasm32-unknown-unknown --release

### Post build

* ./build_post.sh

### Unit tests

* cargo test --target=wasm32-unknown-unknown -p hello_world --no-run
* cargo run -p massa_sc_runner -- target/wasm32-unknown-unknown/debug/deps/hello_world-XXXXX.wasm

## Deploy

* cargo run --bin hello_world_scripts

### Examples

* cargo run -p hello_world_scripts --example get_operations -- __OP_ID_STR__ get_op.log
