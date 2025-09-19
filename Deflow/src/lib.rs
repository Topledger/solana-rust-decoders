use borsh::BorshDeserialize;
use bs58::decode;
use serde::Serialize;

mod pubkey_serializer;

// Generated at build-time by build.rs
include!("idl.rs");

use console_error_panic_hook;
use serde_wasm_bindgen::{from_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// NEW
use serde_json::Value as JsonValue;
use serde_wasm_bindgen::Serializer as WasmSerializer;

/// Simple error struct so `parse()` always returns JSON the UDF can handle
#[derive(Serialize)]
struct ErrorObj {
    error: String,
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
}

/// Walk a serde_json::Value and stringify any integers outside JS's safe range.
/// JS safe integers are in [-2^53 + 1, 2^53 - 1].
fn stringify_unsafe_numbers(v: JsonValue) -> JsonValue {
    match v {
        JsonValue::Number(n) => {
            const MAX_SAFE: u64 = 9_007_199_254_740_991;
            const MIN_SAFE: i64 = -9_007_199_254_740_991;

            if let Some(u) = n.as_u64() {
                if u > MAX_SAFE {
                    return JsonValue::String(u.to_string());
                }
                return JsonValue::Number(n);
            }
            if let Some(i) = n.as_i64() {
                if i > MAX_SAFE as i64 || i < MIN_SAFE {
                    return JsonValue::String(i.to_string());
                }
                return JsonValue::Number(n);
            }
            JsonValue::Number(n)
        }
        JsonValue::Array(a) => JsonValue::Array(a.into_iter().map(stringify_unsafe_numbers).collect()),
        JsonValue::Object(m) => {
            JsonValue::Object(
                m.into_iter()
                    .map(|(k, v)| (k, stringify_unsafe_numbers(v)))
                    .collect(),
            )
        }
        other => other,
    }
}

/// Convert any `Serialize` to a JsValue **as a plain JS object** (not Map),
/// after stringifying unsafe integers.
fn to_js_json<T: Serialize>(val: &T) -> JsValue {
    match serde_json::to_value(val) {
        Ok(json) => {
            let fixed = stringify_unsafe_numbers(json);
            // Force maps → plain JS objects so the UDF gets real JSON
            let ser = WasmSerializer::new().serialize_maps_as_objects(true);
            fixed.serialize(&ser).unwrap_or_else(|e| {
                // last-resort: return an error object
                let err = ErrorObj { error: format!("serialize failed: {}", e) };
                let ser2 = WasmSerializer::new().serialize_maps_as_objects(true);
                err.serialize(&ser2).unwrap()
            })
        }
        Err(e) => {
            let err = ErrorObj { error: format!("serde_json::to_value failed: {}", e) };
            let ser = WasmSerializer::new().serialize_maps_as_objects(true);
            err.serialize(&ser).unwrap()
        }
    }
}

/// Parse a base58-encoded Solana instruction *or* an Anchor event log payload.
/// - `base58_str`: the base58 data string (ix data OR event log byte payload)
/// - `accounts_js`: the array of account pubkeys (strings) used by the instruction
///
/// Returns a JSON-able plain object that your BigQuery JS UDF can return as JSON.
#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: JsValue) -> JsValue {
    // 1) Decode base58 → Vec<u8>
    let decoded = match decode(base58_str).into_vec() {
        Ok(b) => b,
        Err(e) => {
            return to_js_json(&ErrorObj {
                error: format!("base58 decode failed: {}", e),
            });
        }
    };

    // 2) Is it an Anchor-logged event?
    if decoded.len() >= 8 && &decoded[..8] == &events::EVENT_LOG_DISCRIMINATOR {
        match events::Event::decode(&decoded) {
            Ok(ev) => {
                // Return a JSON object (maps-as-objects)
                return to_js_json(&ev);
            }
            Err(e) => {
                return to_js_json(&ErrorObj {
                    error: format!("Event decode failed: {}", e),
                });
            }
        }
    }

    // 3) Otherwise, treat it as an instruction
    let accounts: Vec<String> = match from_value(accounts_js) {
        Ok(v) => v,
        Err(e) => {
            return to_js_json(&ErrorObj {
                error: format!("accounts deserialize failed: {}", e),
            });
        }
    };

    match Instruction::decode(&accounts, &decoded) {
        Ok(ix) => {
            // Return a JSON object (maps-as-objects) with big ints as strings
            to_js_json(&ix)
        }
        Err(e) => {
            to_js_json(&ErrorObj {
                error: format!("Instruction::decode failed: {}", e),
            })
        }
    }
}
