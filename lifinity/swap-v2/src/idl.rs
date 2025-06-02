pub use accounts_data::*;
pub use ix_data::*;
#[allow(dead_code)]
use std::convert::TryInto;
use std::mem;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    use serde::Serialize;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AmmFees {
        pub trade_fee_numerator: u64,
        pub trade_fee_denominator: u64,
        pub owner_trade_fee_numerator: u64,
        pub owner_trade_fee_denominator: u64,
        pub owner_withdraw_fee_numerator: u64,
        pub owner_withdraw_fee_denominator: u64,
        pub host_fee_numerator: u64,
        pub host_fee_denominator: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AmmCurve {
        pub curve_type: u8,
        pub curve_parameters: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AmmConfig {
        pub last_price: u64,
        pub last_balanced_price: u64,
        pub config_denominator: u64,
        pub volume_x: u64,
        pub volume_y: u64,
        pub volume_x_in_y: u64,
        pub deposit_cap: u64,
        pub regression_target: u64,
        pub oracle_type: u64,
        pub oracle_status: u64,
        pub oracle_main_slot_limit: u64,
        pub oracle_sub_confidence_limit: u64,
        pub oracle_sub_slot_limit: u64,
        pub oracle_pc_confidence_limit: u64,
        pub oracle_pc_slot_limit: u64,
        pub std_spread: u64,
        pub std_spread_buffer: u64,
        pub spread_coefficient: u64,
        pub price_buffer_coin: i64,
        pub price_buffer_pc: i64,
        pub rebalance_ratio: u64,
        pub fee_trade: u64,
        pub fee_platform: u64,
        pub oracle_main_slot_buffer: u64,
        pub config_temp4: u64,
        pub config_temp5: u64,
        pub config_temp6: u64,
        pub config_temp7: u64,
        pub config_temp8: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum CurveType {
        Standard,
        ConstantProduct,
    }
    impl Default for CurveType {
        fn default() -> Self {
            Self::Standard
        }
    }
}
pub mod accounts_data {
    use super::*;
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub authority: String,
        pub amm: String,
        pub userTransferAuthority: String,
        pub sourceInfo: String,
        pub destinationInfo: String,
        pub swapSource: String,
        pub swapDestination: String,
        pub poolMint: String,
        pub feeAccount: String,
        pub tokenProgram: String,
        pub oracleMainAccount: String,
        pub oracleSubAccount: String,
        pub oraclePcAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAllTokenTypesAccounts {
        pub amm: String,
        pub authority: String,
        pub userTransferAuthorityInfo: String,
        pub sourceAInfo: String,
        pub sourceBInfo: String,
        pub tokenA: String,
        pub tokenB: String,
        pub poolMint: String,
        pub destination: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAllTokenTypesAccounts {
        pub amm: String,
        pub authority: String,
        pub userTransferAuthorityInfo: String,
        pub sourceInfo: String,
        pub tokenA: String,
        pub tokenB: String,
        pub poolMint: String,
        pub destTokenAInfo: String,
        pub destTokenBInfo: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapArgs {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositAllTokenTypesArgs {
        pub pool_token_amount: u64,
        pub maximum_token_a_amount: u64,
        pub maximum_token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawAllTokenTypesArgs {
        pub pool_token_amount: u64,
        pub minimum_token_a_amount: u64,
        pub minimum_token_b_amount: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Swap {
        accounts: SwapAccounts,
        args: SwapArgs,
    },
    DepositAllTokenTypes {
        accounts: DepositAllTokenTypesAccounts,
        args: DepositAllTokenTypesArgs,
    },
    WithdrawAllTokenTypes {
        accounts: WithdrawAllTokenTypesAccounts,
        args: WithdrawAllTokenTypesArgs,
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
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<SwapArgs>());
                let args = SwapArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let amm = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let sourceInfo = keys.next().unwrap().clone();
                let destinationInfo = keys.next().unwrap().clone();
                let swapSource = keys.next().unwrap().clone();
                let swapDestination = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let oracleMainAccount = keys.next().unwrap().clone();
                let oracleSubAccount = keys.next().unwrap().clone();
                let oraclePcAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    authority,
                    amm,
                    userTransferAuthority,
                    sourceInfo,
                    destinationInfo,
                    swapSource,
                    swapDestination,
                    poolMint,
                    feeAccount,
                    tokenProgram,
                    oracleMainAccount,
                    oracleSubAccount,
                    oraclePcAccount,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [32u8, 95u8, 69u8, 60u8, 75u8, 79u8, 205u8, 238u8] => {
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<DepositAllTokenTypesArgs>());
                let args = DepositAllTokenTypesArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let amm = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let userTransferAuthorityInfo = keys.next().unwrap().clone();
                let sourceAInfo = keys.next().unwrap().clone();
                let sourceBInfo = keys.next().unwrap().clone();
                let tokenA = keys.next().unwrap().clone();
                let tokenB = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAllTokenTypesAccounts {
                    amm,
                    authority,
                    userTransferAuthorityInfo,
                    sourceAInfo,
                    sourceBInfo,
                    tokenA,
                    tokenB,
                    poolMint,
                    destination,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DepositAllTokenTypes { accounts, args });
            }
            [189u8, 254u8, 156u8, 174u8, 210u8, 9u8, 164u8, 216u8] => {
                let (args_bytes, _tail) =
                    rest.split_at(mem::size_of::<WithdrawAllTokenTypesArgs>());
                let args = WithdrawAllTokenTypesArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let amm = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let userTransferAuthorityInfo = keys.next().unwrap().clone();
                let sourceInfo = keys.next().unwrap().clone();
                let tokenA = keys.next().unwrap().clone();
                let tokenB = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let destTokenAInfo = keys.next().unwrap().clone();
                let destTokenBInfo = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAllTokenTypesAccounts {
                    amm,
                    authority,
                    userTransferAuthorityInfo,
                    sourceInfo,
                    tokenA,
                    tokenB,
                    poolMint,
                    destTokenAInfo,
                    destTokenBInfo,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawAllTokenTypes { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
