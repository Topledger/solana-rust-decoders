use bs58::decode;
use serde::Serializer;
use anchor_lang::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod pubkey_serializer;

// helper for u64/u128 → string in JSON (if needed later)
pub fn serialize_to_string<S, T>(x: &T, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string())
}

// generated code
include!("idl.rs");

#[wasm_bindgen(start)]
pub fn run() { console_error_panic_hook::set_once(); }

#[wasm_bindgen]
pub fn parse(base58_str: &str) -> JsValue {
    let decoded = match decode(base58_str).into_vec() {
        Ok(b) => b,
        Err(e) => return JsValue::from_str(&format!("base58 decode failed: {}", e)),
    };

    // try instruction decode
    let dummy_accounts: Vec<String> = vec!["".to_string(); 32];
    match Instruction::decode(&dummy_accounts, &decoded) {
        Ok(ix) => JsValue::from_str(&format!("{:?}", ix)),
        Err(e) => JsValue::from_str(&format!("decode failed: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpmm_decode() {
        let sample = "66JafaVu7KMxSjJTzEMLxRQNKjo67bQR5";
        let bytes = decode(sample).into_vec().unwrap_or_default();
        let dummy: Vec<String> = vec!["".into(); 32];
        match Instruction::decode(&dummy, &bytes) {
            Ok(ix) => println!("Decoded instruction: {:?}", ix),
            Err(e) => println!("Decode error: {}", e),
        }
    }
} 