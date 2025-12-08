.PHONY wasm_pack:
wasm_pack:
	wasm-pack build --release --no-typescript --target no-modules --features wasm

.PHONY wasm_pack_dev:
wasm_pack_dev:
	wasm-pack build --target web --dev --features wasm

.PHONY convert_and_gcloud_upload:
convert_and_gcloud_upload:
	go build utils/convert.go && mv convert utils/convert
	utils/script.sh solana_labs inflation