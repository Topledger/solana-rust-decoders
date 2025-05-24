#!/bin/bash
name=${2}
folder=${1}

cd utils || exit
./convert < ../pkg/${name}_bg.wasm > wasm_bytes.js
gsutil cp wasm_bytes.js gs://tt-bq-js/solana/${folder}/${name}/wasm_bytes.js
gsutil cp ../pkg/${name}.js gs://tt-bq-js/solana/${folder}/${name}/