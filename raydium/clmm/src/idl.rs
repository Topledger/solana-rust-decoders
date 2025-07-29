extern crate serde;
pub use accounts_data::*;
use anchor_lang::prelude::*;
#[allow(dead_code)]
use borsh::BorshDeserialize;
pub use ix_data::*;
pub use typedefs::*;
pub mod typedefs {
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use serde::Serialize;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AmmConfig {
        pub bump: u8,
        pub index: u16,
        #[serde(with = "pubkey_serde")]
        pub owner: Pubkey,
        pub protocol_fee_rate: u32,
        pub trade_fee_rate: u32,
        pub tick_spacing: u16,
        pub fund_fee_rate: u32,
        pub padding_u32: u32,
        #[serde(with = "pubkey_serde")]
        pub fund_owner: Pubkey,
        pub padding: [u64; 3],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CollectPersonalFeeEvent {
        #[serde(with = "pubkey_serde")]
        pub position_nft_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account_1: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CollectProtocolFeeEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub recipient_token_account_1: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ConfigChangeEvent {
        pub index: u16,
        #[serde(with = "pubkey_serde")]
        pub owner: Pubkey,
        pub protocol_fee_rate: u32,
        pub trade_fee_rate: u32,
        pub tick_spacing: u16,
        pub fund_fee_rate: u32,
        #[serde(with = "pubkey_serde")]
        pub fund_owner: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CreatePersonalPositionEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub minter: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub nft_owner: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposit_amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposit_amount_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposit_amount_0_transfer_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposit_amount_1_transfer_fee: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct DecreaseLiquidityEvent {
        #[serde(with = "pubkey_serde")]
        pub position_nft_mint: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub decrease_amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub decrease_amount_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_amount_1: u64,
        pub reward_amounts: [u64; 3],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transfer_fee_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transfer_fee_1: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct IncreaseLiquidityEvent {
        #[serde(with = "pubkey_serde")]
        pub position_nft_mint: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_transfer_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_transfer_fee: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InitializeRewardParam {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub open_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub end_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub emissions_per_second_x64: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LiquidityCalculateEvent {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pool_liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pool_sqrt_price_x64: u128,
        pub pool_tick: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub calc_amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub calc_amount_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trade_fee_owed_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trade_fee_owed_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transfer_fee_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transfer_fee_1: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LiquidityChangeEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        pub tick: i32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_before: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_after: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Observation {
        pub block_timestamp: u32,
        pub tick_cumulative: i64,
        pub padding: [u64; 4],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ObservationState {
        pub initialized: bool,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub recent_epoch: u64,
        pub observation_index: u16,
        #[serde(with = "pubkey_serde")]
        pub pool_id: Pubkey,
        #[serde(skip_serializing)]
        pub observations: [Observation; 100],
        pub padding: [u64; 4],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct OperationState {
        pub bump: u8,
        pub operation_owners: [Pubkey; 10],
        #[serde(skip_serializing)]
        pub whitelist_mints: [Pubkey; 100],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PersonalPositionState {
        pub bump: [u8; 1],
        #[serde(with = "pubkey_serde")]
        pub nft_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub pool_id: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_inside_0_last_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_inside_1_last_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_fees_owed_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_fees_owed_1: u64,
        pub reward_infos: [PositionRewardInfo; 3],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub recent_epoch: u64,
        pub padding: [u64; 7],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PoolCreatedEvent {
        #[serde(with = "pubkey_serde")]
        pub token_mint_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_mint_1: Pubkey,
        pub tick_spacing: u16,
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_x64: u128,
        pub tick: i32,
        #[serde(with = "pubkey_serde")]
        pub token_vault_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_vault_1: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PoolState {
        pub bump: [u8; 1],
        #[serde(with = "pubkey_serde")]
        pub amm_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub owner: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_mint_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_mint_1: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_vault_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_vault_1: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub observation_key: Pubkey,
        pub mint_decimals_0: u8,
        pub mint_decimals_1: u8,
        pub tick_spacing: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_x64: u128,
        pub tick_current: i32,
        pub padding3: u16,
        pub padding4: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_global_0_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_global_1_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub protocol_fees_token_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub protocol_fees_token_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_in_amount_token_0: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_out_amount_token_1: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_in_amount_token_1: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_out_amount_token_0: u128,
        pub status: u8,
        pub padding: [u8; 7],
        pub reward_infos: [RewardInfo; 3],
        pub tick_array_bitmap: [u64; 16],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fees_token_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fees_claimed_token_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fees_token_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fees_claimed_token_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fund_fees_token_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fund_fees_token_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub open_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub recent_epoch: u64,
        pub padding1: [u64; 24],
        pub padding2: [u64; 32],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PositionRewardInfo {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub growth_inside_last_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_amount_owed: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ProtocolPositionState {
        pub bump: u8,
        #[serde(with = "pubkey_serde")]
        pub pool_id: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_inside_0_last_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_inside_1_last_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_fees_owed_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_fees_owed_1: u64,
        pub reward_growth_inside: [u128; 3],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub recent_epoch: u64,
        pub padding: [u64; 7],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RewardInfo {
        pub reward_state: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub open_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub end_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_update_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub emissions_per_second_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_total_emissioned: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_claimed: u64,
        #[serde(with = "pubkey_serde")]
        pub token_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_vault: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_growth_global_x64: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SupportMintAssociated {
        pub bump: u8,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub padding: [u64; 8],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub sender: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_account_0: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_account_1: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transfer_fee_0: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transfer_fee_1: u64,
        pub zero_for_one: bool,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        pub tick: i32,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TickArrayBitmapExtension {
        #[serde(with = "pubkey_serde")]
        pub pool_id: Pubkey,
        pub positive_tick_array_bitmap: [[u64; 8]; 14],
        pub negative_tick_array_bitmap: [[u64; 8]; 14],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct TickArrayState {
        #[serde(with = "pubkey_serde")]
        pub pool_id: Pubkey,
        pub start_tick_index: i32,
        #[serde(skip_serializing)]
        pub ticks: [TickState; 60],
        pub initialized_tick_count: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub recent_epoch: u64,
        #[serde(skip_serializing)]
        pub padding: [u8; 107],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TickState {
        pub tick: i32,
        pub liquidity_net: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_gross: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_outside_0_x64: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_growth_outside_1_x64: u128,
        pub reward_growths_outside_x64: [u128; 3],
        pub padding: [u32; 13],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateRewardInfosEvent {
        pub reward_growth_global_x64: [u128; 3],
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct ClosePositionAccounts {
        pub nft_owner: String,
        pub position_nft_mint: String,
        pub position_nft_account: String,
        pub personal_position: String,
        pub system_program: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectFundFeeAccounts {
        pub owner: String,
        pub pool_state: String,
        pub amm_config: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub recipient_token_account_0: Option<String>,
        pub recipient_token_account_1: Option<String>,
        pub token_program: String,
        pub token_program_2022: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: String,
        pub pool_state: String,
        pub amm_config: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub recipient_token_account_0: Option<String>,
        pub recipient_token_account_1: Option<String>,
        pub token_program: String,
        pub token_program_2022: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectRemainingRewardsAccounts {
        pub reward_funder: String,
        pub funder_token_account: String,
        pub pool_state: String,
        pub reward_token_vault: String,
        pub reward_vault_mint: Option<String>,
        pub token_program: String,
        pub token_program_2022: Option<String>,
        pub memo_program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateAmmConfigAccounts {
        pub owner: String,
        pub amm_config: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOperationAccountAccounts {
        pub owner: String,
        pub operation_state: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreatePoolAccounts {
        pub pool_creator: String,
        pub amm_config: String,
        pub pool_state: String,
        pub token_mint_0: String,
        pub token_mint_1: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub observation_state: String,
        pub tick_array_bitmap: Option<String>,
        pub token_program_0: String,
        pub token_program_1: Option<String>,
        pub system_program: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateSupportMintAssociatedAccounts {
        pub owner: String,
        pub token_mint: String,
        pub support_mint_associated: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecreaseLiquidityAccounts {
        pub nft_owner: String,
        pub nft_account: String,
        pub personal_position: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub recipient_token_account_0: String,
        pub recipient_token_account_1: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecreaseLiquidityV2Accounts {
        pub nft_owner: String,
        pub nft_account: String,
        pub personal_position: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub recipient_token_account_0: String,
        pub recipient_token_account_1: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub memo_program: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreaseLiquidityAccounts {
        pub nft_owner: String,
        pub nft_account: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub personal_position: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub token_account_0: String,
        pub token_account_1: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreaseLiquidityV2Accounts {
        pub nft_owner: String,
        pub nft_account: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub personal_position: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub token_account_0: String,
        pub token_account_1: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeRewardAccounts {
        pub reward_funder: String,
        pub funder_token_account: String,
        pub amm_config: String,
        pub pool_state: String,
        pub operation_state: String,
        pub reward_token_mint: String,
        pub reward_token_vault: Option<String>,
        pub reward_token_program: String,
        pub system_program: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenPositionAccounts {
        pub payer: String,
        pub position_nft_owner: String,
        pub position_nft_mint: String,
        pub position_nft_account: String,
        pub metadata_account: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub personal_position: String,
        pub token_account_0: String,
        pub token_account_1: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub rent: String,
        pub system_program: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub metadata_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenPositionV2Accounts {
        pub payer: String,
        pub position_nft_owner: String,
        pub position_nft_mint: String,
        pub position_nft_account: String,
        pub metadata_account: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub personal_position: String,
        pub token_account_0: String,
        pub token_account_1: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub rent: String,
        pub system_program: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub metadata_program: String,
        pub token_program_2022: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenPositionWithToken22NftAccounts {
        pub payer: String,
        pub position_nft_owner: String,
        pub position_nft_mint: String,
        pub position_nft_account: String,
        pub pool_state: String,
        pub protocol_position: String,
        pub tick_array_lower: String,
        pub tick_array_upper: String,
        pub personal_position: String,
        pub token_account_0: String,
        pub token_account_1: String,
        pub token_vault_0: String,
        pub token_vault_1: String,
        pub rent: String,
        pub system_program: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub token_program_2022: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetRewardParamsAccounts {
        pub authority: String,
        pub amm_config: String,
        pub pool_state: String,
        pub operation_state: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub payer: String,
        pub amm_config: String,
        pub pool_state: String,
        pub input_token_account: String,
        pub output_token_account: String,
        pub input_vault: String,
        pub output_vault: String,
        pub observation_state: String,
        pub token_program: String,
        pub tick_array: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapRouterBaseInAccounts {
        pub payer: String,
        pub input_token_account: String,
        pub input_token_mint: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub memo_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapV2Accounts {
        pub payer: String,
        pub amm_config: String,
        pub pool_state: String,
        pub input_token_account: String,
        pub output_token_account: String,
        pub input_vault: String,
        pub output_vault: String,
        pub observation_state: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub memo_program: String,
        pub input_vault_mint: String,
        pub output_vault_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferRewardOwnerAccounts {
        pub authority: String,
        pub pool_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAmmConfigAccounts {
        pub owner: String,
        pub amm_config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateOperationAccountAccounts {
        pub owner: String,
        pub operation_state: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: String,
        pub pool_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateRewardInfosAccounts {
        pub pool_state: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectFundFeeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_requested: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectProtocolFeeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_requested: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectRemainingRewardsArguments {
        pub reward_index: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateAmmConfigArguments {
        pub index: u16,
        pub tick_spacing: u16,
        pub trade_fee_rate: u32,
        pub protocol_fee_rate: u32,
        pub fund_fee_rate: Option<u32>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOperationAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePoolArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_x64: u128,
        pub open_time: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateSupportMintAssociatedArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreaseLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_min: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_min: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreaseLiquidityV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_min: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_min: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_max: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseLiquidityV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_max: u64,
        pub base_flag: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeRewardArguments {
        pub param: InitializeRewardParam,
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
        pub amount_0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_max: u64,
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
        pub amount_0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_max: u64,
        pub with_metadata: bool,
        pub base_flag: bool,
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
        pub amount_0_max: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_1_max: u64,
        pub with_metadata: bool,
        pub base_flag: bool,
    }
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
    pub struct SwapRouterBaseInArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_out_minimum: u64,
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
    pub struct TransferRewardOwnerArguments {
        #[serde(with = "pubkey_serde")]
        pub new_owner: Pubkey,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateAmmConfigArguments {
        pub param: u8,
        pub value: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateOperationAccountArguments {
        pub param: u8,
        pub keys: Vec<Pubkey>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePoolStatusArguments {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardInfosArguments {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    ClosePosition {
        accounts: ClosePositionAccounts,
        args: ClosePositionArguments,
    },
    CollectFundFee {
        accounts: CollectFundFeeAccounts,
        args: CollectFundFeeArguments,
    },
    CollectProtocolFee {
        accounts: CollectProtocolFeeAccounts,
        args: CollectProtocolFeeArguments,
    },
    CollectRemainingRewards {
        accounts: CollectRemainingRewardsAccounts,
        args: CollectRemainingRewardsArguments,
    },
    CreateAmmConfig {
        accounts: CreateAmmConfigAccounts,
        args: CreateAmmConfigArguments,
    },
    CreateOperationAccount {
        accounts: CreateOperationAccountAccounts,
        args: CreateOperationAccountArguments,
    },
    CreatePool {
        accounts: CreatePoolAccounts,
        args: CreatePoolArguments,
    },
    CreateSupportMintAssociated {
        accounts: CreateSupportMintAssociatedAccounts,
        args: CreateSupportMintAssociatedArguments,
    },
    DecreaseLiquidity {
        accounts: DecreaseLiquidityAccounts,
        args: DecreaseLiquidityArguments,
    },
    DecreaseLiquidityV2 {
        accounts: DecreaseLiquidityV2Accounts,
        args: DecreaseLiquidityV2Arguments,
    },
    IncreaseLiquidity {
        accounts: IncreaseLiquidityAccounts,
        args: IncreaseLiquidityArguments,
    },
    IncreaseLiquidityV2 {
        accounts: IncreaseLiquidityV2Accounts,
        args: IncreaseLiquidityV2Arguments,
    },
    InitializeReward {
        accounts: InitializeRewardAccounts,
        args: InitializeRewardArguments,
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
    SetRewardParams {
        accounts: SetRewardParamsAccounts,
        args: SetRewardParamsArguments,
    },
    Swap {
        accounts: SwapAccounts,
        args: SwapArguments,
    },
    SwapRouterBaseIn {
        accounts: SwapRouterBaseInAccounts,
        args: SwapRouterBaseInArguments,
    },
    SwapV2 {
        accounts: SwapV2Accounts,
        args: SwapV2Arguments,
    },
    TransferRewardOwner {
        accounts: TransferRewardOwnerAccounts,
        args: TransferRewardOwnerArguments,
    },
    UpdateAmmConfig {
        accounts: UpdateAmmConfigAccounts,
        args: UpdateAmmConfigArguments,
    },
    UpdateOperationAccount {
        accounts: UpdateOperationAccountAccounts,
        args: UpdateOperationAccountArguments,
    },
    UpdatePoolStatus {
        accounts: UpdatePoolStatusAccounts,
        args: UpdatePoolStatusArguments,
    },
    UpdateRewardInfos {
        accounts: UpdateRewardInfosAccounts,
        args: UpdateRewardInfosArguments,
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
            [123u8, 134u8, 81u8, 0u8, 49u8, 68u8, 98u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 6usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        6usize
                    );
                }
                let mut required_iter = account_keys.iter().take(6usize);
                let nft_owner = required_iter.next().unwrap().clone();
                let position_nft_mint = required_iter.next().unwrap().clone();
                let position_nft_account = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(6usize);
                let remaining = if account_keys.len() > (6usize + 0usize) {
                    account_keys[(6usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = ClosePositionAccounts {
                    nft_owner,
                    position_nft_mint,
                    position_nft_account,
                    personal_position,
                    system_program,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::ClosePosition { accounts, args });
            }
            [167u8, 138u8, 78u8, 149u8, 223u8, 194u8, 6u8, 126u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectFundFeeArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 8usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        8usize
                    );
                }
                let mut required_iter = account_keys.iter().take(8usize);
                let owner = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let vault_0_mint = required_iter.next().unwrap().clone();
                let vault_1_mint = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(8usize);
                let recipient_token_account_0 = optional_iter.next().map(|s| s.clone());
                let recipient_token_account_1 = optional_iter.next().map(|s| s.clone());
                let token_program_2022 = optional_iter.next().map(|s| s.clone());
                let remaining = if account_keys.len() > (8usize + 3usize) {
                    account_keys[(8usize + 3usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CollectFundFeeAccounts {
                    owner,
                    pool_state,
                    amm_config,
                    token_vault_0,
                    token_vault_1,
                    vault_0_mint,
                    vault_1_mint,
                    recipient_token_account_0,
                    recipient_token_account_1,
                    token_program,
                    token_program_2022,
                    remaining,
                };
                return Ok(Instruction::CollectFundFee { accounts, args });
            }
            [136u8, 136u8, 252u8, 221u8, 194u8, 66u8, 126u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectProtocolFeeArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 8usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        8usize
                    );
                }
                let mut required_iter = account_keys.iter().take(8usize);
                let owner = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let vault_0_mint = required_iter.next().unwrap().clone();
                let vault_1_mint = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(8usize);
                let recipient_token_account_0 = optional_iter.next().map(|s| s.clone());
                let recipient_token_account_1 = optional_iter.next().map(|s| s.clone());
                let token_program_2022 = optional_iter.next().map(|s| s.clone());
                let remaining = if account_keys.len() > (8usize + 3usize) {
                    account_keys[(8usize + 3usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CollectProtocolFeeAccounts {
                    owner,
                    pool_state,
                    amm_config,
                    token_vault_0,
                    token_vault_1,
                    vault_0_mint,
                    vault_1_mint,
                    recipient_token_account_0,
                    recipient_token_account_1,
                    token_program,
                    token_program_2022,
                    remaining,
                };
                return Ok(Instruction::CollectProtocolFee { accounts, args });
            }
            [18u8, 237u8, 166u8, 197u8, 34u8, 16u8, 213u8, 144u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectRemainingRewardsArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 5usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        5usize
                    );
                }
                let mut required_iter = account_keys.iter().take(5usize);
                let reward_funder = required_iter.next().unwrap().clone();
                let funder_token_account = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let reward_token_vault = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(5usize);
                let reward_vault_mint = optional_iter.next().map(|s| s.clone());
                let token_program_2022 = optional_iter.next().map(|s| s.clone());
                let memo_program = optional_iter.next().map(|s| s.clone());
                let remaining = if account_keys.len() > (5usize + 3usize) {
                    account_keys[(5usize + 3usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CollectRemainingRewardsAccounts {
                    reward_funder,
                    funder_token_account,
                    pool_state,
                    reward_token_vault,
                    reward_vault_mint,
                    token_program,
                    token_program_2022,
                    memo_program,
                    remaining,
                };
                return Ok(Instruction::CollectRemainingRewards { accounts, args });
            }
            [137u8, 52u8, 237u8, 212u8, 215u8, 117u8, 108u8, 104u8] => {
                let args = if rest.len() >= 16 {
                    let mut rdr: &[u8] = rest;
                    let index = u16::deserialize(&mut rdr)?;
                    let tick_spacing = u16::deserialize(&mut rdr)?;
                    let trade_fee_rate = u32::deserialize(&mut rdr)?;
                    let protocol_fee_rate = u32::deserialize(&mut rdr)?;
                    let fund_fee_rate = u32::deserialize(&mut rdr)?;
                    CreateAmmConfigArguments {
                        index,
                        tick_spacing,
                        trade_fee_rate,
                        protocol_fee_rate,
                        fund_fee_rate: Some(fund_fee_rate),
                    }
                } else if rest.len() >= 12 {
                    let mut rdr: &[u8] = rest;
                    let index = u16::deserialize(&mut rdr)?;
                    let tick_spacing = u16::deserialize(&mut rdr)?;
                    let trade_fee_rate = u32::deserialize(&mut rdr)?;
                    let protocol_fee_rate = u32::deserialize(&mut rdr)?;
                    CreateAmmConfigArguments {
                        index,
                        tick_spacing,
                        trade_fee_rate,
                        protocol_fee_rate,
                        fund_fee_rate: None,
                    }
                } else {
                    anyhow::bail!(
                        "Insufficient data for create_amm_config: got {} bytes, need at least 12",
                        rest.len()
                    );
                };
                if account_keys.len() < 3usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        3usize
                    );
                }
                let mut required_iter = account_keys.iter().take(3usize);
                let owner = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(3usize);
                let remaining = if account_keys.len() > (3usize + 0usize) {
                    account_keys[(3usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CreateAmmConfigAccounts {
                    owner,
                    amm_config,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateAmmConfig { accounts, args });
            }
            [63u8, 87u8, 148u8, 33u8, 109u8, 35u8, 8u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateOperationAccountArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 3usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        3usize
                    );
                }
                let mut required_iter = account_keys.iter().take(3usize);
                let owner = required_iter.next().unwrap().clone();
                let operation_state = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(3usize);
                let remaining = if account_keys.len() > (3usize + 0usize) {
                    account_keys[(3usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CreateOperationAccountAccounts {
                    owner,
                    operation_state,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateOperationAccount { accounts, args });
            }
            [233u8, 146u8, 209u8, 142u8, 207u8, 104u8, 64u8, 188u8] => {
                let args = if rest.len() >= 24 {
                    let mut rdr: &[u8] = rest;
                    let sqrt_price_x64 = u128::deserialize(&mut rdr)?;
                    let open_time = u64::deserialize(&mut rdr)?;
                    CreatePoolArguments {
                        sqrt_price_x64,
                        open_time: Some(open_time),
                    }
                } else if rest.len() >= 16 {
                    let mut rdr: &[u8] = rest;
                    let sqrt_price_x64 = u128::deserialize(&mut rdr)?;
                    CreatePoolArguments {
                        sqrt_price_x64,
                        open_time: None,
                    }
                } else {
                    anyhow::bail!(
                        "Insufficient data for create_pool: got {} bytes, need at least 16",
                        rest.len()
                    );
                };
                if account_keys.len() < 11usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        11usize
                    );
                }
                let mut required_iter = account_keys.iter().take(11usize);
                let pool_creator = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let token_mint_0 = required_iter.next().unwrap().clone();
                let token_mint_1 = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let observation_state = required_iter.next().unwrap().clone();
                let token_program_0 = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let rent = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(11usize);
                let tick_array_bitmap = optional_iter.next().map(|s| s.clone());
                let token_program_1 = optional_iter.next().map(|s| s.clone());
                let remaining = if account_keys.len() > (11usize + 2usize) {
                    account_keys[(11usize + 2usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CreatePoolAccounts {
                    pool_creator,
                    amm_config,
                    pool_state,
                    token_mint_0,
                    token_mint_1,
                    token_vault_0,
                    token_vault_1,
                    observation_state,
                    tick_array_bitmap,
                    token_program_0,
                    token_program_1,
                    system_program,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreatePool { accounts, args });
            }
            [17u8, 251u8, 65u8, 92u8, 136u8, 242u8, 14u8, 169u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateSupportMintAssociatedArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 4usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        4usize
                    );
                }
                let mut required_iter = account_keys.iter().take(4usize);
                let owner = required_iter.next().unwrap().clone();
                let token_mint = required_iter.next().unwrap().clone();
                let support_mint_associated = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(4usize);
                let remaining = if account_keys.len() > (4usize + 0usize) {
                    account_keys[(4usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = CreateSupportMintAssociatedAccounts {
                    owner,
                    token_mint,
                    support_mint_associated,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateSupportMintAssociated { accounts, args });
            }
            [160u8, 38u8, 208u8, 111u8, 104u8, 91u8, 44u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecreaseLiquidityArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 12usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        12usize
                    );
                }
                let mut required_iter = account_keys.iter().take(12usize);
                let nft_owner = required_iter.next().unwrap().clone();
                let nft_account = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let recipient_token_account_0 = required_iter.next().unwrap().clone();
                let recipient_token_account_1 = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(12usize);
                let remaining = if account_keys.len() > (12usize + 0usize) {
                    account_keys[(12usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = DecreaseLiquidityAccounts {
                    nft_owner,
                    nft_account,
                    personal_position,
                    pool_state,
                    protocol_position,
                    token_vault_0,
                    token_vault_1,
                    tick_array_lower,
                    tick_array_upper,
                    recipient_token_account_0,
                    recipient_token_account_1,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::DecreaseLiquidity { accounts, args });
            }
            [58u8, 127u8, 188u8, 62u8, 79u8, 82u8, 196u8, 96u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecreaseLiquidityV2Arguments::deserialize(&mut rdr)?;
                if account_keys.len() < 16usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        16usize
                    );
                }
                let mut required_iter = account_keys.iter().take(16usize);
                let nft_owner = required_iter.next().unwrap().clone();
                let nft_account = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let recipient_token_account_0 = required_iter.next().unwrap().clone();
                let recipient_token_account_1 = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let memo_program = required_iter.next().unwrap().clone();
                let vault_0_mint = required_iter.next().unwrap().clone();
                let vault_1_mint = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(16usize);
                let remaining = if account_keys.len() > (16usize + 0usize) {
                    account_keys[(16usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = DecreaseLiquidityV2Accounts {
                    nft_owner,
                    nft_account,
                    personal_position,
                    pool_state,
                    protocol_position,
                    token_vault_0,
                    token_vault_1,
                    tick_array_lower,
                    tick_array_upper,
                    recipient_token_account_0,
                    recipient_token_account_1,
                    token_program,
                    token_program_2022,
                    memo_program,
                    vault_0_mint,
                    vault_1_mint,
                    remaining,
                };
                return Ok(Instruction::DecreaseLiquidityV2 { accounts, args });
            }
            [46u8, 156u8, 243u8, 118u8, 13u8, 205u8, 251u8, 178u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseLiquidityArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 12usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        12usize
                    );
                }
                let mut required_iter = account_keys.iter().take(12usize);
                let nft_owner = required_iter.next().unwrap().clone();
                let nft_account = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let token_account_0 = required_iter.next().unwrap().clone();
                let token_account_1 = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(12usize);
                let remaining = if account_keys.len() > (12usize + 0usize) {
                    account_keys[(12usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = IncreaseLiquidityAccounts {
                    nft_owner,
                    nft_account,
                    pool_state,
                    protocol_position,
                    personal_position,
                    tick_array_lower,
                    tick_array_upper,
                    token_account_0,
                    token_account_1,
                    token_vault_0,
                    token_vault_1,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::IncreaseLiquidity { accounts, args });
            }
            [133u8, 29u8, 89u8, 223u8, 69u8, 238u8, 176u8, 10u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseLiquidityV2Arguments::deserialize(&mut rdr)?;
                if account_keys.len() < 15usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        15usize
                    );
                }
                let mut required_iter = account_keys.iter().take(15usize);
                let nft_owner = required_iter.next().unwrap().clone();
                let nft_account = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let token_account_0 = required_iter.next().unwrap().clone();
                let token_account_1 = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let vault_0_mint = required_iter.next().unwrap().clone();
                let vault_1_mint = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(15usize);
                let remaining = if account_keys.len() > (15usize + 0usize) {
                    account_keys[(15usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = IncreaseLiquidityV2Accounts {
                    nft_owner,
                    nft_account,
                    pool_state,
                    protocol_position,
                    personal_position,
                    tick_array_lower,
                    tick_array_upper,
                    token_account_0,
                    token_account_1,
                    token_vault_0,
                    token_vault_1,
                    token_program,
                    token_program_2022,
                    vault_0_mint,
                    vault_1_mint,
                    remaining,
                };
                return Ok(Instruction::IncreaseLiquidityV2 { accounts, args });
            }
            [95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeRewardArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 9usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        9usize
                    );
                }
                let mut required_iter = account_keys.iter().take(9usize);
                let reward_funder = required_iter.next().unwrap().clone();
                let funder_token_account = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let operation_state = required_iter.next().unwrap().clone();
                let reward_token_mint = required_iter.next().unwrap().clone();
                let reward_token_program = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let rent = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(9usize);
                let reward_token_vault = optional_iter.next().map(|s| s.clone());
                let remaining = if account_keys.len() > (9usize + 1usize) {
                    account_keys[(9usize + 1usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = InitializeRewardAccounts {
                    reward_funder,
                    funder_token_account,
                    amm_config,
                    pool_state,
                    operation_state,
                    reward_token_mint,
                    reward_token_vault,
                    reward_token_program,
                    system_program,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeReward { accounts, args });
            }
            [135u8, 128u8, 47u8, 77u8, 15u8, 152u8, 240u8, 49u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenPositionArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 19usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        19usize
                    );
                }
                let mut required_iter = account_keys.iter().take(19usize);
                let payer = required_iter.next().unwrap().clone();
                let position_nft_owner = required_iter.next().unwrap().clone();
                let position_nft_mint = required_iter.next().unwrap().clone();
                let position_nft_account = required_iter.next().unwrap().clone();
                let metadata_account = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let token_account_0 = required_iter.next().unwrap().clone();
                let token_account_1 = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let rent = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let associated_token_program = required_iter.next().unwrap().clone();
                let metadata_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(19usize);
                let remaining = if account_keys.len() > (19usize + 0usize) {
                    account_keys[(19usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = OpenPositionAccounts {
                    payer,
                    position_nft_owner,
                    position_nft_mint,
                    position_nft_account,
                    metadata_account,
                    pool_state,
                    protocol_position,
                    tick_array_lower,
                    tick_array_upper,
                    personal_position,
                    token_account_0,
                    token_account_1,
                    token_vault_0,
                    token_vault_1,
                    rent,
                    system_program,
                    token_program,
                    associated_token_program,
                    metadata_program,
                    remaining,
                };
                return Ok(Instruction::OpenPosition { accounts, args });
            }
            [77u8, 184u8, 74u8, 214u8, 112u8, 86u8, 241u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenPositionV2Arguments::deserialize(&mut rdr)?;
                if account_keys.len() < 22usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        22usize
                    );
                }
                let mut required_iter = account_keys.iter().take(22usize);
                let payer = required_iter.next().unwrap().clone();
                let position_nft_owner = required_iter.next().unwrap().clone();
                let position_nft_mint = required_iter.next().unwrap().clone();
                let position_nft_account = required_iter.next().unwrap().clone();
                let metadata_account = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let token_account_0 = required_iter.next().unwrap().clone();
                let token_account_1 = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let rent = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let associated_token_program = required_iter.next().unwrap().clone();
                let metadata_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let vault_0_mint = required_iter.next().unwrap().clone();
                let vault_1_mint = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(22usize);
                let remaining = if account_keys.len() > (22usize + 0usize) {
                    account_keys[(22usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = OpenPositionV2Accounts {
                    payer,
                    position_nft_owner,
                    position_nft_mint,
                    position_nft_account,
                    metadata_account,
                    pool_state,
                    protocol_position,
                    tick_array_lower,
                    tick_array_upper,
                    personal_position,
                    token_account_0,
                    token_account_1,
                    token_vault_0,
                    token_vault_1,
                    rent,
                    system_program,
                    token_program,
                    associated_token_program,
                    metadata_program,
                    token_program_2022,
                    vault_0_mint,
                    vault_1_mint,
                    remaining,
                };
                return Ok(Instruction::OpenPositionV2 { accounts, args });
            }
            [77u8, 255u8, 174u8, 82u8, 125u8, 29u8, 201u8, 46u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenPositionWithToken22NftArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 20usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        20usize
                    );
                }
                let mut required_iter = account_keys.iter().take(20usize);
                let payer = required_iter.next().unwrap().clone();
                let position_nft_owner = required_iter.next().unwrap().clone();
                let position_nft_mint = required_iter.next().unwrap().clone();
                let position_nft_account = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let protocol_position = required_iter.next().unwrap().clone();
                let tick_array_lower = required_iter.next().unwrap().clone();
                let tick_array_upper = required_iter.next().unwrap().clone();
                let personal_position = required_iter.next().unwrap().clone();
                let token_account_0 = required_iter.next().unwrap().clone();
                let token_account_1 = required_iter.next().unwrap().clone();
                let token_vault_0 = required_iter.next().unwrap().clone();
                let token_vault_1 = required_iter.next().unwrap().clone();
                let rent = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let associated_token_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let vault_0_mint = required_iter.next().unwrap().clone();
                let vault_1_mint = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(20usize);
                let remaining = if account_keys.len() > (20usize + 0usize) {
                    account_keys[(20usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = OpenPositionWithToken22NftAccounts {
                    payer,
                    position_nft_owner,
                    position_nft_mint,
                    position_nft_account,
                    pool_state,
                    protocol_position,
                    tick_array_lower,
                    tick_array_upper,
                    personal_position,
                    token_account_0,
                    token_account_1,
                    token_vault_0,
                    token_vault_1,
                    rent,
                    system_program,
                    token_program,
                    associated_token_program,
                    token_program_2022,
                    vault_0_mint,
                    vault_1_mint,
                    remaining,
                };
                return Ok(Instruction::OpenPositionWithToken22Nft { accounts, args });
            }
            [112u8, 52u8, 167u8, 75u8, 32u8, 201u8, 211u8, 137u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetRewardParamsArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 6usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        6usize
                    );
                }
                let mut required_iter = account_keys.iter().take(6usize);
                let authority = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let operation_state = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(6usize);
                let remaining = if account_keys.len() > (6usize + 0usize) {
                    account_keys[(6usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = SetRewardParamsAccounts {
                    authority,
                    amm_config,
                    pool_state,
                    operation_state,
                    token_program,
                    token_program_2022,
                    remaining,
                };
                return Ok(Instruction::SetRewardParams { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 10usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        10usize
                    );
                }
                let mut required_iter = account_keys.iter().take(10usize);
                let payer = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let input_token_account = required_iter.next().unwrap().clone();
                let output_token_account = required_iter.next().unwrap().clone();
                let input_vault = required_iter.next().unwrap().clone();
                let output_vault = required_iter.next().unwrap().clone();
                let observation_state = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let tick_array = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(10usize);
                let remaining = if account_keys.len() > (10usize + 0usize) {
                    account_keys[(10usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = SwapAccounts {
                    payer,
                    amm_config,
                    pool_state,
                    input_token_account,
                    output_token_account,
                    input_vault,
                    output_vault,
                    observation_state,
                    token_program,
                    tick_array,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [69u8, 125u8, 115u8, 218u8, 245u8, 186u8, 242u8, 196u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapRouterBaseInArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 6usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        6usize
                    );
                }
                let mut required_iter = account_keys.iter().take(6usize);
                let payer = required_iter.next().unwrap().clone();
                let input_token_account = required_iter.next().unwrap().clone();
                let input_token_mint = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let memo_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(6usize);
                let remaining = if account_keys.len() > (6usize + 0usize) {
                    account_keys[(6usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = SwapRouterBaseInAccounts {
                    payer,
                    input_token_account,
                    input_token_mint,
                    token_program,
                    token_program_2022,
                    memo_program,
                    remaining,
                };
                return Ok(Instruction::SwapRouterBaseIn { accounts, args });
            }
            [43u8, 4u8, 237u8, 11u8, 26u8, 201u8, 30u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapV2Arguments::deserialize(&mut rdr)?;
                if account_keys.len() < 13usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        13usize
                    );
                }
                let mut required_iter = account_keys.iter().take(13usize);
                let payer = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let input_token_account = required_iter.next().unwrap().clone();
                let output_token_account = required_iter.next().unwrap().clone();
                let input_vault = required_iter.next().unwrap().clone();
                let output_vault = required_iter.next().unwrap().clone();
                let observation_state = required_iter.next().unwrap().clone();
                let token_program = required_iter.next().unwrap().clone();
                let token_program_2022 = required_iter.next().unwrap().clone();
                let memo_program = required_iter.next().unwrap().clone();
                let input_vault_mint = required_iter.next().unwrap().clone();
                let output_vault_mint = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(13usize);
                let remaining = if account_keys.len() > (13usize + 0usize) {
                    account_keys[(13usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = SwapV2Accounts {
                    payer,
                    amm_config,
                    pool_state,
                    input_token_account,
                    output_token_account,
                    input_vault,
                    output_vault,
                    observation_state,
                    token_program,
                    token_program_2022,
                    memo_program,
                    input_vault_mint,
                    output_vault_mint,
                    remaining,
                };
                return Ok(Instruction::SwapV2 { accounts, args });
            }
            [7u8, 22u8, 12u8, 83u8, 242u8, 43u8, 48u8, 121u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferRewardOwnerArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 2usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        2usize
                    );
                }
                let mut required_iter = account_keys.iter().take(2usize);
                let authority = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(2usize);
                let remaining = if account_keys.len() > (2usize + 0usize) {
                    account_keys[(2usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = TransferRewardOwnerAccounts {
                    authority,
                    pool_state,
                    remaining,
                };
                return Ok(Instruction::TransferRewardOwner { accounts, args });
            }
            [49u8, 60u8, 174u8, 136u8, 154u8, 28u8, 116u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateAmmConfigArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 2usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        2usize
                    );
                }
                let mut required_iter = account_keys.iter().take(2usize);
                let owner = required_iter.next().unwrap().clone();
                let amm_config = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(2usize);
                let remaining = if account_keys.len() > (2usize + 0usize) {
                    account_keys[(2usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = UpdateAmmConfigAccounts {
                    owner,
                    amm_config,
                    remaining,
                };
                return Ok(Instruction::UpdateAmmConfig { accounts, args });
            }
            [127u8, 70u8, 119u8, 40u8, 188u8, 227u8, 61u8, 7u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateOperationAccountArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 3usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        3usize
                    );
                }
                let mut required_iter = account_keys.iter().take(3usize);
                let owner = required_iter.next().unwrap().clone();
                let operation_state = required_iter.next().unwrap().clone();
                let system_program = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(3usize);
                let remaining = if account_keys.len() > (3usize + 0usize) {
                    account_keys[(3usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = UpdateOperationAccountAccounts {
                    owner,
                    operation_state,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::UpdateOperationAccount { accounts, args });
            }
            [130u8, 87u8, 108u8, 6u8, 46u8, 224u8, 117u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePoolStatusArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 2usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        2usize
                    );
                }
                let mut required_iter = account_keys.iter().take(2usize);
                let authority = required_iter.next().unwrap().clone();
                let pool_state = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(2usize);
                let remaining = if account_keys.len() > (2usize + 0usize) {
                    account_keys[(2usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = UpdatePoolStatusAccounts {
                    authority,
                    pool_state,
                    remaining,
                };
                return Ok(Instruction::UpdatePoolStatus { accounts, args });
            }
            [163u8, 172u8, 224u8, 52u8, 11u8, 154u8, 106u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateRewardInfosArguments::deserialize(&mut rdr)?;
                if account_keys.len() < 1usize {
                    anyhow::bail!(
                        "Insufficient accounts: got {}, need at least {} for required accounts",
                        account_keys.len(),
                        1usize
                    );
                }
                let mut required_iter = account_keys.iter().take(1usize);
                let pool_state = required_iter.next().unwrap().clone();
                let mut optional_iter = account_keys.iter().skip(1usize);
                let remaining = if account_keys.len() > (1usize + 0usize) {
                    account_keys[(1usize + 0usize)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = UpdateRewardInfosAccounts {
                    pool_state,
                    remaining,
                };
                return Ok(Instruction::UpdateRewardInfos { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
