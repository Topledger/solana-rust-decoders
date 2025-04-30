pub mod pb;

// Import instruction processing module
pub mod instructions;

pub mod prepare_input_accounts;
use anyhow::Context;
use bs58::decode;
use pb::sf::solana::orca_whirlpool::v1::JsonMeta;


fn main() -> anyhow::Result<()> {
    let data_base58 = "Hcrp3dcPCJ";
    let data = decode(data_base58).into_vec()?;
    let accounts = [
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
        "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB".to_string(),
    ];

    // process
    match instructions::process_instruction(&data, &accounts)
        .context("process_instruction failed") 
    {
        Ok(meta) => {
            // Convert to your 3-key JSON wrapper
            let jm: JsonMeta = meta.into();
            let s = serde_json::to_string_pretty(&jm)
                .context("failed to serialize JsonMeta")?;
            println!("{}", s);
        }
        Err(e) => {
            // Print the full error chain and backtrace
            eprintln!("ERROR: {:#}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}