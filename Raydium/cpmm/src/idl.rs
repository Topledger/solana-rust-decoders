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
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Observation {
        pub block_timestamp: u64,
        pub cumulative_token0_price_x32: u128,
        pub cumulative_token1_price_x32: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TradeDirection {
        ZeroForOne,
        OneForZero,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RoundDirection {
        Floor,
        Ceiling,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PoolStatusBitIndex {
        Deposit,
        Withdraw,
        Swap,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PoolStatusBitFlag {
        Enable,
        Disable,
    }
}
pub mod accounts_data {
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct CreateAmmConfigAccounts {
        pub owner: String,
        pub ammConfig: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAmmConfigAccounts {
        pub owner: String,
        pub ammConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: String,
        pub poolState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: String,
        pub authority: String,
        pub poolState: String,
        pub ammConfig: String,
        pub token0Vault: String,
        pub token1Vault: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub recipientToken0Account: String,
        pub recipientToken1Account: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectFundFeeAccounts {
        pub owner: String,
        pub authority: String,
        pub poolState: String,
        pub ammConfig: String,
        pub token0Vault: String,
        pub token1Vault: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub recipientToken0Account: String,
        pub recipientToken1Account: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub creator: String,
        pub ammConfig: String,
        pub authority: String,
        pub poolState: String,
        pub token0Mint: String,
        pub token1Mint: String,
        pub lpMint: String,
        pub creatorToken0: String,
        pub creatorToken1: String,
        pub creatorLpToken: String,
        pub token0Vault: String,
        pub token1Vault: String,
        pub createPoolFee: String,
        pub observationState: String,
        pub tokenProgram: String,
        pub token0Program: String,
        pub token1Program: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub owner: String,
        pub authority: String,
        pub poolState: String,
        pub ownerLpToken: String,
        pub token0Account: String,
        pub token1Account: String,
        pub token0Vault: String,
        pub token1Vault: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub lpMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub owner: String,
        pub authority: String,
        pub poolState: String,
        pub ownerLpToken: String,
        pub token0Account: String,
        pub token1Account: String,
        pub token0Vault: String,
        pub token1Vault: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub lpMint: String,
        pub memoProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapBaseInputAccounts {
        pub payer: String,
        pub authority: String,
        pub ammConfig: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        pub inputTokenMint: String,
        pub outputTokenMint: String,
        pub observationState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapBaseOutputAccounts {
        pub payer: String,
        pub authority: String,
        pub ammConfig: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        pub inputTokenMint: String,
        pub outputTokenMint: String,
        pub observationState: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateAmmConfigArguments {
        pub index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trade_fee_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub protocol_fee_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fund_fee_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub create_pool_fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateAmmConfigArguments {
        pub param: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub value: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePoolStatusArguments {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectProtocolFeeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_requested: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectFundFeeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_requested: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub init_amount0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub init_amount1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub open_time: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lp_token_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub maximum_token0_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub maximum_token1_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lp_token_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_token0_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_token1_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapBaseInputArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapBaseOutputArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_out: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    CreateAmmConfig {
        accounts: CreateAmmConfigAccounts,
        args: CreateAmmConfigArguments,
    },
    UpdateAmmConfig {
        accounts: UpdateAmmConfigAccounts,
        args: UpdateAmmConfigArguments,
    },
    UpdatePoolStatus {
        accounts: UpdatePoolStatusAccounts,
        args: UpdatePoolStatusArguments,
    },
    CollectProtocolFee {
        accounts: CollectProtocolFeeAccounts,
        args: CollectProtocolFeeArguments,
    },
    CollectFundFee {
        accounts: CollectFundFeeAccounts,
        args: CollectFundFeeArguments,
    },
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArguments,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArguments,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArguments,
    },
    SwapBaseInput {
        accounts: SwapBaseInputAccounts,
        args: SwapBaseInputArguments,
    },
    SwapBaseOutput {
        accounts: SwapBaseOutputAccounts,
        args: SwapBaseOutputArguments,
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
            [137u8, 52u8, 237u8, 212u8, 215u8, 117u8, 108u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateAmmConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateAmmConfigAccounts {
                    owner,
                    ammConfig,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateAmmConfig { accounts, args });
            }
            [49u8, 60u8, 174u8, 136u8, 154u8, 28u8, 116u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateAmmConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAmmConfigAccounts {
                    owner,
                    ammConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateAmmConfig { accounts, args });
            }
            [130u8, 87u8, 108u8, 6u8, 46u8, 224u8, 117u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePoolStatusArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePoolStatusAccounts {
                    authority,
                    poolState,
                    remaining,
                };
                return Ok(Instruction::UpdatePoolStatus { accounts, args });
            }
            [136u8, 136u8, 252u8, 221u8, 194u8, 66u8, 126u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectProtocolFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let token0Vault = keys.next().unwrap().clone();
                let token1Vault = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let recipientToken0Account = keys.next().unwrap().clone();
                let recipientToken1Account = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectProtocolFeeAccounts {
                    owner,
                    authority,
                    poolState,
                    ammConfig,
                    token0Vault,
                    token1Vault,
                    vault0Mint,
                    vault1Mint,
                    recipientToken0Account,
                    recipientToken1Account,
                    tokenProgram,
                    tokenProgram2022,
                    remaining,
                };
                return Ok(Instruction::CollectProtocolFee { accounts, args });
            }
            [167u8, 138u8, 78u8, 149u8, 223u8, 194u8, 6u8, 126u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectFundFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let token0Vault = keys.next().unwrap().clone();
                let token1Vault = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let recipientToken0Account = keys.next().unwrap().clone();
                let recipientToken1Account = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectFundFeeAccounts {
                    owner,
                    authority,
                    poolState,
                    ammConfig,
                    token0Vault,
                    token1Vault,
                    vault0Mint,
                    vault1Mint,
                    recipientToken0Account,
                    recipientToken1Account,
                    tokenProgram,
                    tokenProgram2022,
                    remaining,
                };
                return Ok(Instruction::CollectFundFee { accounts, args });
            }
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let creator = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let token0Mint = keys.next().unwrap().clone();
                let token1Mint = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let creatorToken0 = keys.next().unwrap().clone();
                let creatorToken1 = keys.next().unwrap().clone();
                let creatorLpToken = keys.next().unwrap().clone();
                let token0Vault = keys.next().unwrap().clone();
                let token1Vault = keys.next().unwrap().clone();
                let createPoolFee = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let token0Program = keys.next().unwrap().clone();
                let token1Program = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    creator,
                    ammConfig,
                    authority,
                    poolState,
                    token0Mint,
                    token1Mint,
                    lpMint,
                    creatorToken0,
                    creatorToken1,
                    creatorLpToken,
                    token0Vault,
                    token1Vault,
                    createPoolFee,
                    observationState,
                    tokenProgram,
                    token0Program,
                    token1Program,
                    associatedTokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let ownerLpToken = keys.next().unwrap().clone();
                let token0Account = keys.next().unwrap().clone();
                let token1Account = keys.next().unwrap().clone();
                let token0Vault = keys.next().unwrap().clone();
                let token1Vault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    owner,
                    authority,
                    poolState,
                    ownerLpToken,
                    token0Account,
                    token1Account,
                    token0Vault,
                    token1Vault,
                    tokenProgram,
                    tokenProgram2022,
                    vault0Mint,
                    vault1Mint,
                    lpMint,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let ownerLpToken = keys.next().unwrap().clone();
                let token0Account = keys.next().unwrap().clone();
                let token1Account = keys.next().unwrap().clone();
                let token0Vault = keys.next().unwrap().clone();
                let token1Vault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    owner,
                    authority,
                    poolState,
                    ownerLpToken,
                    token0Account,
                    token1Account,
                    token0Vault,
                    token1Vault,
                    tokenProgram,
                    tokenProgram2022,
                    vault0Mint,
                    vault1Mint,
                    lpMint,
                    memoProgram,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            [143u8, 190u8, 90u8, 218u8, 196u8, 30u8, 51u8, 222u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapBaseInputArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let inputTokenMint = keys.next().unwrap().clone();
                let outputTokenMint = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapBaseInputAccounts {
                    payer,
                    authority,
                    ammConfig,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    inputTokenProgram,
                    outputTokenProgram,
                    inputTokenMint,
                    outputTokenMint,
                    observationState,
                    remaining,
                };
                return Ok(Instruction::SwapBaseInput { accounts, args });
            }
            [55u8, 217u8, 98u8, 86u8, 163u8, 74u8, 180u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapBaseOutputArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let inputTokenMint = keys.next().unwrap().clone();
                let outputTokenMint = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapBaseOutputAccounts {
                    payer,
                    authority,
                    ammConfig,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    inputTokenProgram,
                    outputTokenProgram,
                    inputTokenMint,
                    outputTokenMint,
                    observationState,
                    remaining,
                };
                return Ok(Instruction::SwapBaseOutput { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LpChangeEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_id: [u8; 32usize],
        pub lp_amount_before: u64,
        pub token0_vault_before: u64,
        pub token1_vault_before: u64,
        pub token0_amount: u64,
        pub token1_amount: u64,
        pub token0_transfer_fee: u64,
        pub token1_transfer_fee: u64,
        pub change_type: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_id: [u8; 32usize],
        pub input_vault_before: u64,
        pub output_vault_before: u64,
        pub input_amount: u64,
        pub output_amount: u64,
        pub input_transfer_fee: u64,
        pub output_transfer_fee: u64,
        pub base_input: bool,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        LpChangeEvent { args: LpChangeEvent },
        SwapEvent { args: SwapEvent },
    }
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        #[doc = r" Decode a raw Anchor‐logged event:"]
        #[doc = r"  [ EVENT_LOG_DISCRIMINATOR (8) ]"]
        #[doc = r"  [ REAL_EVENT_DISCRIMINATOR (8) ]"]
        #[doc = r"  [ Borsh payload …           ]"]
        pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
            if data.len() < 16 {
                anyhow::bail!("Event log too short: {}", data.len());
            }
            let (wrapper, rest) = data.split_at(1);
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!(
                    "Missing event log prefix: expected {:x?}, got {:x?}",
                    EVENT_LOG_DISCRIMINATOR,
                    wrapper
                );
            }
            let (disc_slice, payload) = rest.split_at(8);
            let disc: [u8; 8] = disc_slice.try_into().unwrap();
            match disc {
                [121u8, 163u8, 205u8, 201u8, 57u8, 218u8, 117u8, 60u8] => {
                    let mut rdr = &payload[..];
                    let args = LpChangeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LpChangeEvent { args });
                }
                [64u8, 198u8, 205u8, 232u8, 38u8, 8u8, 113u8, 226u8] => {
                    let mut rdr = &payload[..];
                    let args = SwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
