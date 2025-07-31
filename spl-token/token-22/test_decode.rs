Created test file
:Instruction;

fn main() {
    let data = bs58::decode("5m8jf3aGfwugzHAoTm6chdNJNK").into_vec().unwrap();
    let accounts = vec[200~pip install solana~
        "source_account".to_string(),
        "mint_account".to_string(), 
        "destination_account".to_string(),
        "authority_account".to_string()
    ];
    
    match Instruction::decode(&data, &accounts) {
        Ok(instruction) => {
            println!("Successfully decoded: {:?}", instruction);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
