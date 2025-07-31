use single_validator_stake_pool::*;
use std::env;

fn main() {
    // Usage: decode <data_base58> [account1 ...]
    let mut args = env::args().skip(1);
    let b58 = args.next().expect("Usage: decode <data_base58> [account1 ...]");
    let bytes = bs58::decode(&b58).into_vec().expect("invalid base58");

    let mut accounts: Vec<String> = args.collect();
    if accounts.is_empty() {
        // provide a sufficiently long dummy list so account look-ups do not panic
        accounts = vec!["".to_string(); 32];
    }

    match Instruction::decode(&accounts, &bytes) {
        Ok(ix) => println!("{:?}", ix),
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1);
        }
    }
} 