#!/bin/bash

set -ev

cd ../contract && cargo build --target wasm32-unknown-unknown --release && cd -

mkdir -p ./out && cp ../../target/wasm32-unknown-unknown/release/api3_contract.wasm ./out/main.wasm
