#!/bin/bash
set -e

WASM_FILENAME=hello_world
TARGET=target/wasm32-unknown-unknown/release/main.wasm
WAT=target/wasm32-unknown-unknown/release/main.wat
rm -fv ${TARGET}
cp -v target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wasm target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wasm.backup
echo "Generating wat file: ${WAT}"
wasm2wat target/wasm32-unknown-unknown/release/${WASM_FILENAME}.wasm > ${WAT}
echo "Generating wasm file: ${TARGET}"
wat2wasm ${WAT} -o ${TARGET}
du -hcs ${TARGET}

