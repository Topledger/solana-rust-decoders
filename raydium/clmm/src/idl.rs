extern crate serde;
pub use accounts_data::*;
use anchor_lang::prelude::*;
#[allow(dead_code)]
use borsh::BorshDeserialize;
pub use ix_data::*;
use serde::Serialize;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AmmConfig {
        pub bump: u8,
        pub index: u16,
        pub owner: Pubkey,
        pub protocol_fee_rate: u32,
        pub trade_fee_rate: u32,
        pub tick_spacing: u16,
        pub fund_fee_rate: u32,
        pub padding_u32: u32,
        pub fund_owner: Pubkey,
        pub padding: [u64; 3],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CollectPersonalFeeEvent {
        pub position_nft_mint: Pubkey,
        pub recipient_token_account_0: Pubkey,
        pub recipient_token_account_1: Pubkey,
        pub amount_0: u64,
        pub amount_1: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CollectProtocolFeeEvent {
        pub pool_state: Pubkey,
        pub recipient_token_account_0: Pubkey,
        pub recipient_token_account_1: Pubkey,
        pub amount_0: u64,
        pub amount_1: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ConfigChangeEvent {
        pub index: u16,
        pub owner: Pubkey,
        pub protocol_fee_rate: u32,
        pub trade_fee_rate: u32,
        pub tick_spacing: u16,
        pub fund_fee_rate: u32,
        pub fund_owner: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CreatePersonalPositionEvent {
        pub pool_state: Pubkey,
        pub minter: Pubkey,
        pub nft_owner: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub liquidity: u128,
        pub deposit_amount_0: u64,
        pub deposit_amount_1: u64,
        pub deposit_amount_0_transfer_fee: u64,
        pub deposit_amount_1_transfer_fee: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct DecreaseLiquidityEvent {
        pub position_nft_mint: Pubkey,
        pub liquidity: u128,
        pub decrease_amount_0: u64,
        pub decrease_amount_1: u64,
        pub fee_amount_0: u64,
        pub fee_amount_1: u64,
        pub reward_amounts: [u64; 3],
        pub transfer_fee_0: u64,
        pub transfer_fee_1: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct IncreaseLiquidityEvent {
        pub position_nft_mint: Pubkey,
        pub liquidity: u128,
        pub amount_0: u64,
        pub amount_1: u64,
        pub amount_0_transfer_fee: u64,
        pub amount_1_transfer_fee: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InitializeRewardParam {
        pub open_time: u64,
        pub end_time: u64,
        pub emissions_per_second_x64: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LiquidityCalculateEvent {
        pub pool_liquidity: u128,
        pub pool_sqrt_price_x64: u128,
        pub pool_tick: i32,
        pub calc_amount_0: u64,
        pub calc_amount_1: u64,
        pub trade_fee_owed_0: u64,
        pub trade_fee_owed_1: u64,
        pub transfer_fee_0: u64,
        pub transfer_fee_1: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LiquidityChangeEvent {
        pub pool_state: Pubkey,
        pub tick: i32,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub liquidity_before: u128,
        pub liquidity_after: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Observation {
        pub block_timestamp: u32,
        pub tick_cumulative: i64,
        pub padding: [u64; 4],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ObservationState {
        pub initialized: bool,
        pub recent_epoch: u64,
        pub observation_index: u16,
        pub pool_id: Pubkey,
        pub observations: [Observation; 100],
        pub padding: [u64; 4],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct OperationState {
        pub bump: u8,
        pub operation_owners: [Pubkey; 10],
        pub whitelist_mints: [Pubkey; 100],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PersonalPositionState {
        pub bump: [u8; 1],
        pub nft_mint: Pubkey,
        pub pool_id: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x64: u128,
        pub fee_growth_inside_1_last_x64: u128,
        pub token_fees_owed_0: u64,
        pub token_fees_owed_1: u64,
        pub reward_infos: [PositionRewardInfo; 3],
        pub recent_epoch: u64,
        pub padding: [u64; 7],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PoolCreatedEvent {
        pub token_mint_0: Pubkey,
        pub token_mint_1: Pubkey,
        pub tick_spacing: u16,
        pub pool_state: Pubkey,
        pub sqrt_price_x64: u128,
        pub tick: i32,
        pub token_vault_0: Pubkey,
        pub token_vault_1: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PoolState {
        pub bump: [u8; 1],
        pub amm_config: Pubkey,
        pub owner: Pubkey,
        pub token_mint_0: Pubkey,
        pub token_mint_1: Pubkey,
        pub token_vault_0: Pubkey,
        pub token_vault_1: Pubkey,
        pub observation_key: Pubkey,
        pub mint_decimals_0: u8,
        pub mint_decimals_1: u8,
        pub tick_spacing: u16,
        pub liquidity: u128,
        pub sqrt_price_x64: u128,
        pub tick_current: i32,
        pub padding3: u16,
        pub padding4: u16,
        pub fee_growth_global_0_x64: u128,
        pub fee_growth_global_1_x64: u128,
        pub protocol_fees_token_0: u64,
        pub protocol_fees_token_1: u64,
        pub swap_in_amount_token_0: u128,
        pub swap_out_amount_token_1: u128,
        pub swap_in_amount_token_1: u128,
        pub swap_out_amount_token_0: u128,
        pub status: u8,
        pub padding: [u8; 7],
        pub reward_infos: [RewardInfo; 3],
        pub tick_array_bitmap: [u64; 16],
        pub total_fees_token_0: u64,
        pub total_fees_claimed_token_0: u64,
        pub total_fees_token_1: u64,
        pub total_fees_claimed_token_1: u64,
        pub fund_fees_token_0: u64,
        pub fund_fees_token_1: u64,
        pub open_time: u64,
        pub recent_epoch: u64,
        pub padding1: [u64; 24],
        pub padding2: [u64; 32],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PositionRewardInfo {
        pub growth_inside_last_x64: u128,
        pub reward_amount_owed: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ProtocolPositionState {
        pub bump: u8,
        pub pool_id: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x64: u128,
        pub fee_growth_inside_1_last_x64: u128,
        pub token_fees_owed_0: u64,
        pub token_fees_owed_1: u64,
        pub reward_growth_inside: [u128; 3],
        pub recent_epoch: u64,
        pub padding: [u64; 7],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RewardInfo {
        pub reward_state: u8,
        pub open_time: u64,
        pub end_time: u64,
        pub last_update_time: u64,
        pub emissions_per_second_x64: u128,
        pub reward_total_emissioned: u64,
        pub reward_claimed: u64,
        pub token_mint: Pubkey,
        pub token_vault: Pubkey,
        pub authority: Pubkey,
        pub reward_growth_global_x64: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SupportMintAssociated {
        pub bump: u8,
        pub mint: Pubkey,
        pub padding: [u64; 8],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        pub pool_state: Pubkey,
        pub sender: Pubkey,
        pub token_account_0: Pubkey,
        pub token_account_1: Pubkey,
        pub amount_0: u64,
        pub transfer_fee_0: u64,
        pub amount_1: u64,
        pub transfer_fee_1: u64,
        pub zero_for_one: bool,
        pub sqrt_price_x64: u128,
        pub liquidity: u128,
        pub tick: i32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TickArrayBitmapExtension {
        pub pool_id: Pubkey,
        pub positive_tick_array_bitmap: [[u64; 8]; 14],
        pub negative_tick_array_bitmap: [[u64; 8]; 14],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct TickArrayState {
        pub pool_id: Pubkey,
        pub start_tick_index: i32,
        pub ticks: [TickState; 60],
        pub initialized_tick_count: u8,
        pub recent_epoch: u64,
        pub padding: [u8; 107],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TickState {
        pub tick: i32,
        pub liquidity_net: i128,
        pub liquidity_gross: u128,
        pub fee_growth_outside_0_x64: u128,
        pub fee_growth_outside_1_x64: u128,
        pub reward_growths_outside_x64: [u128; 3],
        pub padding: [u32; 13],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
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
        pub recipient_token_account_0: String,
        pub recipient_token_account_1: String,
        pub token_program: String,
        pub token_program_2022: String,
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
        pub recipient_token_account_0: String,
        pub recipient_token_account_1: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectRemainingRewardsAccounts {
        pub reward_funder: String,
        pub funder_token_account: String,
        pub pool_state: String,
        pub reward_token_vault: String,
        pub reward_vault_mint: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub memo_program: String,
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
        pub tick_array_bitmap: String,
        pub token_program_0: String,
        pub token_program_1: String,
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
        pub reward_token_vault: String,
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
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct ClosePositionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CollectFundFeeArguments {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CollectProtocolFeeArguments {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CollectRemainingRewardsArguments {
        pub reward_index: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CreateAmmConfigArguments {
        pub index: u16,
        pub tick_spacing: u16,
        pub trade_fee_rate: u32,
        pub protocol_fee_rate: u32,
        pub fund_fee_rate: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CreateOperationAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CreatePoolArguments {
        pub sqrt_price_x64: u128,
        pub open_time: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CreateSupportMintAssociatedArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct DecreaseLiquidityArguments {
        pub liquidity: u128,
        pub amount_0_min: u64,
        pub amount_1_min: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct DecreaseLiquidityV2Arguments {
        pub liquidity: u128,
        pub amount_0_min: u64,
        pub amount_1_min: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct IncreaseLiquidityArguments {
        pub liquidity: u128,
        pub amount_0_max: u64,
        pub amount_1_max: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct IncreaseLiquidityV2Arguments {
        pub liquidity: u128,
        pub amount_0_max: u64,
        pub amount_1_max: u64,
        pub base_flag: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeRewardArguments {
        pub param: InitializeRewardParam,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct OpenPositionArguments {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        pub liquidity: u128,
        pub amount_0_max: u64,
        pub amount_1_max: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct OpenPositionV2Arguments {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        pub liquidity: u128,
        pub amount_0_max: u64,
        pub amount_1_max: u64,
        pub with_metadata: bool,
        pub base_flag: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct OpenPositionWithToken22NftArguments {
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub tick_array_lower_start_index: i32,
        pub tick_array_upper_start_index: i32,
        pub liquidity: u128,
        pub amount_0_max: u64,
        pub amount_1_max: u64,
        pub with_metadata: bool,
        pub base_flag: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SetRewardParamsArguments {
        pub reward_index: u8,
        pub emissions_per_second_x64: u128,
        pub open_time: u64,
        pub end_time: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapArguments {
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub sqrt_price_limit_x64: u128,
        pub is_base_input: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapRouterBaseInArguments {
        pub amount_in: u64,
        pub amount_out_minimum: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapV2Arguments {
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub sqrt_price_limit_x64: u128,
        pub is_base_input: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct TransferRewardOwnerArguments {
        pub new_owner: Pubkey,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateAmmConfigArguments {
        pub param: u8,
        pub value: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateOperationAccountArguments {
        pub param: u8,
        pub keys: Vec<Pubkey>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdatePoolStatusArguments {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateRewardInfosArguments {}
}
#[derive(Debug)]
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
                let mut keys = account_keys.iter();
                let nft_owner = keys.next().unwrap().clone();
                let position_nft_mint = keys.next().unwrap().clone();
                let position_nft_account = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let recipient_token_account_0 = keys.next().unwrap().clone();
                let recipient_token_account_1 = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let recipient_token_account_0 = keys.next().unwrap().clone();
                let recipient_token_account_1 = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let reward_funder = keys.next().unwrap().clone();
                let funder_token_account = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let reward_token_vault = keys.next().unwrap().clone();
                let reward_vault_mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let memo_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut rdr: &[u8] = rest;
                let args = CreateAmmConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let operation_state = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateOperationAccountAccounts {
                    owner,
                    operation_state,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateOperationAccount { accounts, args });
            }
            [233u8, 146u8, 209u8, 142u8, 207u8, 104u8, 64u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreatePoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool_creator = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let token_mint_0 = keys.next().unwrap().clone();
                let token_mint_1 = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let observation_state = keys.next().unwrap().clone();
                let tick_array_bitmap = keys.next().unwrap().clone();
                let token_program_0 = keys.next().unwrap().clone();
                let token_program_1 = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let token_mint = keys.next().unwrap().clone();
                let support_mint_associated = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let nft_owner = keys.next().unwrap().clone();
                let nft_account = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let recipient_token_account_0 = keys.next().unwrap().clone();
                let recipient_token_account_1 = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let nft_owner = keys.next().unwrap().clone();
                let nft_account = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let recipient_token_account_0 = keys.next().unwrap().clone();
                let recipient_token_account_1 = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let memo_program = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let nft_owner = keys.next().unwrap().clone();
                let nft_account = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let token_account_0 = keys.next().unwrap().clone();
                let token_account_1 = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let nft_owner = keys.next().unwrap().clone();
                let nft_account = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let token_account_0 = keys.next().unwrap().clone();
                let token_account_1 = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let reward_funder = keys.next().unwrap().clone();
                let funder_token_account = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let operation_state = keys.next().unwrap().clone();
                let reward_token_mint = keys.next().unwrap().clone();
                let reward_token_vault = keys.next().unwrap().clone();
                let reward_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let position_nft_owner = keys.next().unwrap().clone();
                let position_nft_mint = keys.next().unwrap().clone();
                let position_nft_account = keys.next().unwrap().clone();
                let metadata_account = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let token_account_0 = keys.next().unwrap().clone();
                let token_account_1 = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let metadata_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let position_nft_owner = keys.next().unwrap().clone();
                let position_nft_mint = keys.next().unwrap().clone();
                let position_nft_account = keys.next().unwrap().clone();
                let metadata_account = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let token_account_0 = keys.next().unwrap().clone();
                let token_account_1 = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let metadata_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let position_nft_owner = keys.next().unwrap().clone();
                let position_nft_mint = keys.next().unwrap().clone();
                let position_nft_account = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let protocol_position = keys.next().unwrap().clone();
                let tick_array_lower = keys.next().unwrap().clone();
                let tick_array_upper = keys.next().unwrap().clone();
                let personal_position = keys.next().unwrap().clone();
                let token_account_0 = keys.next().unwrap().clone();
                let token_account_1 = keys.next().unwrap().clone();
                let token_vault_0 = keys.next().unwrap().clone();
                let token_vault_1 = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let operation_state = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let input_token_account = keys.next().unwrap().clone();
                let output_token_account = keys.next().unwrap().clone();
                let input_vault = keys.next().unwrap().clone();
                let output_vault = keys.next().unwrap().clone();
                let observation_state = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let tick_array = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let input_token_account = keys.next().unwrap().clone();
                let input_token_mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let memo_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let input_token_account = keys.next().unwrap().clone();
                let output_token_account = keys.next().unwrap().clone();
                let input_vault = keys.next().unwrap().clone();
                let output_vault = keys.next().unwrap().clone();
                let observation_state = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let memo_program = keys.next().unwrap().clone();
                let input_vault_mint = keys.next().unwrap().clone();
                let output_vault_mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let operation_state = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
                let mut keys = account_keys.iter();
                let pool_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
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
pub mod events {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {}
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        pub fn decode(_data: &[u8]) -> anyhow::Result<Self> {
            anyhow::bail!("Event decoding not implemented for CLMM")
        }
    }
}
