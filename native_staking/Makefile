.PHONY wasm_pack:
wasm_pack:
<<<<<<< HEAD
	wasm-pack build --no-typescript --target no-modules
=======
	wasm-pack build --release --no-typescript --target no-modules
>>>>>>> 58c35ac961c98dfedb802a091c42873dcbb7b734

.PHONY wasm_pack_dev:
wasm_pack_dev:
	wasm-pack build --target web --dev

.PHONY convert_and_gcloud_upload:
convert_and_gcloud_upload:
	go build utils/convert.go && mv convert utils/convert
<<<<<<< HEAD
	utils/script.sh system_program_transfers
=======
	utils/script.sh solana native_staking
>>>>>>> 58c35ac961c98dfedb802a091c42873dcbb7b734
