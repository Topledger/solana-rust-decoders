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
    pub struct DataV2LpToken {
        pub name: String,
        pub symbol: String,
        pub uri: String,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Fee {
        pub fee: FeeEnum,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FlashLoanFee {
        pub fee_ratio: Rational,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Pool {
        #[serde(with = "pubkey_serde")]
        pub fee_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub lp_mint: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub incoming_stake: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ProtocolFee {
        #[serde(with = "pubkey_serde")]
        pub destination: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        pub fee_ratio: Rational,
        pub referrer_fee_ratio: Rational,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StakeAccountRecord {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_at_creation: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Rational {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub num: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub denom: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidityLinearParams {
        pub max_liq_remaining: Rational,
        pub zero_liq_remaining: Rational,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum FeeEnum {
        Flat { ratio: Rational },
        LiquidityLinear { params: LiquidityLinearParams },
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitProtocolFeeAccounts {
        pub payer: String,
        pub protocolFeeAccount: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetProtocolFeeAccounts {
        pub authority: String,
        pub protocolFeeAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreatePoolAccounts {
        pub payer: String,
        pub feeAuthority: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub feeAccount: String,
        pub lpMint: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityAccounts {
        pub from: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub lpMint: String,
        pub mintLpTokensTo: String,
        pub flashAccount: String,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidityAccounts {
        pub burnLpTokensFromAuthority: String,
        pub to: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub lpMint: String,
        pub burnLpTokensFrom: String,
        pub flashAccount: String,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetFeeAccounts {
        pub feeAuthority: String,
        pub poolAccount: String,
        pub feeAccount: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetFeeAuthorityAccounts {
        pub feeAuthority: String,
        pub poolAccount: String,
        pub newFeeAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetLpTokenMetadataAccounts {
        pub payer: String,
        pub feeAuthority: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub lpMint: String,
        pub metadata: String,
        pub metadataProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeactivateStakeAccountAccounts {
        pub stakeAccount: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub clock: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReclaimStakeAccountAccounts {
        pub stakeAccount: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub stakeAccountRecordAccount: String,
        pub clock: String,
        pub stakeHistory: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnstakeAccounts {
        pub unstaker: String,
        pub stakeAccount: String,
        pub destination: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub feeAccount: String,
        pub stakeAccountRecordAccount: String,
        pub protocolFeeAccount: String,
        pub protocolFeeDestination: String,
        pub clock: String,
        pub stakeProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnstakeWsolAccounts {
        pub unstaker: String,
        pub stakeAccount: String,
        pub destination: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub feeAccount: String,
        pub stakeAccountRecordAccount: String,
        pub protocolFeeAccount: String,
        pub protocolFeeDestination: String,
        pub clock: String,
        pub stakeProgram: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetFlashLoanFeeAccounts {
        pub payer: String,
        pub feeAuthority: String,
        pub poolAccount: String,
        pub flashLoanFeeAccount: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TakeFlashLoanAccounts {
        pub receiver: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub flashAccount: String,
        pub systemProgram: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RepayFlashLoanAccounts {
        pub repayer: String,
        pub poolAccount: String,
        pub poolSolReserves: String,
        pub flashAccount: String,
        pub flashLoanFeeAccount: String,
        pub protocolFeeAccount: String,
        pub protocolFeeDestination: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitProtocolFeeArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetProtocolFeeArgs {
        pub protocol_fee: ProtocolFee,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePoolArgs {
        pub fee: Fee,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidityArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_lp: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetFeeArgs {
        pub fee: Fee,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetFeeAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetLpTokenMetadataArgs {
        pub data: DataV2LpToken,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeactivateStakeAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReclaimStakeAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnstakeArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnstakeWsolArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetFlashLoanFeeArgs {
        pub flash_loan_fee: FlashLoanFee,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TakeFlashLoanArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RepayFlashLoanArgs {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitProtocolFee {
        accounts: InitProtocolFeeAccounts,
        args: InitProtocolFeeArgs,
    },
    SetProtocolFee {
        accounts: SetProtocolFeeAccounts,
        args: SetProtocolFeeArgs,
    },
    CreatePool {
        accounts: CreatePoolAccounts,
        args: CreatePoolArgs,
    },
    AddLiquidity {
        accounts: AddLiquidityAccounts,
        args: AddLiquidityArgs,
    },
    RemoveLiquidity {
        accounts: RemoveLiquidityAccounts,
        args: RemoveLiquidityArgs,
    },
    SetFee {
        accounts: SetFeeAccounts,
        args: SetFeeArgs,
    },
    SetFeeAuthority {
        accounts: SetFeeAuthorityAccounts,
        args: SetFeeAuthorityArgs,
    },
    SetLpTokenMetadata {
        accounts: SetLpTokenMetadataAccounts,
        args: SetLpTokenMetadataArgs,
    },
    DeactivateStakeAccount {
        accounts: DeactivateStakeAccountAccounts,
        args: DeactivateStakeAccountArgs,
    },
    ReclaimStakeAccount {
        accounts: ReclaimStakeAccountAccounts,
        args: ReclaimStakeAccountArgs,
    },
    Unstake {
        accounts: UnstakeAccounts,
        args: UnstakeArgs,
    },
    UnstakeWsol {
        accounts: UnstakeWsolAccounts,
        args: UnstakeWsolArgs,
    },
    SetFlashLoanFee {
        accounts: SetFlashLoanFeeAccounts,
        args: SetFlashLoanFeeArgs,
    },
    TakeFlashLoan {
        accounts: TakeFlashLoanAccounts,
        args: TakeFlashLoanArgs,
    },
    RepayFlashLoan {
        accounts: RepayFlashLoanAccounts,
        args: RepayFlashLoanArgs,
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
            [225u8, 155u8, 167u8, 170u8, 29u8, 145u8, 165u8, 90u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitProtocolFeeArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let payer = keys.next().unwrap().clone();
                let protocolFeeAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitProtocolFeeAccounts {
                    payer,
                    protocolFeeAccount,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitProtocolFee { accounts, args });
            }
            [173u8, 239u8, 83u8, 242u8, 136u8, 43u8, 144u8, 217u8] => {
                let mut rdr: &[u8] = rest;
                let protocol_fee: ProtocolFee = ProtocolFee::deserialize(&mut rdr)?;
                let args = SetProtocolFeeArgs { protocol_fee };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let authority = keys.next().unwrap().clone();
                let protocolFeeAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetProtocolFeeAccounts {
                    authority,
                    protocolFeeAccount,
                    remaining,
                };
                return Ok(Instruction::SetProtocolFee { accounts, args });
            }
            [233u8, 146u8, 209u8, 142u8, 207u8, 104u8, 64u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let fee: Fee = Fee::deserialize(&mut rdr)?;
                let args = CreatePoolArgs { fee };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 9usize;
                let payer = keys.next().unwrap().clone();
                let feeAuthority = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreatePoolAccounts {
                    payer,
                    feeAuthority,
                    poolAccount,
                    poolSolReserves,
                    feeAccount,
                    lpMint,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreatePool { accounts, args });
            }
            [181u8, 157u8, 89u8, 67u8, 143u8, 182u8, 52u8, 72u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = u64::deserialize(&mut rdr)?;
                let args = AddLiquidityArgs { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let from = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let mintLpTokensTo = keys.next().unwrap().clone();
                let flashAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityAccounts {
                    from,
                    poolAccount,
                    poolSolReserves,
                    lpMint,
                    mintLpTokensTo,
                    flashAccount,
                    tokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::AddLiquidity { accounts, args });
            }
            [80u8, 85u8, 209u8, 72u8, 24u8, 206u8, 177u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let amount_lp: u64 = u64::deserialize(&mut rdr)?;
                let args = RemoveLiquidityArgs { amount_lp };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let burnLpTokensFromAuthority = keys.next().unwrap().clone();
                let to = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let burnLpTokensFrom = keys.next().unwrap().clone();
                let flashAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidityAccounts {
                    burnLpTokensFromAuthority,
                    to,
                    poolAccount,
                    poolSolReserves,
                    lpMint,
                    burnLpTokensFrom,
                    flashAccount,
                    tokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidity { accounts, args });
            }
            [18u8, 154u8, 24u8, 18u8, 237u8, 214u8, 19u8, 80u8] => {
                let mut rdr: &[u8] = rest;
                let fee: Fee = Fee::deserialize(&mut rdr)?;
                let args = SetFeeArgs { fee };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let feeAuthority = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetFeeAccounts {
                    feeAuthority,
                    poolAccount,
                    feeAccount,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::SetFee { accounts, args });
            }
            [31u8, 1u8, 50u8, 87u8, 237u8, 101u8, 97u8, 132u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetFeeAuthorityArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let feeAuthority = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let newFeeAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetFeeAuthorityAccounts {
                    feeAuthority,
                    poolAccount,
                    newFeeAuthority,
                    remaining,
                };
                return Ok(Instruction::SetFeeAuthority { accounts, args });
            }
            [71u8, 73u8, 56u8, 155u8, 202u8, 142u8, 100u8, 150u8] => {
                let mut rdr: &[u8] = rest;
                let data: DataV2LpToken = DataV2LpToken::deserialize(&mut rdr)?;
                let args = SetLpTokenMetadataArgs { data };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 9usize;
                let payer = keys.next().unwrap().clone();
                let feeAuthority = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetLpTokenMetadataAccounts {
                    payer,
                    feeAuthority,
                    poolAccount,
                    poolSolReserves,
                    lpMint,
                    metadata,
                    metadataProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::SetLpTokenMetadata { accounts, args });
            }
            [217u8, 64u8, 76u8, 16u8, 216u8, 77u8, 123u8, 226u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeactivateStakeAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let stakeAccount = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeactivateStakeAccountAccounts {
                    stakeAccount,
                    poolAccount,
                    poolSolReserves,
                    clock,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::DeactivateStakeAccount { accounts, args });
            }
            [47u8, 127u8, 90u8, 221u8, 10u8, 160u8, 183u8, 117u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReclaimStakeAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let stakeAccount = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let stakeAccountRecordAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReclaimStakeAccountAccounts {
                    stakeAccount,
                    poolAccount,
                    poolSolReserves,
                    stakeAccountRecordAccount,
                    clock,
                    stakeHistory,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::ReclaimStakeAccount { accounts, args });
            }
            [90u8, 95u8, 107u8, 42u8, 205u8, 124u8, 50u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnstakeArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 11usize;
                let unstaker = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let stakeAccountRecordAccount = keys.next().unwrap().clone();
                let protocolFeeAccount = keys.next().unwrap().clone();
                let protocolFeeDestination = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let systemProgram = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = UnstakeAccounts {
                    unstaker,
                    stakeAccount,
                    destination,
                    poolAccount,
                    poolSolReserves,
                    feeAccount,
                    stakeAccountRecordAccount,
                    protocolFeeAccount,
                    protocolFeeDestination,
                    clock,
                    stakeProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::Unstake { accounts, args });
            }
            [125u8, 93u8, 190u8, 135u8, 89u8, 174u8, 142u8, 149u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnstakeWsolArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 13usize;
                let unstaker = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let stakeAccountRecordAccount = keys.next().unwrap().clone();
                let protocolFeeAccount = keys.next().unwrap().clone();
                let protocolFeeDestination = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnstakeWsolAccounts {
                    unstaker,
                    stakeAccount,
                    destination,
                    poolAccount,
                    poolSolReserves,
                    feeAccount,
                    stakeAccountRecordAccount,
                    protocolFeeAccount,
                    protocolFeeDestination,
                    clock,
                    stakeProgram,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::UnstakeWsol { accounts, args });
            }
            [21u8, 27u8, 137u8, 29u8, 226u8, 149u8, 221u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let flash_loan_fee: FlashLoanFee = FlashLoanFee::deserialize(&mut rdr)?;
                let args = SetFlashLoanFeeArgs { flash_loan_fee };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let payer = keys.next().unwrap().clone();
                let feeAuthority = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let flashLoanFeeAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetFlashLoanFeeAccounts {
                    payer,
                    feeAuthority,
                    poolAccount,
                    flashLoanFeeAccount,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::SetFlashLoanFee { accounts, args });
            }
            [64u8, 124u8, 6u8, 57u8, 151u8, 155u8, 26u8, 195u8] => {
                let mut rdr: &[u8] = rest;
                let lamports: u64 = u64::deserialize(&mut rdr)?;
                let args = TakeFlashLoanArgs { lamports };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let receiver = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let flashAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TakeFlashLoanAccounts {
                    receiver,
                    poolAccount,
                    poolSolReserves,
                    flashAccount,
                    systemProgram,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::TakeFlashLoan { accounts, args });
            }
            [119u8, 239u8, 18u8, 45u8, 194u8, 107u8, 31u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let args = RepayFlashLoanArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let repayer = keys.next().unwrap().clone();
                let poolAccount = keys.next().unwrap().clone();
                let poolSolReserves = keys.next().unwrap().clone();
                let flashAccount = keys.next().unwrap().clone();
                let flashLoanFeeAccount = keys.next().unwrap().clone();
                let protocolFeeAccount = keys.next().unwrap().clone();
                let protocolFeeDestination = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RepayFlashLoanAccounts {
                    repayer,
                    poolAccount,
                    poolSolReserves,
                    flashAccount,
                    flashLoanFeeAccount,
                    protocolFeeAccount,
                    protocolFeeDestination,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::RepayFlashLoan { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
