// build.rs

use quote::quote;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use quote::format_ident;

fn main() {
    // Simple stub - no code generation for now
    println!("cargo:rerun-if-changed=idls/gmx_solana.json");
} 