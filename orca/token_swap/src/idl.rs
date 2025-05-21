use borsh::{BorshDeserialize, BorshSerialize};
use anyhow::{Error, anyhow};
use serde::Serialize;
use serde::Serializer;

fn u64_to_string<S>(x: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.to_string())
}
// -----------------------------------------------------------------------------
// Inlined SPL token-swap layouts
// -----------------------------------------------------------------------------

/// All swap fees (64 bytes)
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq, Serialize)]
pub struct Fees {
    pub trade_fee_numerator:            u64,
    pub trade_fee_denominator:          u64,
    pub owner_trade_fee_numerator:      u64,
    pub owner_trade_fee_denominator:    u64,
    pub owner_withdraw_fee_numerator:   u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator:             u64,
    pub host_fee_denominator:           u64,
}

/// Curve type tag (1 byte)
#[repr(u8)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq, Serialize)]
pub enum CurveType {
    ConstantProduct = 0,
    ConstantPrice   = 1,
    Offset          = 2,
}

/// SwapCurve wrapper: 1-byte tag + 32-byte calculator data
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
pub struct SwapCurve {
    pub curve_type: CurveType,
    pub calculator: [u8; 32],
}

// -----------------------------------------------------------------------------
// Argument structs (typedefs)
// -----------------------------------------------------------------------------

pub mod typedefs {
    use super::*;

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitializeArgs {
        pub fees: Fees,
        pub swap_curve: SwapCurve,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SwapArgs {
        #[serde(serialize_with = "u64_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "u64_to_string")]
        pub minimum_amount_out: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemoveLiquidityTypesArgs {
        pub pool_token_amount: u64,
        pub maximum_token_a_amount: u64,
        pub maximum_token_b_amount: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DepositArgs {
        pub pool_token_amount: u64,
        pub minimum_token_a_amount: u64,
        pub minimum_token_b_amount: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DepositSingleTokenTypeExactAmountInArgs {
        pub source_token_amount: u64,
        pub minimum_pool_token_amount: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawSingleTokenTypeExactAmountOutArgs {
        pub destination_token_amount: u64,
        pub maximum_pool_token_amount: u64,
    }
}

// -----------------------------------------------------------------------------
// Accounts for each instruction (now String-based)
// -----------------------------------------------------------------------------

pub mod accounts_data {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub token_swap_account: String,
        pub swap_authority: String,
        pub token_a: String,
        pub token_b: String,
        pub pool_token_mint: String,
        pub pool_token_account: String,
        pub intial_pool_token_account: String,
        pub token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub token_swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub user_source: String,
        pub pool_source: String,
        pub pool_destination: String,
        pub user_destination: String,
        pub pool_mint: String,
        pub fee_account: String,
        pub token_program: String
    }

    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidityTypesAccounts {
        pub token_swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub pool_mint: String,
        pub source_pool_account: String,
        pub from_a: String,
        pub from_b: String,
        pub user_account_a: String,
        pub user_account_b: String,
        pub fee_account: String,
        pub token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub token_swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub source_a: String,
        pub source_b: String,
        pub into_a: String,
        pub into_b: String,
        pub pool_token: String,
        pub pool_account: String,
        pub token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct DepositSingleTokenTypeExactAmountInAccounts {
        pub token_swap_account: String,
        pub swap_authority: String,
        pub user_transfer_authority: String,
        pub token_a_source: String,
        pub token_a_swap: String,
        pub token_b_swap: String,
        pub pool_token_mint: String,
        pub pool_token_account: String,
        pub token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct WithdrawSingleTokenTypeExactAmountOutAccounts {
        pub token_swap: String,
        pub swap_authority: String,
        pub user_transfer_authority: String,
        pub pool_mint: String,
        pub source_pool_account: String,
        pub token_a_swap_account: String,
        pub token_b_swap_account: String,
        pub user_account_a_to_credit: String,
        pub fee_account: String,
        pub token_program: String,
    }
}

// -----------------------------------------------------------------------------
// Wrappers for Borsh deserialization of instruction data
// -----------------------------------------------------------------------------

pub mod ix_data {
    use super::typedefs::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;

    #[derive(BorshDeserialize, Debug, Serialize)] pub struct InitializeIx { pub args: InitializeArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct SwapIx     { pub args: SwapArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct RemoveLiquidityTypesIx { pub args: RemoveLiquidityTypesArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct DepositIx { pub args: DepositArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct DepositSingleTokenTypeExactAmountInIx { pub args: DepositSingleTokenTypeExactAmountInArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct WithdrawSingleTokenTypeExactAmountOutIx { pub args: WithdrawSingleTokenTypeExactAmountOutArgs }
}

// -----------------------------------------------------------------------------
// Main Instruction enum and decoder (accounts as Strings)
// -----------------------------------------------------------------------------

#[derive(Serialize, Debug)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize { accounts: accounts_data::InitializeAccounts, args: typedefs::InitializeArgs },
    Swap       { accounts: accounts_data::SwapAccounts,       args: typedefs::SwapArgs },
    Deposit { accounts: accounts_data::DepositAccounts, args: typedefs::DepositArgs },
    RemoveLiquidity { accounts: accounts_data::RemoveLiquidityTypesAccounts, args: typedefs::RemoveLiquidityTypesArgs },
    DepositSingleTokenTypeExactAmountIn { accounts: accounts_data::DepositSingleTokenTypeExactAmountInAccounts, args: typedefs::DepositSingleTokenTypeExactAmountInArgs },
    WithdrawSingleTokenTypeExactAmountOut { accounts: accounts_data::WithdrawSingleTokenTypeExactAmountOutAccounts, args: typedefs::WithdrawSingleTokenTypeExactAmountOutArgs },
}

impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> std::result::Result<Self, Error> {
        let (&tag, rest) = data.split_first().ok_or_else(|| anyhow!("Empty instruction data"))?;
        let mut keys = account_keys.iter();
        let mut slice_reader = rest;
        match tag {
            0 => {
                let args = typedefs::InitializeArgs::deserialize(&mut slice_reader)?;
                let accounts = accounts_data::InitializeAccounts {
                    token_swap_account: keys.next().unwrap().clone(),
                    swap_authority: keys.next().unwrap().clone(),
                    token_a: keys.next().unwrap().clone(),
                    token_b: keys.next().unwrap().clone(),
                    pool_token_mint: keys.next().unwrap().clone(),
                    pool_token_account: keys.next().unwrap().clone(),
                    intial_pool_token_account: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::Initialize { accounts, args })
            }
            1 => {
                let args = typedefs::SwapArgs::deserialize(&mut slice_reader)?;
                let accounts = accounts_data::SwapAccounts {
                    token_swap: keys.next().unwrap().clone(),
                    authority: keys.next().unwrap().clone(),
                    user_transfer_authority: keys.next().unwrap().clone(),
                    user_source: keys.next().unwrap().clone(),
                    pool_source: keys.next().unwrap().clone(),
                    pool_destination: keys.next().unwrap().clone(),
                    user_destination: keys.next().unwrap().clone(),
                    pool_mint: keys.next().unwrap().clone(),
                    fee_account: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone()
                   
                };
                Ok(Instruction::Swap { accounts, args })
            }
           
            2 => {
                let args = typedefs::DepositArgs::deserialize(&mut slice_reader)?;
                let accounts = accounts_data::DepositAccounts {
                    token_swap: keys.next().unwrap().clone(),
                    authority: keys.next().unwrap().clone(),
                    user_transfer_authority: keys.next().unwrap().clone(),
                    source_a: keys.next().unwrap().clone(),
                    source_b: keys.next().unwrap().clone(),
                    into_a: keys.next().unwrap().clone(),
                    into_b: keys.next().unwrap().clone(),
                    pool_token: keys.next().unwrap().clone(),
                    pool_account: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::Deposit { accounts, args })
            }
            3 => {
                let args = typedefs::RemoveLiquidityTypesArgs::deserialize(&mut slice_reader)?;
                let accounts = accounts_data::RemoveLiquidityTypesAccounts {

                    token_swap: keys.next().unwrap().clone(),
                    authority: keys.next().unwrap().clone(),
                    user_transfer_authority: keys.next().unwrap().clone(),
                    pool_mint: keys.next().unwrap().clone(),
                    source_pool_account: keys.next().unwrap().clone(),
                    from_a: keys.next().unwrap().clone(),
                    from_b: keys.next().unwrap().clone(),
                    user_account_a: keys.next().unwrap().clone(),
                    user_account_b: keys.next().unwrap().clone(),
                    fee_account: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::RemoveLiquidity { accounts, args })
            }
            4 => {
                let args = typedefs::DepositSingleTokenTypeExactAmountInArgs::deserialize(&mut slice_reader)?;
                let accounts = accounts_data::DepositSingleTokenTypeExactAmountInAccounts {
                    token_swap_account: keys.next().unwrap().clone(),
                    swap_authority: keys.next().unwrap().clone(),
                    user_transfer_authority: keys.next().unwrap().clone(),
                    token_a_source: keys.next().unwrap().clone(),
                    token_a_swap: keys.next().unwrap().clone(),
                    token_b_swap: keys.next().unwrap().clone(),
                    pool_token_mint: keys.next().unwrap().clone(),
                    pool_token_account: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::DepositSingleTokenTypeExactAmountIn { accounts, args })
            }
            5 => {
                let args = typedefs::WithdrawSingleTokenTypeExactAmountOutArgs::deserialize(&mut slice_reader)?;
                let accounts = accounts_data::WithdrawSingleTokenTypeExactAmountOutAccounts {
                    token_swap: keys.next().unwrap().clone(),
                    swap_authority: keys.next().unwrap().clone(),
                    user_transfer_authority: keys.next().unwrap().clone(),
                    pool_mint: keys.next().unwrap().clone(),
                    source_pool_account: keys.next().unwrap().clone(),
                    token_a_swap_account: keys.next().unwrap().clone(),
                    token_b_swap_account: keys.next().unwrap().clone(),
                    user_account_a_to_credit: keys.next().unwrap().clone(),
                    fee_account: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::WithdrawSingleTokenTypeExactAmountOut { accounts, args })
            }
            _ => Err(anyhow!("Unknown instruction tag: {}", tag)),
        }
    }
}
