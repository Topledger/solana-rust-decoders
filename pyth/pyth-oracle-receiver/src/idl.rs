use crate::pubkey_serializer::pubkey_serde;
use crate::pubkey_serializer::pubkey_serde_option;
use serde::Serializer;
#[allow(dead_code)]
use std::convert::TryInto;
fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string())
}
pub use accounts_data::*;
pub use ix_data::*;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PostUpdateParams {
        pub merkle_price_update: MerklePriceUpdate,
        pub treasury_id: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MerklePriceUpdate {
        pub message: Vec<u8>,
        pub proof: Vec<[u8; 20usize]>,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct UpdatePriceFeedAccounts {
        pub payer: String,
        pub pythSolanaReceiver: String,
        pub encodedVaa: String,
        pub config: String,
        pub treasury: String,
        pub priceFeedAccount: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePriceFeedArguments {
        pub params: PostUpdateParams,
        pub shard_id: u16,
        pub feed_id: [u8; 32usize],
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    UpdatePriceFeed {
        accounts: UpdatePriceFeedAccounts,
        args: UpdatePriceFeedArguments,
    },
}
impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
        if data.len() < 8 {
            anyhow::bail!("Data too short: {}", data.len());
        }
        let (disc_slice, rest) = data.split_at(8);
        let disc: [u8; 8] = disc_slice.try_into().unwrap();
        match disc {
            [28u8, 9u8, 93u8, 150u8, 86u8, 153u8, 188u8, 115u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePriceFeedArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let pythSolanaReceiver = keys.next().unwrap().clone();
                let encodedVaa = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let treasury = keys.next().unwrap().clone();
                let priceFeedAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePriceFeedAccounts {
                    payer,
                    pythSolanaReceiver,
                    encodedVaa,
                    config,
                    treasury,
                    priceFeedAccount,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::UpdatePriceFeed { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;
}
