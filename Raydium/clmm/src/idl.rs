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
    pub struct InitializeRewardParam {
        pub open_time: u64,
        pub end_time: u64,
        pub emissions_per_second_x64: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Observation {
        pub block_timestamp: u32,
        pub tick_cumulative: i64,
        pub padding: [u64; 4usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PositionRewardInfo {
        pub growth_inside_last_x64: u128,
        pub reward_amount_owed: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardInfo {
        pub reward_state: u8,
        pub open_time: u64,
        pub end_time: u64,
        pub last_update_time: u64,
        pub emissions_per_second_x64: u128,
        pub reward_total_emissioned: u64,
        pub reward_claimed: u64,
        #[serde(with = "pubkey_serde")]
        pub token_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_vault: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        pub reward_growth_global_x64: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TickState {
        pub tick: i32,
        pub liquidity_net: i128,
        pub liquidity_gross: u128,
        pub fee_growth_outside0_x64: u128,
        pub fee_growth_outside1_x64: u128,
        pub reward_growths_outside_x64: [u128; 3usize],
        pub padding: [u32; 13usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PoolStatusBitIndex {
        OpenPositionOrIncreaseLiquidity,
        DecreaseLiquidity,
        CollectFee,
        CollectReward,
        Swap,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PoolStatusBitFlag {
        Enable,
        Disable,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RewardState {
        Uninitialized,
        Initialized,
        Opening,
        Ended,
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
    pub struct CreatePoolAccounts {
        pub poolCreator: String,
        pub ammConfig: String,
        pub poolState: String,
        pub tokenMint0: String,
        pub tokenMint1: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub observationState: String,
        pub tickArrayBitmap: String,
        pub tokenProgram0: String,
        pub tokenProgram1: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: String,
        pub poolState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOperationAccountAccounts {
        pub owner: String,
        pub operationState: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateOperationAccountAccounts {
        pub owner: String,
        pub operationState: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferRewardOwnerAccounts {
        pub authority: String,
        pub poolState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeRewardAccounts {
        pub rewardFunder: String,
        pub funderTokenAccount: String,
        pub ammConfig: String,
        pub poolState: String,
        pub operationState: String,
        pub rewardTokenMint: String,
        pub rewardTokenVault: String,
        pub rewardTokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectRemainingRewardsAccounts {
        pub rewardFunder: String,
        pub funderTokenAccount: String,
        pub poolState: String,
        pub rewardTokenVault: String,
        pub rewardVaultMint: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub memoProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateRewardInfosAccounts {
        pub poolState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetRewardParamsAccounts {
        pub authority: String,
        pub ammConfig: String,
        pub poolState: String,
        pub operationState: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: String,
        pub poolState: String,
        pub ammConfig: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub recipientTokenAccount0: String,
        pub recipientTokenAccount1: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectFundFeeAccounts {
        pub owner: String,
        pub poolState: String,
        pub ammConfig: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub recipientTokenAccount0: String,
        pub recipientTokenAccount1: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenPositionAccounts {
        pub payer: String,
        pub positionNftOwner: String,
        pub positionNftMint: String,
        pub positionNftAccount: String,
        pub metadataAccount: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub personalPosition: String,
        pub tokenAccount0: String,
        pub tokenAccount1: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub rent: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub metadataProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenPositionV2Accounts {
        pub payer: String,
        pub positionNftOwner: String,
        pub positionNftMint: String,
        pub positionNftAccount: String,
        pub metadataAccount: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub personalPosition: String,
        pub tokenAccount0: String,
        pub tokenAccount1: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub rent: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub metadataProgram: String,
        pub tokenProgram2022: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenPositionWithToken22NftAccounts {
        pub payer: String,
        pub positionNftOwner: String,
        pub positionNftMint: String,
        pub positionNftAccount: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub personalPosition: String,
        pub tokenAccount0: String,
        pub tokenAccount1: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub rent: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub tokenProgram2022: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePositionAccounts {
        pub nftOwner: String,
        pub positionNftMint: String,
        pub positionNftAccount: String,
        pub personalPosition: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreaseLiquidityAccounts {
        pub nftOwner: String,
        pub nftAccount: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub personalPosition: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub tokenAccount0: String,
        pub tokenAccount1: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreaseLiquidityV2Accounts {
        pub nftOwner: String,
        pub nftAccount: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub personalPosition: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub tokenAccount0: String,
        pub tokenAccount1: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecreaseLiquidityAccounts {
        pub nftOwner: String,
        pub nftAccount: String,
        pub personalPosition: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub recipientTokenAccount0: String,
        pub recipientTokenAccount1: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecreaseLiquidityV2Accounts {
        pub nftOwner: String,
        pub nftAccount: String,
        pub personalPosition: String,
        pub poolState: String,
        pub protocolPosition: String,
        pub tokenVault0: String,
        pub tokenVault1: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub recipientTokenAccount0: String,
        pub recipientTokenAccount1: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub memoProgram: String,
        pub vault0Mint: String,
        pub vault1Mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub payer: String,
        pub ammConfig: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub observationState: String,
        pub tokenProgram: String,
        pub tickArray: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapV2Accounts {
        pub payer: String,
        pub ammConfig: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub observationState: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub memoProgram: String,
        pub inputVaultMint: String,
        pub outputVaultMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapRouterBaseInAccounts {
        pub payer: String,
        pub inputTokenAccount: String,
        pub inputTokenMint: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        pub memoProgram: String,
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
        pub tick_spacing: u16,
        pub trade_fee_rate: u32,
        pub protocol_fee_rate: u32,
        pub fund_fee_rate: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateAmmConfigArguments {
        pub param: u8,
        pub value: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePoolArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub open_time: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePoolStatusArguments {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOperationAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateOperationAccountArguments {
        pub param: u8,
        pub keys: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferRewardOwnerArguments {
        #[serde(with = "pubkey_serde")]
        pub new_owner: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeRewardArguments {
        pub param: InitializeRewardParam,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectRemainingRewardsArguments {
        pub reward_index: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardInfosArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetRewardParamsArguments {
        pub reward_index: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub emissions_per_second_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub open_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub end_time: u64,
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
    pub struct OpenPositionArguments {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_max: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OpenPositionV2Arguments {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_max: u64,
        pub with_metadata: bool,
        pub base_flag: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OpenPositionWithToken22NftArguments {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_max: u64,
        pub with_metadata: bool,
        pub base_flag: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_max: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseLiquidityV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_max: u64,
        pub base_flag: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreaseLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_min: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_min: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreaseLiquidityV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount0_min: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount1_min: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub other_amount_threshold: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_limit_x64: u128,
        pub is_base_input: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub other_amount_threshold: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_limit_x64: u128,
        pub is_base_input: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapRouterBaseInArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_out_minimum: u64,
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
    CreatePool {
        accounts: CreatePoolAccounts,
        args: CreatePoolArguments,
    },
    UpdatePoolStatus {
        accounts: UpdatePoolStatusAccounts,
        args: UpdatePoolStatusArguments,
    },
    CreateOperationAccount {
        accounts: CreateOperationAccountAccounts,
        args: CreateOperationAccountArguments,
    },
    UpdateOperationAccount {
        accounts: UpdateOperationAccountAccounts,
        args: UpdateOperationAccountArguments,
    },
    TransferRewardOwner {
        accounts: TransferRewardOwnerAccounts,
        args: TransferRewardOwnerArguments,
    },
    InitializeReward {
        accounts: InitializeRewardAccounts,
        args: InitializeRewardArguments,
    },
    CollectRemainingRewards {
        accounts: CollectRemainingRewardsAccounts,
        args: CollectRemainingRewardsArguments,
    },
    UpdateRewardInfos {
        accounts: UpdateRewardInfosAccounts,
        args: UpdateRewardInfosArguments,
    },
    SetRewardParams {
        accounts: SetRewardParamsAccounts,
        args: SetRewardParamsArguments,
    },
    CollectProtocolFee {
        accounts: CollectProtocolFeeAccounts,
        args: CollectProtocolFeeArguments,
    },
    CollectFundFee {
        accounts: CollectFundFeeAccounts,
        args: CollectFundFeeArguments,
    },
    OpenPosition {
        accounts: OpenPositionAccounts,
        args: OpenPositionArguments,
    },
    OpenPositionV2 {
        accounts: OpenPositionV2Accounts,
        args: OpenPositionV2Arguments,
    },
    OpenPositionWithToken22Nft {
        accounts: OpenPositionWithToken22NftAccounts,
        args: OpenPositionWithToken22NftArguments,
    },
    ClosePosition {
        accounts: ClosePositionAccounts,
        args: ClosePositionArguments,
    },
    IncreaseLiquidity {
        accounts: IncreaseLiquidityAccounts,
        args: IncreaseLiquidityArguments,
    },
    IncreaseLiquidityV2 {
        accounts: IncreaseLiquidityV2Accounts,
        args: IncreaseLiquidityV2Arguments,
    },
    DecreaseLiquidity {
        accounts: DecreaseLiquidityAccounts,
        args: DecreaseLiquidityArguments,
    },
    DecreaseLiquidityV2 {
        accounts: DecreaseLiquidityV2Accounts,
        args: DecreaseLiquidityV2Arguments,
    },
    Swap {
        accounts: SwapAccounts,
        args: SwapArguments,
    },
    SwapV2 {
        accounts: SwapV2Accounts,
        args: SwapV2Arguments,
    },
    SwapRouterBaseIn {
        accounts: SwapRouterBaseInAccounts,
        args: SwapRouterBaseInArguments,
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
            [233u8, 146u8, 209u8, 142u8, 207u8, 104u8, 64u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreatePoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let poolCreator = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let tokenMint0 = keys.next().unwrap().clone();
                let tokenMint1 = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let tickArrayBitmap = keys.next().unwrap().clone();
                let tokenProgram0 = keys.next().unwrap().clone();
                let tokenProgram1 = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreatePoolAccounts {
                    poolCreator,
                    ammConfig,
                    poolState,
                    tokenMint0,
                    tokenMint1,
                    tokenVault0,
                    tokenVault1,
                    observationState,
                    tickArrayBitmap,
                    tokenProgram0,
                    tokenProgram1,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreatePool { accounts, args });
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
            [63u8, 87u8, 148u8, 33u8, 109u8, 35u8, 8u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateOperationAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let operationState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateOperationAccountAccounts {
                    owner,
                    operationState,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateOperationAccount { accounts, args });
            }
            [127u8, 70u8, 119u8, 40u8, 188u8, 227u8, 61u8, 7u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateOperationAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let operationState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateOperationAccountAccounts {
                    owner,
                    operationState,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::UpdateOperationAccount { accounts, args });
            }
            [7u8, 22u8, 12u8, 83u8, 242u8, 43u8, 48u8, 121u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferRewardOwnerArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferRewardOwnerAccounts {
                    authority,
                    poolState,
                    remaining,
                };
                return Ok(Instruction::TransferRewardOwner { accounts, args });
            }
            [95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeRewardArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let rewardFunder = keys.next().unwrap().clone();
                let funderTokenAccount = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let operationState = keys.next().unwrap().clone();
                let rewardTokenMint = keys.next().unwrap().clone();
                let rewardTokenVault = keys.next().unwrap().clone();
                let rewardTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeRewardAccounts {
                    rewardFunder,
                    funderTokenAccount,
                    ammConfig,
                    poolState,
                    operationState,
                    rewardTokenMint,
                    rewardTokenVault,
                    rewardTokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeReward { accounts, args });
            }
            [18u8, 237u8, 166u8, 197u8, 34u8, 16u8, 213u8, 144u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectRemainingRewardsArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let rewardFunder = keys.next().unwrap().clone();
                let funderTokenAccount = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let rewardTokenVault = keys.next().unwrap().clone();
                let rewardVaultMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectRemainingRewardsAccounts {
                    rewardFunder,
                    funderTokenAccount,
                    poolState,
                    rewardTokenVault,
                    rewardVaultMint,
                    tokenProgram,
                    tokenProgram2022,
                    memoProgram,
                    remaining,
                };
                return Ok(Instruction::CollectRemainingRewards { accounts, args });
            }
            [163u8, 172u8, 224u8, 52u8, 11u8, 154u8, 106u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateRewardInfosArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let poolState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateRewardInfosAccounts {
                    poolState,
                    remaining,
                };
                return Ok(Instruction::UpdateRewardInfos { accounts, args });
            }
            [112u8, 52u8, 167u8, 75u8, 32u8, 201u8, 211u8, 137u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetRewardParamsArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let operationState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetRewardParamsAccounts {
                    authority,
                    ammConfig,
                    poolState,
                    operationState,
                    tokenProgram,
                    tokenProgram2022,
                    remaining,
                };
                return Ok(Instruction::SetRewardParams { accounts, args });
            }
            [136u8, 136u8, 252u8, 221u8, 194u8, 66u8, 126u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectProtocolFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let recipientTokenAccount0 = keys.next().unwrap().clone();
                let recipientTokenAccount1 = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectProtocolFeeAccounts {
                    owner,
                    poolState,
                    ammConfig,
                    tokenVault0,
                    tokenVault1,
                    vault0Mint,
                    vault1Mint,
                    recipientTokenAccount0,
                    recipientTokenAccount1,
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
                let poolState = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let recipientTokenAccount0 = keys.next().unwrap().clone();
                let recipientTokenAccount1 = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectFundFeeAccounts {
                    owner,
                    poolState,
                    ammConfig,
                    tokenVault0,
                    tokenVault1,
                    vault0Mint,
                    vault1Mint,
                    recipientTokenAccount0,
                    recipientTokenAccount1,
                    tokenProgram,
                    tokenProgram2022,
                    remaining,
                };
                return Ok(Instruction::CollectFundFee { accounts, args });
            }
            [135u8, 128u8, 47u8, 77u8, 15u8, 152u8, 240u8, 49u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenPositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let positionNftOwner = keys.next().unwrap().clone();
                let positionNftMint = keys.next().unwrap().clone();
                let positionNftAccount = keys.next().unwrap().clone();
                let metadataAccount = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let tokenAccount0 = keys.next().unwrap().clone();
                let tokenAccount1 = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OpenPositionAccounts {
                    payer,
                    positionNftOwner,
                    positionNftMint,
                    positionNftAccount,
                    metadataAccount,
                    poolState,
                    protocolPosition,
                    tickArrayLower,
                    tickArrayUpper,
                    personalPosition,
                    tokenAccount0,
                    tokenAccount1,
                    tokenVault0,
                    tokenVault1,
                    rent,
                    systemProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    metadataProgram,
                    remaining,
                };
                return Ok(Instruction::OpenPosition { accounts, args });
            }
            [77u8, 184u8, 74u8, 214u8, 112u8, 86u8, 241u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenPositionV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let positionNftOwner = keys.next().unwrap().clone();
                let positionNftMint = keys.next().unwrap().clone();
                let positionNftAccount = keys.next().unwrap().clone();
                let metadataAccount = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let tokenAccount0 = keys.next().unwrap().clone();
                let tokenAccount1 = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OpenPositionV2Accounts {
                    payer,
                    positionNftOwner,
                    positionNftMint,
                    positionNftAccount,
                    metadataAccount,
                    poolState,
                    protocolPosition,
                    tickArrayLower,
                    tickArrayUpper,
                    personalPosition,
                    tokenAccount0,
                    tokenAccount1,
                    tokenVault0,
                    tokenVault1,
                    rent,
                    systemProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    metadataProgram,
                    tokenProgram2022,
                    vault0Mint,
                    vault1Mint,
                    remaining,
                };
                return Ok(Instruction::OpenPositionV2 { accounts, args });
            }
            [77u8, 255u8, 174u8, 82u8, 125u8, 29u8, 201u8, 46u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenPositionWithToken22NftArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let positionNftOwner = keys.next().unwrap().clone();
                let positionNftMint = keys.next().unwrap().clone();
                let positionNftAccount = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let tokenAccount0 = keys.next().unwrap().clone();
                let tokenAccount1 = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OpenPositionWithToken22NftAccounts {
                    payer,
                    positionNftOwner,
                    positionNftMint,
                    positionNftAccount,
                    poolState,
                    protocolPosition,
                    tickArrayLower,
                    tickArrayUpper,
                    personalPosition,
                    tokenAccount0,
                    tokenAccount1,
                    tokenVault0,
                    tokenVault1,
                    rent,
                    systemProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    tokenProgram2022,
                    vault0Mint,
                    vault1Mint,
                    remaining,
                };
                return Ok(Instruction::OpenPositionWithToken22Nft { accounts, args });
            }
            [123u8, 134u8, 81u8, 0u8, 49u8, 68u8, 98u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let nftOwner = keys.next().unwrap().clone();
                let positionNftMint = keys.next().unwrap().clone();
                let positionNftAccount = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePositionAccounts {
                    nftOwner,
                    positionNftMint,
                    positionNftAccount,
                    personalPosition,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::ClosePosition { accounts, args });
            }
            [46u8, 156u8, 243u8, 118u8, 13u8, 205u8, 251u8, 178u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let nftOwner = keys.next().unwrap().clone();
                let nftAccount = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let tokenAccount0 = keys.next().unwrap().clone();
                let tokenAccount1 = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreaseLiquidityAccounts {
                    nftOwner,
                    nftAccount,
                    poolState,
                    protocolPosition,
                    personalPosition,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAccount0,
                    tokenAccount1,
                    tokenVault0,
                    tokenVault1,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::IncreaseLiquidity { accounts, args });
            }
            [133u8, 29u8, 89u8, 223u8, 69u8, 238u8, 176u8, 10u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseLiquidityV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let nftOwner = keys.next().unwrap().clone();
                let nftAccount = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let tokenAccount0 = keys.next().unwrap().clone();
                let tokenAccount1 = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreaseLiquidityV2Accounts {
                    nftOwner,
                    nftAccount,
                    poolState,
                    protocolPosition,
                    personalPosition,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAccount0,
                    tokenAccount1,
                    tokenVault0,
                    tokenVault1,
                    tokenProgram,
                    tokenProgram2022,
                    vault0Mint,
                    vault1Mint,
                    remaining,
                };
                return Ok(Instruction::IncreaseLiquidityV2 { accounts, args });
            }
            [160u8, 38u8, 208u8, 111u8, 104u8, 91u8, 44u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecreaseLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let nftOwner = keys.next().unwrap().clone();
                let nftAccount = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let recipientTokenAccount0 = keys.next().unwrap().clone();
                let recipientTokenAccount1 = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DecreaseLiquidityAccounts {
                    nftOwner,
                    nftAccount,
                    personalPosition,
                    poolState,
                    protocolPosition,
                    tokenVault0,
                    tokenVault1,
                    tickArrayLower,
                    tickArrayUpper,
                    recipientTokenAccount0,
                    recipientTokenAccount1,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DecreaseLiquidity { accounts, args });
            }
            [58u8, 127u8, 188u8, 62u8, 79u8, 82u8, 196u8, 96u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecreaseLiquidityV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let nftOwner = keys.next().unwrap().clone();
                let nftAccount = keys.next().unwrap().clone();
                let personalPosition = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let protocolPosition = keys.next().unwrap().clone();
                let tokenVault0 = keys.next().unwrap().clone();
                let tokenVault1 = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let recipientTokenAccount0 = keys.next().unwrap().clone();
                let recipientTokenAccount1 = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let vault0Mint = keys.next().unwrap().clone();
                let vault1Mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DecreaseLiquidityV2Accounts {
                    nftOwner,
                    nftAccount,
                    personalPosition,
                    poolState,
                    protocolPosition,
                    tokenVault0,
                    tokenVault1,
                    tickArrayLower,
                    tickArrayUpper,
                    recipientTokenAccount0,
                    recipientTokenAccount1,
                    tokenProgram,
                    tokenProgram2022,
                    memoProgram,
                    vault0Mint,
                    vault1Mint,
                    remaining,
                };
                return Ok(Instruction::DecreaseLiquidityV2 { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tickArray = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    payer,
                    ammConfig,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    observationState,
                    tokenProgram,
                    tickArray,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [43u8, 4u8, 237u8, 11u8, 26u8, 201u8, 30u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let inputVaultMint = keys.next().unwrap().clone();
                let outputVaultMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapV2Accounts {
                    payer,
                    ammConfig,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    observationState,
                    tokenProgram,
                    tokenProgram2022,
                    memoProgram,
                    inputVaultMint,
                    outputVaultMint,
                    remaining,
                };
                return Ok(Instruction::SwapV2 { accounts, args });
            }
            [69u8, 125u8, 115u8, 218u8, 245u8, 186u8, 242u8, 196u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapRouterBaseInArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let inputTokenMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapRouterBaseInAccounts {
                    payer,
                    inputTokenAccount,
                    inputTokenMint,
                    tokenProgram,
                    tokenProgram2022,
                    memoProgram,
                    remaining,
                };
                return Ok(Instruction::SwapRouterBaseIn { accounts, args });
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
    pub struct ConfigChangeEvent {
        pub index: u16,
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        pub protocol_fee_rate: u32,
        pub trade_fee_rate: u32,
        pub tick_spacing: u16,
        pub fund_fee_rate: u32,
        #[serde(with = "pubkey_serde")]
        pub fund_owner: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePersonalPositionEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub minter: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub nft_owner: [u8; 32usize],
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub liquidity: u128,
        pub deposit_amount0: u64,
        pub deposit_amount1: u64,
        pub deposit_amount0_transfer_fee: u64,
        pub deposit_amount1_transfer_fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseLiquidityEvent {
        #[serde(with = "pubkey_serde")]
        pub position_nft_mint: [u8; 32usize],
        pub liquidity: u128,
        pub amount0: u64,
        pub amount1: u64,
        pub amount0_transfer_fee: u64,
        pub amount1_transfer_fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreaseLiquidityEvent {
        #[serde(with = "pubkey_serde")]
        pub position_nft_mint: [u8; 32usize],
        pub liquidity: u128,
        pub decrease_amount0: u64,
        pub decrease_amount1: u64,
        pub fee_amount0: u64,
        pub fee_amount1: u64,
        pub reward_amounts: [u64; 3usize],
        pub transfer_fee0: u64,
        pub transfer_fee1: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidityCalculateEvent {
        pub pool_liquidity: u128,
        pub pool_sqrt_price_x64: u128,
        pub pool_tick: i32,
        pub calc_amount0: u64,
        pub calc_amount1: u64,
        pub trade_fee_owed0: u64,
        pub trade_fee_owed1: u64,
        pub transfer_fee0: u64,
        pub transfer_fee1: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectPersonalFeeEvent {
        #[serde(with = "pubkey_serde")]
        pub position_nft_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account0: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account1: [u8; 32usize],
        pub amount0: u64,
        pub amount1: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardInfosEvent {
        pub reward_growth_global_x64: [u128; 3usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PoolCreatedEvent {
        #[serde(with = "pubkey_serde")]
        pub token_mint0: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_mint1: [u8; 32usize],
        pub tick_spacing: u16,
        #[serde(with = "pubkey_serde")]
        pub pool_state: [u8; 32usize],
        pub sqrt_price_x64: u128,
        pub tick: i32,
        #[serde(with = "pubkey_serde")]
        pub token_vault0: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_vault1: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectProtocolFeeEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account0: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account1: [u8; 32usize],
        pub amount0: u64,
        pub amount1: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub sender: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_account0: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_account1: [u8; 32usize],
        pub amount0: u64,
        pub transfer_fee0: u64,
        pub amount1: u64,
        pub transfer_fee1: u64,
        pub zero_for_one: bool,
        pub sqrt_price_x64: u128,
        pub liquidity: u128,
        pub tick: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidityChangeEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: [u8; 32usize],
        pub tick: i32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub liquidity_before: u128,
        pub liquidity_after: u128,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        ConfigChangeEvent { args: ConfigChangeEvent },
        CreatePersonalPositionEvent { args: CreatePersonalPositionEvent },
        IncreaseLiquidityEvent { args: IncreaseLiquidityEvent },
        DecreaseLiquidityEvent { args: DecreaseLiquidityEvent },
        LiquidityCalculateEvent { args: LiquidityCalculateEvent },
        CollectPersonalFeeEvent { args: CollectPersonalFeeEvent },
        UpdateRewardInfosEvent { args: UpdateRewardInfosEvent },
        PoolCreatedEvent { args: PoolCreatedEvent },
        CollectProtocolFeeEvent { args: CollectProtocolFeeEvent },
        SwapEvent { args: SwapEvent },
        LiquidityChangeEvent { args: LiquidityChangeEvent },
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
                [247u8, 189u8, 7u8, 119u8, 106u8, 112u8, 95u8, 151u8] => {
                    let mut rdr = &payload[..];
                    let args = ConfigChangeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ConfigChangeEvent { args });
                }
                [100u8, 30u8, 87u8, 249u8, 196u8, 223u8, 154u8, 206u8] => {
                    let mut rdr = &payload[..];
                    let args = CreatePersonalPositionEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CreatePersonalPositionEvent { args });
                }
                [49u8, 79u8, 105u8, 212u8, 32u8, 34u8, 30u8, 84u8] => {
                    let mut rdr = &payload[..];
                    let args = IncreaseLiquidityEvent::deserialize(&mut rdr)?;
                    return Ok(Event::IncreaseLiquidityEvent { args });
                }
                [58u8, 222u8, 86u8, 58u8, 68u8, 50u8, 85u8, 56u8] => {
                    let mut rdr = &payload[..];
                    let args = DecreaseLiquidityEvent::deserialize(&mut rdr)?;
                    return Ok(Event::DecreaseLiquidityEvent { args });
                }
                [237u8, 112u8, 148u8, 230u8, 57u8, 84u8, 180u8, 162u8] => {
                    let mut rdr = &payload[..];
                    let args = LiquidityCalculateEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LiquidityCalculateEvent { args });
                }
                [166u8, 174u8, 105u8, 192u8, 81u8, 161u8, 83u8, 105u8] => {
                    let mut rdr = &payload[..];
                    let args = CollectPersonalFeeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CollectPersonalFeeEvent { args });
                }
                [109u8, 127u8, 186u8, 78u8, 114u8, 65u8, 37u8, 236u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdateRewardInfosEvent::deserialize(&mut rdr)?;
                    return Ok(Event::UpdateRewardInfosEvent { args });
                }
                [25u8, 94u8, 75u8, 47u8, 112u8, 99u8, 53u8, 63u8] => {
                    let mut rdr = &payload[..];
                    let args = PoolCreatedEvent::deserialize(&mut rdr)?;
                    return Ok(Event::PoolCreatedEvent { args });
                }
                [206u8, 87u8, 17u8, 79u8, 45u8, 41u8, 213u8, 61u8] => {
                    let mut rdr = &payload[..];
                    let args = CollectProtocolFeeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CollectProtocolFeeEvent { args });
                }
                [64u8, 198u8, 205u8, 232u8, 38u8, 8u8, 113u8, 226u8] => {
                    let mut rdr = &payload[..];
                    let args = SwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapEvent { args });
                }
                [126u8, 240u8, 175u8, 206u8, 158u8, 88u8, 153u8, 107u8] => {
                    let mut rdr = &payload[..];
                    let args = LiquidityChangeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LiquidityChangeEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
