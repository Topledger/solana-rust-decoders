#!/bin/bash
<<<<<<< HEAD
name=${1}

cd utils || exit
./convert < ../pkg/${name}_bg.wasm > wasm_bytes.js
gsutil cp wasm_bytes.js gs://tt-bq-js/solana/${name}/wasm_bytes.js
gsutil cp ../pkg/${name}.js gs://tt-bq-js/solana/${name}/
=======
name=${2}
folder=${1}

cd utils || exit
./convert < ../pkg/${name}_bg.wasm > wasm_bytes.js
gsutil cp wasm_bytes.js gs://tt-bq-js/solana/${folder}/${name}/wasm_bytes.js
gsutil cp ../pkg/${name}.js gs://tt-bq-js/solana/${folder}/${name}/
>>>>>>> 58c35ac961c98dfedb802a091c42873dcbb7b734
