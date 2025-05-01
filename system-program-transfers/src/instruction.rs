
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use borsh::BorshDeserialize;

// our two discriminators:
const TRANSFER_DISCRIMINATOR:         u32 = 2;
const TRANSFER_WITH_SEED_DISCRIMINATOR:u32 = 11;

// —————— the pieces we’ll serialize —————— //

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferAccounts {
    pub funding_account:  String,
    pub recipient_account:String,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct TransferArgs {
    pub lamports: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferWithSeedAccounts {
    pub funding_account:  String,
    pub base_account:     String,
    pub recipient_account:String,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct TransferWithSeedArgs {
    pub lamports:   u64,
    pub from_seed:  String,
    pub from_owner: String,
}

// tag the enum so Serde emits a `"instruction_type"` field for us
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Transfer {
        args:     TransferArgs,
        accounts: TransferAccounts,
    },
    TransferWithSeed {
        args:     TransferWithSeedArgs,
        accounts: TransferWithSeedAccounts,
    },
}

impl Instruction {
    pub fn decode(
        account_keys: &[String],
        data: &[u8],
    ) -> Result<Self> {
        if data.len() < 4 {
            return Err(anyhow!("instruction data too short"));
        }
        // pull off the 4-byte little-endian discriminator
        let (disc_bytes, rest) = data.split_at(4);
        let disc = u32::from_le_bytes(disc_bytes.try_into().unwrap());

        match disc {
            TRANSFER_DISCRIMINATOR => {
                // deserialize just the lamports
                let args = TransferArgs::try_from_slice(rest).map_err(|e| anyhow!("failed to parse TransferArgs: {}", e))?;

                // map the first two account keys straight into strings
                let accounts = TransferAccounts {
                    funding_account:   account_keys[0].to_string(),
                    recipient_account: account_keys[1].to_string(),
                };
               
                Ok(Instruction::Transfer {
                    args,
                    accounts
                })
            }

            TRANSFER_WITH_SEED_DISCRIMINATOR => {
                // parse lamports + seed + owner‐pubkey manually
                let lamports = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                let split = rest.len() - 32;
                let from_seed  = String::from_utf8_lossy(&rest[8..split]).into_owned();
                let from_owner = String::from_utf8_lossy(&rest[split..]).into_owned();

                // now consume exactly three account‐keys
                let funding_account   = account_keys[0].to_string();
                let base_account      = account_keys[1].to_string();
                let recipient_account = account_keys[2].to_string();

                Ok(Instruction::TransferWithSeed {
                    args: TransferWithSeedArgs { lamports, from_seed, from_owner },
                    accounts: TransferWithSeedAccounts { funding_account, base_account, recipient_account },
                })
            }

            _ => Err(anyhow!("unknown discriminator: {}", disc)),
        }
    }
}


