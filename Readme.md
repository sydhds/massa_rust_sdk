# Massa rust sdk

A rust sdk to develop smart contract for [Massa blockchain](www.massa.net)

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
