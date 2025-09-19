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
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AdminSetCoinCreatorEvent {
        pub timestamp: i64,
        pub admin_set_coin_creator_authority: [u8; 32usize],
        pub base_mint: [u8; 32usize],
        pub pool: [u8; 32usize],
        pub old_coin_creator: [u8; 32usize],
        pub new_coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AdminUpdateTokenIncentivesEvent {
        pub start_time: i64,
        pub end_time: i64,
        pub day_number: u64,
        pub token_supply_per_day: u64,
        pub mint: [u8; 32usize],
        pub seconds_in_a_day: i64,
        pub timestamp: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BondingCurve {
        pub virtual_token_reserves: u64,
        pub virtual_sol_reserves: u64,
        pub real_token_reserves: u64,
        pub real_sol_reserves: u64,
        pub token_total_supply: u64,
        pub complete: bool,
        pub creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BuyEvent {
        pub timestamp: i64,
        pub base_amount_out: u64,
        pub max_quote_amount_in: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub quote_amount_in: u64,
        pub lp_fee_basis_points: u64,
        pub lp_fee: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee: u64,
        pub quote_amount_in_with_lp_fee: u64,
        pub user_quote_amount_in: u64,
        pub pool: [u8; 32usize],
        pub user: [u8; 32usize],
        pub user_base_token_account: [u8; 32usize],
        pub user_quote_token_account: [u8; 32usize],
        pub protocol_fee_recipient: [u8; 32usize],
        pub protocol_fee_recipient_token_account: [u8; 32usize],
        pub coin_creator: [u8; 32usize],
        pub coin_creator_fee_basis_points: u64,
        pub coin_creator_fee: u64,
        pub track_volume: bool,
        pub total_unclaimed_tokens: u64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
        pub last_update_timestamp: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ClaimTokenIncentivesEvent {
        pub user: [u8; 32usize],
        pub mint: [u8; 32usize],
        pub amount: u64,
        pub timestamp: i64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CloseUserVolumeAccumulatorEvent {
        pub user: [u8; 32usize],
        pub timestamp: i64,
        pub total_unclaimed_tokens: u64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
        pub last_update_timestamp: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CollectCoinCreatorFeeEvent {
        pub timestamp: i64,
        pub coin_creator: [u8; 32usize],
        pub coin_creator_fee: u64,
        pub coin_creator_vault_ata: [u8; 32usize],
        pub coin_creator_token_account: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateConfigEvent {
        pub timestamp: i64,
        pub admin: [u8; 32usize],
        pub lp_fee_basis_points: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        pub coin_creator_fee_basis_points: u64,
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreatePoolEvent {
        pub timestamp: i64,
        pub index: u16,
        pub creator: [u8; 32usize],
        pub base_mint: [u8; 32usize],
        pub quote_mint: [u8; 32usize],
        pub base_mint_decimals: u8,
        pub quote_mint_decimals: u8,
        pub base_amount_in: u64,
        pub quote_amount_in: u64,
        pub pool_base_amount: u64,
        pub pool_quote_amount: u64,
        pub minimum_liquidity: u64,
        pub initial_liquidity: u64,
        pub lp_token_amount_out: u64,
        pub pool_bump: u8,
        pub pool: [u8; 32usize],
        pub lp_mint: [u8; 32usize],
        pub user_base_token_account: [u8; 32usize],
        pub user_quote_token_account: [u8; 32usize],
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DepositEvent {
        pub timestamp: i64,
        pub lp_token_amount_out: u64,
        pub max_base_amount_in: u64,
        pub max_quote_amount_in: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub base_amount_in: u64,
        pub quote_amount_in: u64,
        pub lp_mint_supply: u64,
        pub pool: [u8; 32usize],
        pub user: [u8; 32usize],
        pub user_base_token_account: [u8; 32usize],
        pub user_quote_token_account: [u8; 32usize],
        pub user_pool_token_account: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DisableEvent {
        pub timestamp: i64,
        pub admin: [u8; 32usize],
        pub disable_create_pool: bool,
        pub disable_deposit: bool,
        pub disable_withdraw: bool,
        pub disable_buy: bool,
        pub disable_sell: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ExtendAccountEvent {
        pub timestamp: i64,
        pub account: [u8; 32usize],
        pub user: [u8; 32usize],
        pub current_size: u64,
        pub new_size: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeConfig {
        pub bump: u8,
        pub admin: [u8; 32usize],
        pub flat_fees: Fees,
        pub fee_tiers: Vec<FeeTier>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeTier {
        pub market_cap_lamports_threshold: u128,
        pub fees: Fees,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Fees {
        pub lp_fee_bps: u64,
        pub protocol_fee_bps: u64,
        pub creator_fee_bps: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct GlobalConfig {
        pub admin: [u8; 32usize],
        pub lp_fee_basis_points: u64,
        pub protocol_fee_basis_points: u64,
        pub disable_flags: u8,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        pub coin_creator_fee_basis_points: u64,
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct GlobalVolumeAccumulator {
        pub start_time: i64,
        pub end_time: i64,
        pub seconds_in_a_day: i64,
        pub mint: [u8; 32usize],
        pub total_token_supply: [u64; 30usize],
        pub sol_volumes: [u64; 30usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitUserVolumeAccumulatorEvent {
        pub payer: [u8; 32usize],
        pub user: [u8; 32usize],
        pub timestamp: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OptionBool(bool);
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Pool {
        pub pool_bump: u8,
        pub index: u16,
        pub creator: [u8; 32usize],
        pub base_mint: [u8; 32usize],
        pub quote_mint: [u8; 32usize],
        pub lp_mint: [u8; 32usize],
        pub pool_base_token_account: [u8; 32usize],
        pub pool_quote_token_account: [u8; 32usize],
        pub lp_supply: u64,
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SellEvent {
        pub timestamp: i64,
        pub base_amount_in: u64,
        pub min_quote_amount_out: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub quote_amount_out: u64,
        pub lp_fee_basis_points: u64,
        pub lp_fee: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee: u64,
        pub quote_amount_out_without_lp_fee: u64,
        pub user_quote_amount_out: u64,
        pub pool: [u8; 32usize],
        pub user: [u8; 32usize],
        pub user_base_token_account: [u8; 32usize],
        pub user_quote_token_account: [u8; 32usize],
        pub protocol_fee_recipient: [u8; 32usize],
        pub protocol_fee_recipient_token_account: [u8; 32usize],
        pub coin_creator: [u8; 32usize],
        pub coin_creator_fee_basis_points: u64,
        pub coin_creator_fee: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetBondingCurveCoinCreatorEvent {
        pub timestamp: i64,
        pub base_mint: [u8; 32usize],
        pub pool: [u8; 32usize],
        pub bonding_curve: [u8; 32usize],
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetMetaplexCoinCreatorEvent {
        pub timestamp: i64,
        pub base_mint: [u8; 32usize],
        pub pool: [u8; 32usize],
        pub metadata: [u8; 32usize],
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SyncUserVolumeAccumulatorEvent {
        pub user: [u8; 32usize],
        pub total_claimed_tokens_before: u64,
        pub total_claimed_tokens_after: u64,
        pub timestamp: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateAdminEvent {
        pub timestamp: i64,
        pub admin: [u8; 32usize],
        pub new_admin: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateFeeConfigEvent {
        pub timestamp: i64,
        pub admin: [u8; 32usize],
        pub lp_fee_basis_points: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        pub coin_creator_fee_basis_points: u64,
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UserVolumeAccumulator {
        pub user: [u8; 32usize],
        pub needs_claim: bool,
        pub total_unclaimed_tokens: u64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
        pub last_update_timestamp: i64,
        pub has_total_claimed_tokens: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawEvent {
        pub timestamp: i64,
        pub lp_token_amount_in: u64,
        pub min_base_amount_out: u64,
        pub min_quote_amount_out: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub base_amount_out: u64,
        pub quote_amount_out: u64,
        pub lp_mint_supply: u64,
        pub pool: [u8; 32usize],
        pub user: [u8; 32usize],
        pub user_base_token_account: [u8; 32usize],
        pub user_quote_token_account: [u8; 32usize],
        pub user_pool_token_account: [u8; 32usize],
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct AdminSetCoinCreatorAccounts {
        pub admin_set_coin_creator_authority: String,
        pub global_config: String,
        pub pool: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AdminUpdateTokenIncentivesAccounts {
        pub admin: String,
        pub global_config: String,
        pub global_volume_accumulator: String,
        pub mint: String,
        pub global_incentive_token_account: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BuyAccounts {
        pub pool: String,
        pub user: String,
        pub global_config: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub user_base_token_account: String,
        pub user_quote_token_account: String,
        pub pool_base_token_account: String,
        pub pool_quote_token_account: String,
        pub protocol_fee_recipient: String,
        pub protocol_fee_recipient_token_account: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub coin_creator_vault_ata: String,
        pub coin_creator_vault_authority: String,
        pub global_volume_accumulator: String,
        pub user_volume_accumulator: String,
        pub fee_config: String,
        pub fee_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimTokenIncentivesAccounts {
        pub user: String,
        pub user_ata: String,
        pub global_volume_accumulator: String,
        pub global_incentive_token_account: String,
        pub user_volume_accumulator: String,
        pub mint: String,
        pub token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseUserVolumeAccumulatorAccounts {
        pub user: String,
        pub user_volume_accumulator: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectCoinCreatorFeeAccounts {
        pub quote_mint: String,
        pub quote_token_program: String,
        pub coin_creator: String,
        pub coin_creator_vault_authority: String,
        pub coin_creator_vault_ata: String,
        pub coin_creator_token_account: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateConfigAccounts {
        pub admin: String,
        pub global_config: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreatePoolAccounts {
        pub pool: String,
        pub global_config: String,
        pub creator: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub lp_mint: String,
        pub user_base_token_account: String,
        pub user_quote_token_account: String,
        pub user_pool_token_account: String,
        pub pool_base_token_account: String,
        pub pool_quote_token_account: String,
        pub system_program: String,
        pub token_2022_program: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub associated_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub pool: String,
        pub global_config: String,
        pub user: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub lp_mint: String,
        pub user_base_token_account: String,
        pub user_quote_token_account: String,
        pub user_pool_token_account: String,
        pub pool_base_token_account: String,
        pub pool_quote_token_account: String,
        pub token_program: String,
        pub token_2022_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DisableAccounts {
        pub admin: String,
        pub global_config: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ExtendAccountAccounts {
        pub account: String,
        pub user: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitUserVolumeAccumulatorAccounts {
        pub payer: String,
        pub user: String,
        pub user_volume_accumulator: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SellAccounts {
        pub pool: String,
        pub user: String,
        pub global_config: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub user_base_token_account: String,
        pub user_quote_token_account: String,
        pub pool_base_token_account: String,
        pub pool_quote_token_account: String,
        pub protocol_fee_recipient: String,
        pub protocol_fee_recipient_token_account: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub coin_creator_vault_ata: String,
        pub coin_creator_vault_authority: String,
        pub fee_config: String,
        pub fee_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetCoinCreatorAccounts {
        pub pool: String,
        pub metadata: String,
        pub bonding_curve: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SyncUserVolumeAccumulatorAccounts {
        pub user: String,
        pub global_volume_accumulator: String,
        pub user_volume_accumulator: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAdminAccounts {
        pub admin: String,
        pub global_config: String,
        pub new_admin: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFeeConfigAccounts {
        pub admin: String,
        pub global_config: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub pool: String,
        pub global_config: String,
        pub user: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub lp_mint: String,
        pub user_base_token_account: String,
        pub user_quote_token_account: String,
        pub user_pool_token_account: String,
        pub pool_base_token_account: String,
        pub pool_quote_token_account: String,
        pub token_program: String,
        pub token_2022_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AdminSetCoinCreatorArguments {
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AdminUpdateTokenIncentivesArguments {
        pub start_time: i64,
        pub end_time: i64,
        pub seconds_in_a_day: i64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub day_number: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub token_supply_per_day: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BuyArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub base_amount_out: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub max_quote_amount_in: u64,
        pub track_volume: OptionBool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimTokenIncentivesArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseUserVolumeAccumulatorArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectCoinCreatorFeeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateConfigArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub lp_fee_basis_points: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub protocol_fee_basis_points: u64,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        #[serde(serialize_with = "super::serialize_to_string")]
        pub coin_creator_fee_basis_points: u64,
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePoolArguments {
        pub index: u16,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub base_amount_in: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub quote_amount_in: u64,
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub lp_token_amount_out: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub max_base_amount_in: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub max_quote_amount_in: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DisableArguments {
        pub disable_create_pool: bool,
        pub disable_deposit: bool,
        pub disable_withdraw: bool,
        pub disable_buy: bool,
        pub disable_sell: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExtendAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitUserVolumeAccumulatorArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SellArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub base_amount_in: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub min_quote_amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetCoinCreatorArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SyncUserVolumeAccumulatorArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateAdminArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFeeConfigArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub lp_fee_basis_points: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub protocol_fee_basis_points: u64,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        #[serde(serialize_with = "super::serialize_to_string")]
        pub coin_creator_fee_basis_points: u64,
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub lp_token_amount_in: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub min_base_amount_out: u64,
        #[serde(serialize_with = "super::serialize_to_string")]
        pub min_quote_amount_out: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    AdminSetCoinCreator {
        accounts: AdminSetCoinCreatorAccounts,
        args: AdminSetCoinCreatorArguments,
    },
    AdminUpdateTokenIncentives {
        accounts: AdminUpdateTokenIncentivesAccounts,
        args: AdminUpdateTokenIncentivesArguments,
    },
    Buy {
        accounts: BuyAccounts,
        args: BuyArguments,
    },
    ClaimTokenIncentives {
        accounts: ClaimTokenIncentivesAccounts,
        args: ClaimTokenIncentivesArguments,
    },
    CloseUserVolumeAccumulator {
        accounts: CloseUserVolumeAccumulatorAccounts,
        args: CloseUserVolumeAccumulatorArguments,
    },
    CollectCoinCreatorFee {
        accounts: CollectCoinCreatorFeeAccounts,
        args: CollectCoinCreatorFeeArguments,
    },
    CreateConfig {
        accounts: CreateConfigAccounts,
        args: CreateConfigArguments,
    },
    CreatePool {
        accounts: CreatePoolAccounts,
        args: CreatePoolArguments,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArguments,
    },
    Disable {
        accounts: DisableAccounts,
        args: DisableArguments,
    },
    ExtendAccount {
        accounts: ExtendAccountAccounts,
        args: ExtendAccountArguments,
    },
    InitUserVolumeAccumulator {
        accounts: InitUserVolumeAccumulatorAccounts,
        args: InitUserVolumeAccumulatorArguments,
    },
    Sell {
        accounts: SellAccounts,
        args: SellArguments,
    },
    SetCoinCreator {
        accounts: SetCoinCreatorAccounts,
        args: SetCoinCreatorArguments,
    },
    SyncUserVolumeAccumulator {
        accounts: SyncUserVolumeAccumulatorAccounts,
        args: SyncUserVolumeAccumulatorArguments,
    },
    UpdateAdmin {
        accounts: UpdateAdminAccounts,
        args: UpdateAdminArguments,
    },
    UpdateFeeConfig {
        accounts: UpdateFeeConfigAccounts,
        args: UpdateFeeConfigArguments,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArguments,
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
            [242u8, 40u8, 117u8, 145u8, 73u8, 96u8, 105u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = AdminSetCoinCreatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin_set_coin_creator_authority = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AdminSetCoinCreatorAccounts {
                    admin_set_coin_creator_authority,
                    global_config,
                    pool,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AdminSetCoinCreator { accounts, args });
            }
            [209u8, 11u8, 115u8, 87u8, 213u8, 23u8, 124u8, 204u8] => {
                let mut rdr: &[u8] = rest;
                let args = AdminUpdateTokenIncentivesArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let global_volume_accumulator = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let global_incentive_token_account = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AdminUpdateTokenIncentivesAccounts {
                    admin,
                    global_config,
                    global_volume_accumulator,
                    mint,
                    global_incentive_token_account,
                    associated_token_program,
                    system_program,
                    token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AdminUpdateTokenIncentives { accounts, args });
            }
            [102u8, 6u8, 61u8, 18u8, 1u8, 218u8, 235u8, 234u8] => {
                let mut rdr: &[u8] = rest;
                let args = BuyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let base_mint = keys.next().unwrap().clone();
                let quote_mint = keys.next().unwrap().clone();
                let user_base_token_account = keys.next().unwrap().clone();
                let user_quote_token_account = keys.next().unwrap().clone();
                let pool_base_token_account = keys.next().unwrap().clone();
                let pool_quote_token_account = keys.next().unwrap().clone();
                let protocol_fee_recipient = keys.next().unwrap().clone();
                let protocol_fee_recipient_token_account = keys.next().unwrap().clone();
                let base_token_program = keys.next().unwrap().clone();
                let quote_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let coin_creator_vault_ata = keys.next().unwrap().clone();
                let coin_creator_vault_authority = keys.next().unwrap().clone();
                let global_volume_accumulator = keys.next().unwrap().clone();
                let user_volume_accumulator = keys.next().unwrap().clone();
                let fee_config = keys.next().unwrap().clone();
                let fee_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BuyAccounts {
                    pool,
                    user,
                    global_config,
                    base_mint,
                    quote_mint,
                    user_base_token_account,
                    user_quote_token_account,
                    pool_base_token_account,
                    pool_quote_token_account,
                    protocol_fee_recipient,
                    protocol_fee_recipient_token_account,
                    base_token_program,
                    quote_token_program,
                    system_program,
                    associated_token_program,
                    event_authority,
                    program,
                    coin_creator_vault_ata,
                    coin_creator_vault_authority,
                    global_volume_accumulator,
                    user_volume_accumulator,
                    fee_config,
                    fee_program,
                    remaining,
                };
                return Ok(Instruction::Buy { accounts, args });
            }
            [16u8, 4u8, 71u8, 28u8, 204u8, 1u8, 40u8, 27u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimTokenIncentivesArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user = keys.next().unwrap().clone();
                let user_ata = keys.next().unwrap().clone();
                let global_volume_accumulator = keys.next().unwrap().clone();
                let global_incentive_token_account = keys.next().unwrap().clone();
                let user_volume_accumulator = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimTokenIncentivesAccounts {
                    user,
                    user_ata,
                    global_volume_accumulator,
                    global_incentive_token_account,
                    user_volume_accumulator,
                    mint,
                    token_program,
                    system_program,
                    associated_token_program,
                    event_authority,
                    program,
                    payer,
                    remaining,
                };
                return Ok(Instruction::ClaimTokenIncentives { accounts, args });
            }
            [249u8, 69u8, 164u8, 218u8, 150u8, 103u8, 84u8, 138u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseUserVolumeAccumulatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user = keys.next().unwrap().clone();
                let user_volume_accumulator = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseUserVolumeAccumulatorAccounts {
                    user,
                    user_volume_accumulator,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CloseUserVolumeAccumulator { accounts, args });
            }
            [160u8, 57u8, 89u8, 42u8, 181u8, 139u8, 43u8, 66u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectCoinCreatorFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let quote_mint = keys.next().unwrap().clone();
                let quote_token_program = keys.next().unwrap().clone();
                let coin_creator = keys.next().unwrap().clone();
                let coin_creator_vault_authority = keys.next().unwrap().clone();
                let coin_creator_vault_ata = keys.next().unwrap().clone();
                let coin_creator_token_account = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectCoinCreatorFeeAccounts {
                    quote_mint,
                    quote_token_program,
                    coin_creator,
                    coin_creator_vault_authority,
                    coin_creator_vault_ata,
                    coin_creator_token_account,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CollectCoinCreatorFee { accounts, args });
            }
            [201u8, 207u8, 243u8, 114u8, 75u8, 111u8, 47u8, 189u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateConfigAccounts {
                    admin,
                    global_config,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CreateConfig { accounts, args });
            }
            [233u8, 146u8, 209u8, 142u8, 207u8, 104u8, 64u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreatePoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let creator = keys.next().unwrap().clone();
                let base_mint = keys.next().unwrap().clone();
                let quote_mint = keys.next().unwrap().clone();
                let lp_mint = keys.next().unwrap().clone();
                let user_base_token_account = keys.next().unwrap().clone();
                let user_quote_token_account = keys.next().unwrap().clone();
                let user_pool_token_account = keys.next().unwrap().clone();
                let pool_base_token_account = keys.next().unwrap().clone();
                let pool_quote_token_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let base_token_program = keys.next().unwrap().clone();
                let quote_token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreatePoolAccounts {
                    pool,
                    global_config,
                    creator,
                    base_mint,
                    quote_mint,
                    lp_mint,
                    user_base_token_account,
                    user_quote_token_account,
                    user_pool_token_account,
                    pool_base_token_account,
                    pool_quote_token_account,
                    system_program,
                    token_2022_program,
                    base_token_program,
                    quote_token_program,
                    associated_token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CreatePool { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let base_mint = keys.next().unwrap().clone();
                let quote_mint = keys.next().unwrap().clone();
                let lp_mint = keys.next().unwrap().clone();
                let user_base_token_account = keys.next().unwrap().clone();
                let user_quote_token_account = keys.next().unwrap().clone();
                let user_pool_token_account = keys.next().unwrap().clone();
                let pool_base_token_account = keys.next().unwrap().clone();
                let pool_quote_token_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    pool,
                    global_config,
                    user,
                    base_mint,
                    quote_mint,
                    lp_mint,
                    user_base_token_account,
                    user_quote_token_account,
                    user_pool_token_account,
                    pool_base_token_account,
                    pool_quote_token_account,
                    token_program,
                    token_2022_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [185u8, 173u8, 187u8, 90u8, 216u8, 15u8, 238u8, 233u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableAccounts {
                    admin,
                    global_config,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Disable { accounts, args });
            }
            [234u8, 102u8, 194u8, 203u8, 150u8, 72u8, 62u8, 229u8] => {
                let mut rdr: &[u8] = rest;
                let args = ExtendAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let account = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ExtendAccountAccounts {
                    account,
                    user,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ExtendAccount { accounts, args });
            }
            [94u8, 6u8, 202u8, 115u8, 255u8, 96u8, 232u8, 183u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitUserVolumeAccumulatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let user_volume_accumulator = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitUserVolumeAccumulatorAccounts {
                    payer,
                    user,
                    user_volume_accumulator,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitUserVolumeAccumulator { accounts, args });
            }
            [51u8, 230u8, 133u8, 164u8, 1u8, 127u8, 131u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let args = SellArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let base_mint = keys.next().unwrap().clone();
                let quote_mint = keys.next().unwrap().clone();
                let user_base_token_account = keys.next().unwrap().clone();
                let user_quote_token_account = keys.next().unwrap().clone();
                let pool_base_token_account = keys.next().unwrap().clone();
                let pool_quote_token_account = keys.next().unwrap().clone();
                let protocol_fee_recipient = keys.next().unwrap().clone();
                let protocol_fee_recipient_token_account = keys.next().unwrap().clone();
                let base_token_program = keys.next().unwrap().clone();
                let quote_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let coin_creator_vault_ata = keys.next().unwrap().clone();
                let coin_creator_vault_authority = keys.next().unwrap().clone();
                let fee_config = keys.next().unwrap().clone();
                let fee_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SellAccounts {
                    pool,
                    user,
                    global_config,
                    base_mint,
                    quote_mint,
                    user_base_token_account,
                    user_quote_token_account,
                    pool_base_token_account,
                    pool_quote_token_account,
                    protocol_fee_recipient,
                    protocol_fee_recipient_token_account,
                    base_token_program,
                    quote_token_program,
                    system_program,
                    associated_token_program,
                    event_authority,
                    program,
                    coin_creator_vault_ata,
                    coin_creator_vault_authority,
                    fee_config,
                    fee_program,
                    remaining,
                };
                return Ok(Instruction::Sell { accounts, args });
            }
            [210u8, 149u8, 128u8, 45u8, 188u8, 58u8, 78u8, 175u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetCoinCreatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let bonding_curve = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetCoinCreatorAccounts {
                    pool,
                    metadata,
                    bonding_curve,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SetCoinCreator { accounts, args });
            }
            [86u8, 31u8, 192u8, 87u8, 163u8, 87u8, 79u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let args = SyncUserVolumeAccumulatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user = keys.next().unwrap().clone();
                let global_volume_accumulator = keys.next().unwrap().clone();
                let user_volume_accumulator = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SyncUserVolumeAccumulatorAccounts {
                    user,
                    global_volume_accumulator,
                    user_volume_accumulator,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SyncUserVolumeAccumulator { accounts, args });
            }
            [161u8, 176u8, 40u8, 213u8, 60u8, 184u8, 179u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateAdminArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let new_admin = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAdminAccounts {
                    admin,
                    global_config,
                    new_admin,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateAdmin { accounts, args });
            }
            [104u8, 184u8, 103u8, 242u8, 88u8, 151u8, 107u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateFeeConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFeeConfigAccounts {
                    admin,
                    global_config,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateFeeConfig { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let global_config = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let base_mint = keys.next().unwrap().clone();
                let quote_mint = keys.next().unwrap().clone();
                let lp_mint = keys.next().unwrap().clone();
                let user_base_token_account = keys.next().unwrap().clone();
                let user_quote_token_account = keys.next().unwrap().clone();
                let user_pool_token_account = keys.next().unwrap().clone();
                let pool_base_token_account = keys.next().unwrap().clone();
                let pool_quote_token_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    pool,
                    global_config,
                    user,
                    base_mint,
                    quote_mint,
                    lp_mint,
                    user_base_token_account,
                    user_quote_token_account,
                    user_pool_token_account,
                    pool_base_token_account,
                    pool_quote_token_account,
                    token_program,
                    token_2022_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AdminSetCoinCreatorEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub admin_set_coin_creator_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub base_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub old_coin_creator: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub new_coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AdminUpdateTokenIncentivesEvent {
        pub start_time: i64,
        pub end_time: i64,
        pub day_number: u64,
        pub token_supply_per_day: u64,
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        pub seconds_in_a_day: i64,
        pub timestamp: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BuyEvent {
        pub timestamp: i64,
        pub base_amount_out: u64,
        pub max_quote_amount_in: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub quote_amount_in: u64,
        pub lp_fee_basis_points: u64,
        pub lp_fee: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee: u64,
        pub quote_amount_in_with_lp_fee: u64,
        pub user_quote_amount_in: u64,
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_base_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_quote_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub protocol_fee_recipient: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub protocol_fee_recipient_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub coin_creator: [u8; 32usize],
        pub coin_creator_fee_basis_points: u64,
        pub coin_creator_fee: u64,
        pub track_volume: bool,
        pub total_unclaimed_tokens: u64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
        pub last_update_timestamp: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimTokenIncentivesEvent {
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        pub amount: u64,
        pub timestamp: i64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseUserVolumeAccumulatorEvent {
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub timestamp: i64,
        pub total_unclaimed_tokens: u64,
        pub total_claimed_tokens: u64,
        pub current_sol_volume: u64,
        pub last_update_timestamp: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectCoinCreatorFeeEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub coin_creator: [u8; 32usize],
        pub coin_creator_fee: u64,
        #[serde(with = "pubkey_serde")]
        pub coin_creator_vault_ata: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub coin_creator_token_account: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateConfigEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub admin: [u8; 32usize],
        pub lp_fee_basis_points: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        pub coin_creator_fee_basis_points: u64,
        #[serde(with = "pubkey_serde")]
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePoolEvent {
        pub timestamp: i64,
        pub index: u16,
        #[serde(with = "pubkey_serde")]
        pub creator: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub base_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub quote_mint: [u8; 32usize],
        pub base_mint_decimals: u8,
        pub quote_mint_decimals: u8,
        pub base_amount_in: u64,
        pub quote_amount_in: u64,
        pub pool_base_amount: u64,
        pub pool_quote_amount: u64,
        pub minimum_liquidity: u64,
        pub initial_liquidity: u64,
        pub lp_token_amount_out: u64,
        pub pool_bump: u8,
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub lp_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_base_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_quote_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositEvent {
        pub timestamp: i64,
        pub lp_token_amount_out: u64,
        pub max_base_amount_in: u64,
        pub max_quote_amount_in: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub base_amount_in: u64,
        pub quote_amount_in: u64,
        pub lp_mint_supply: u64,
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_base_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_quote_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_pool_token_account: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DisableEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub admin: [u8; 32usize],
        pub disable_create_pool: bool,
        pub disable_deposit: bool,
        pub disable_withdraw: bool,
        pub disable_buy: bool,
        pub disable_sell: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExtendAccountEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub current_size: u64,
        pub new_size: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitUserVolumeAccumulatorEvent {
        #[serde(with = "pubkey_serde")]
        pub payer: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub timestamp: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SellEvent {
        pub timestamp: i64,
        pub base_amount_in: u64,
        pub min_quote_amount_out: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub quote_amount_out: u64,
        pub lp_fee_basis_points: u64,
        pub lp_fee: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee: u64,
        pub quote_amount_out_without_lp_fee: u64,
        pub user_quote_amount_out: u64,
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_base_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_quote_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub protocol_fee_recipient: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub protocol_fee_recipient_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub coin_creator: [u8; 32usize],
        pub coin_creator_fee_basis_points: u64,
        pub coin_creator_fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetBondingCurveCoinCreatorEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub base_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub bonding_curve: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetMetaplexCoinCreatorEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub base_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub metadata: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub coin_creator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SyncUserVolumeAccumulatorEvent {
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub total_claimed_tokens_before: u64,
        pub total_claimed_tokens_after: u64,
        pub timestamp: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateAdminEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub admin: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub new_admin: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFeeConfigEvent {
        pub timestamp: i64,
        #[serde(with = "pubkey_serde")]
        pub admin: [u8; 32usize],
        pub lp_fee_basis_points: u64,
        pub protocol_fee_basis_points: u64,
        pub protocol_fee_recipients: [[u8; 32usize]; 8usize],
        pub coin_creator_fee_basis_points: u64,
        #[serde(with = "pubkey_serde")]
        pub admin_set_coin_creator_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawEvent {
        pub timestamp: i64,
        pub lp_token_amount_in: u64,
        pub min_base_amount_out: u64,
        pub min_quote_amount_out: u64,
        pub user_base_token_reserves: u64,
        pub user_quote_token_reserves: u64,
        pub pool_base_token_reserves: u64,
        pub pool_quote_token_reserves: u64,
        pub base_amount_out: u64,
        pub quote_amount_out: u64,
        pub lp_mint_supply: u64,
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_base_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_quote_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user_pool_token_account: [u8; 32usize],
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        AdminSetCoinCreatorEvent {
            args: AdminSetCoinCreatorEvent,
        },
        AdminUpdateTokenIncentivesEvent {
            args: AdminUpdateTokenIncentivesEvent,
        },
        BuyEvent {
            args: BuyEvent,
        },
        ClaimTokenIncentivesEvent {
            args: ClaimTokenIncentivesEvent,
        },
        CloseUserVolumeAccumulatorEvent {
            args: CloseUserVolumeAccumulatorEvent,
        },
        CollectCoinCreatorFeeEvent {
            args: CollectCoinCreatorFeeEvent,
        },
        CreateConfigEvent {
            args: CreateConfigEvent,
        },
        CreatePoolEvent {
            args: CreatePoolEvent,
        },
        DepositEvent {
            args: DepositEvent,
        },
        DisableEvent {
            args: DisableEvent,
        },
        ExtendAccountEvent {
            args: ExtendAccountEvent,
        },
        InitUserVolumeAccumulatorEvent {
            args: InitUserVolumeAccumulatorEvent,
        },
        SellEvent {
            args: SellEvent,
        },
        SetBondingCurveCoinCreatorEvent {
            args: SetBondingCurveCoinCreatorEvent,
        },
        SetMetaplexCoinCreatorEvent {
            args: SetMetaplexCoinCreatorEvent,
        },
        SyncUserVolumeAccumulatorEvent {
            args: SyncUserVolumeAccumulatorEvent,
        },
        UpdateAdminEvent {
            args: UpdateAdminEvent,
        },
        UpdateFeeConfigEvent {
            args: UpdateFeeConfigEvent,
        },
        WithdrawEvent {
            args: WithdrawEvent,
        },
    }
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
            if data.len() < 16 {
                anyhow::bail!("Event log too short: {}", data.len());
            }
            let (wrapper, rest) = data.split_at(8);
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!("Missing event log prefix");
            }
            let (disc_slice, payload) = rest.split_at(8);
            let disc: [u8; 8] = disc_slice.try_into().unwrap();
            match disc {
                [45u8, 220u8, 93u8, 24u8, 25u8, 97u8, 172u8, 104u8] => {
                    let mut rdr = &payload[..];
                    let args = AdminSetCoinCreatorEvent::deserialize(&mut rdr)?;
                    return Ok(Event::AdminSetCoinCreatorEvent { args });
                }
                [147u8, 250u8, 108u8, 120u8, 247u8, 29u8, 67u8, 222u8] => {
                    let mut rdr = &payload[..];
                    let args = AdminUpdateTokenIncentivesEvent::deserialize(&mut rdr)?;
                    return Ok(Event::AdminUpdateTokenIncentivesEvent { args });
                }
                [103u8, 244u8, 82u8, 31u8, 44u8, 245u8, 119u8, 119u8] => {
                    let mut rdr = &payload[..];
                    let args = BuyEvent::deserialize(&mut rdr)?;
                    return Ok(Event::BuyEvent { args });
                }
                [79u8, 172u8, 246u8, 49u8, 205u8, 91u8, 206u8, 232u8] => {
                    let mut rdr = &payload[..];
                    let args = ClaimTokenIncentivesEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimTokenIncentivesEvent { args });
                }
                [146u8, 159u8, 189u8, 172u8, 146u8, 88u8, 56u8, 244u8] => {
                    let mut rdr = &payload[..];
                    let args = CloseUserVolumeAccumulatorEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CloseUserVolumeAccumulatorEvent { args });
                }
                [232u8, 245u8, 194u8, 238u8, 234u8, 218u8, 58u8, 89u8] => {
                    let mut rdr = &payload[..];
                    let args = CollectCoinCreatorFeeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CollectCoinCreatorFeeEvent { args });
                }
                [107u8, 52u8, 89u8, 129u8, 55u8, 226u8, 81u8, 22u8] => {
                    let mut rdr = &payload[..];
                    let args = CreateConfigEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CreateConfigEvent { args });
                }
                [177u8, 49u8, 12u8, 210u8, 160u8, 118u8, 167u8, 116u8] => {
                    let mut rdr = &payload[..];
                    let args = CreatePoolEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CreatePoolEvent { args });
                }
                [120u8, 248u8, 61u8, 83u8, 31u8, 142u8, 107u8, 144u8] => {
                    let mut rdr = &payload[..];
                    let args = DepositEvent::deserialize(&mut rdr)?;
                    return Ok(Event::DepositEvent { args });
                }
                [107u8, 253u8, 193u8, 76u8, 228u8, 202u8, 27u8, 104u8] => {
                    let mut rdr = &payload[..];
                    let args = DisableEvent::deserialize(&mut rdr)?;
                    return Ok(Event::DisableEvent { args });
                }
                [97u8, 97u8, 215u8, 144u8, 93u8, 146u8, 22u8, 124u8] => {
                    let mut rdr = &payload[..];
                    let args = ExtendAccountEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ExtendAccountEvent { args });
                }
                [134u8, 36u8, 13u8, 72u8, 232u8, 101u8, 130u8, 216u8] => {
                    let mut rdr = &payload[..];
                    let args = InitUserVolumeAccumulatorEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InitUserVolumeAccumulatorEvent { args });
                }
                [62u8, 47u8, 55u8, 10u8, 165u8, 3u8, 220u8, 42u8] => {
                    let mut rdr = &payload[..];
                    let args = SellEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SellEvent { args });
                }
                [242u8, 231u8, 235u8, 102u8, 65u8, 99u8, 189u8, 211u8] => {
                    let mut rdr = &payload[..];
                    let args = SetBondingCurveCoinCreatorEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SetBondingCurveCoinCreatorEvent { args });
                }
                [150u8, 107u8, 199u8, 123u8, 124u8, 207u8, 102u8, 228u8] => {
                    let mut rdr = &payload[..];
                    let args = SetMetaplexCoinCreatorEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SetMetaplexCoinCreatorEvent { args });
                }
                [197u8, 122u8, 167u8, 124u8, 116u8, 81u8, 91u8, 255u8] => {
                    let mut rdr = &payload[..];
                    let args = SyncUserVolumeAccumulatorEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SyncUserVolumeAccumulatorEvent { args });
                }
                [225u8, 152u8, 171u8, 87u8, 246u8, 63u8, 66u8, 234u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdateAdminEvent::deserialize(&mut rdr)?;
                    return Ok(Event::UpdateAdminEvent { args });
                }
                [90u8, 23u8, 65u8, 35u8, 62u8, 244u8, 188u8, 208u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdateFeeConfigEvent::deserialize(&mut rdr)?;
                    return Ok(Event::UpdateFeeConfigEvent { args });
                }
                [22u8, 9u8, 133u8, 26u8, 160u8, 44u8, 71u8, 192u8] => {
                    let mut rdr = &payload[..];
                    let args = WithdrawEvent::deserialize(&mut rdr)?;
                    return Ok(Event::WithdrawEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
