#!/bin/bash
set -e

WASM_FILENAME=hello_world
TARGET=target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wasm
WAT=target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wat
# rm -fv ${TARGET}
cp -v target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wasm target/wasm32-unknown-unknown/release/${WASM_FILENAME}.backup.wasm
echo "Generating wat file: ${WAT}"
wasm2wat target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wasm > ${WAT}
echo "Generating wasm file: ${TARGET}"
wat2wasm ${WAT} -o ${TARGET}
du -hcs ${TARGET}

