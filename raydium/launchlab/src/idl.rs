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
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct ClaimVestedEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub beneficiary: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub claim_amount: u64,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct ConstantCurve {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub supply: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_base_sell: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_quote_fund_raising: u64,
        pub migrate_type: u8,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct CreateVestingEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub beneficiary: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_amount: u64,
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum CurveParams {
        Constant { data: ConstantCurve },
        Fixed { data: FixedCurve },
        Linear { data: LinearCurve },
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct FixedCurve {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub supply: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_quote_fund_raising: u64,
        pub migrate_type: u8,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct GlobalConfig {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        pub curve_type: u8,
        pub index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub migrate_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trade_fee_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_share_fee_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_base_supply: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_lock_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_base_sell_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_base_migrate_rate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_quote_fund_raising: u64,
        #[serde(with = "pubkey_serde")]
        pub quote_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub protocol_fee_owner: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub migrate_fee_owner: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub migrate_to_amm_wallet: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub migrate_to_cpswap_wallet: Pubkey,
        pub padding: [u64; 16],
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct LinearCurve {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub supply: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_quote_fund_raising: u64,
        pub migrate_type: u8,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct MigrateNftInfo {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub platform_scale: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub creator_scale: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub burn_scale: u64,
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct MintParams {
        pub decimals: u8,
        pub name: String,
        pub symbol: String,
        pub uri: String,
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct PlatformConfig {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        #[serde(with = "pubkey_serde")]
        pub platform_fee_wallet: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub platform_nft_wallet: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub platform_scale: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub creator_scale: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub burn_scale: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_rate: u64,
        #[serde(skip_serializing)]
        pub name: [u8; 64],
        #[serde(skip_serializing)]
        pub web: [u8; 256],
        #[serde(skip_serializing)]
        pub img: [u8; 256],
        #[serde(skip_serializing)]
        pub padding: [u8; 256],
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub enum PlatformConfigParam {
        FeeWallet(Pubkey),
        NFTWallet(Pubkey),
        MigrateNftInfo(MigrateNftInfo),
        FeeRate(u64),
        Name(String),
        Web(String),
        Img(String),
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct PlatformParams {
        pub migrate_nft_info: MigrateNftInfo,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fee_rate: u64,
        pub name: String,
        pub web: String,
        pub img: String,
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct PoolCreateEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub creator: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub config: Pubkey,
        pub base_mint_param: MintParams,
        pub curve_param: CurveParams,
        pub vesting_param: VestingParams,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct PoolState {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        pub auth_bump: u8,
        pub status: u8,
        pub base_decimals: u8,
        pub quote_decimals: u8,
        pub migrate_type: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub supply: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_base_sell: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub virtual_base: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub virtual_quote: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub real_base: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub real_quote: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_quote_fund_raising: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_protocol_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub platform_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub migrate_fee: u64,
        pub vesting_schedule: VestingSchedule,
        #[serde(with = "pubkey_serde")]
        pub global_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub platform_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub base_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub quote_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub base_vault: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub quote_vault: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub creator: Pubkey,
        pub padding: [u64; 8],
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum PoolStatus {
        Fund,
        Migrate,
        Trade,
    }
    impl Default for PoolStatus {
        fn default() -> Self {
            Self::Fund
        }
    }
    #[derive(serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum TradeDirection {
        Buy,
        Sell,
    }
    impl Default for TradeDirection {
        fn default() -> Self {
            Self::Buy
        }
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct TradeEvent {
        #[serde(with = "pubkey_serde")]
        pub pool_state: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_base_sell: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub virtual_base: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub virtual_quote: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub real_base_before: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub real_quote_before: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub real_base_after: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub real_quote_after: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub protocol_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub platform_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_fee: u64,
        pub trade_direction: TradeDirection,
        pub pool_status: PoolStatus,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct VestingParams {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_locked_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cliff_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unlock_period: u64,
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct VestingRecord {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        #[serde(with = "pubkey_serde")]
        pub pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub beneficiary: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub claimed_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_share_amount: u64,
        pub padding: [u64; 8],
    }
    #[derive(
        serde :: Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default,
    )]
    pub struct VestingSchedule {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_locked_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cliff_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unlock_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub start_time: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub allocated_share_amount: u64,
    }
    impl Default for CurveParams {
        fn default() -> Self {
            Self::Constant {
                data: ConstantCurve::default(),
            }
        }
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, serde :: Serialize)]
    pub struct BuyExactInAccounts {
        pub payer: String,
        pub authority: String,
        pub global_config: String,
        pub platform_config: String,
        pub pool_state: String,
        pub user_base_token: String,
        pub user_quote_token: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub base_token_mint: String,
        pub quote_token_mint: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct BuyExactOutAccounts {
        pub payer: String,
        pub authority: String,
        pub global_config: String,
        pub platform_config: String,
        pub pool_state: String,
        pub user_base_token: String,
        pub user_quote_token: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub base_token_mint: String,
        pub quote_token_mint: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ClaimPlatformFeeAccounts {
        pub platform_fee_wallet: String,
        pub authority: String,
        pub pool_state: String,
        pub platform_config: String,
        pub quote_vault: String,
        pub recipient_token_account: String,
        pub quote_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ClaimVestedTokenAccounts {
        pub beneficiary: String,
        pub authority: String,
        pub pool_state: String,
        pub vesting_record: String,
        pub base_vault: String,
        pub user_base_token: String,
        pub base_token_mint: String,
        pub base_token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CollectFeeAccounts {
        pub owner: String,
        pub authority: String,
        pub pool_state: String,
        pub global_config: String,
        pub quote_vault: String,
        pub quote_mint: String,
        pub recipient_token_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CollectMigrateFeeAccounts {
        pub owner: String,
        pub authority: String,
        pub pool_state: String,
        pub global_config: String,
        pub quote_vault: String,
        pub quote_mint: String,
        pub recipient_token_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CreateConfigAccounts {
        pub owner: String,
        pub global_config: String,
        pub quote_token_mint: String,
        pub protocol_fee_owner: String,
        pub migrate_fee_owner: String,
        pub migrate_to_amm_wallet: String,
        pub migrate_to_cpswap_wallet: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CreatePlatformConfigAccounts {
        pub platform_admin: String,
        pub platform_fee_wallet: String,
        pub platform_nft_wallet: String,
        pub platform_config: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CreateVestingAccountAccounts {
        pub creator: String,
        pub beneficiary: String,
        pub pool_state: String,
        pub vesting_record: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeAccounts {
        pub payer: String,
        pub creator: String,
        pub global_config: String,
        pub platform_config: String,
        pub authority: String,
        pub pool_state: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub metadata_account: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub metadata_program: String,
        pub system_program: String,
        pub rent_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct MigrateToAmmAccounts {
        pub payer: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub openbook_program: String,
        pub market: String,
        pub request_queue: String,
        pub event_queue: String,
        pub bids: String,
        pub asks: String,
        pub market_vault_signer: String,
        pub market_base_vault: String,
        pub market_quote_vault: String,
        pub amm_program: String,
        pub amm_pool: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_lp_mint: String,
        pub amm_base_vault: String,
        pub amm_quote_vault: String,
        pub amm_target_orders: String,
        pub amm_config: String,
        pub amm_create_fee_destination: String,
        pub authority: String,
        pub pool_state: String,
        pub global_config: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub pool_lp_token: String,
        pub spl_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub rent_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct MigrateToCpswapAccounts {
        pub payer: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub platform_config: String,
        pub cpswap_program: String,
        pub cpswap_pool: String,
        pub cpswap_authority: String,
        pub cpswap_lp_mint: String,
        pub cpswap_base_vault: String,
        pub cpswap_quote_vault: String,
        pub cpswap_config: String,
        pub cpswap_create_pool_fee: String,
        pub cpswap_observation: String,
        pub lock_program: String,
        pub lock_authority: String,
        pub lock_lp_vault: String,
        pub authority: String,
        pub pool_state: String,
        pub global_config: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub pool_lp_token: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub rent_program: String,
        pub metadata_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SellExactInAccounts {
        pub payer: String,
        pub authority: String,
        pub global_config: String,
        pub platform_config: String,
        pub pool_state: String,
        pub user_base_token: String,
        pub user_quote_token: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub base_token_mint: String,
        pub quote_token_mint: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SellExactOutAccounts {
        pub payer: String,
        pub authority: String,
        pub global_config: String,
        pub platform_config: String,
        pub pool_state: String,
        pub user_base_token: String,
        pub user_quote_token: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub base_token_mint: String,
        pub quote_token_mint: String,
        pub base_token_program: String,
        pub quote_token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateConfigAccounts {
        pub owner: String,
        pub global_config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePlatformConfigAccounts {
        pub platform_admin: String,
        pub platform_config: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct BuyExactInArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_fee_rate: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct BuyExactOutArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub maximum_amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_fee_rate: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct ClaimPlatformFeeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct ClaimVestedTokenArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CollectFeeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CollectMigrateFeeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CreateConfigArguments {
        pub curve_type: u8,
        pub index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub migrate_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trade_fee_rate: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CreatePlatformConfigArguments {
        pub platform_params: PlatformParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CreateVestingAccountArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct InitializeArguments {
        pub base_mint_param: MintParams,
        pub curve_param: CurveParams,
        pub vesting_param: VestingParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct MigrateToAmmArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_lot_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_lot_size: u64,
        pub market_vault_signer_nonce: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct MigrateToCpswapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SellExactInArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_fee_rate: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SellExactOutArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub maximum_amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_fee_rate: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct UpdateConfigArguments {
        pub param: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub value: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct UpdatePlatformConfigArguments {
        pub param: PlatformConfigParam,
    }
}
#[derive(Debug, serde :: Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    BuyExactIn {
        accounts: BuyExactInAccounts,
        args: BuyExactInArguments,
    },
    BuyExactOut {
        accounts: BuyExactOutAccounts,
        args: BuyExactOutArguments,
    },
    ClaimPlatformFee {
        accounts: ClaimPlatformFeeAccounts,
        args: ClaimPlatformFeeArguments,
    },
    ClaimVestedToken {
        accounts: ClaimVestedTokenAccounts,
        args: ClaimVestedTokenArguments,
    },
    CollectFee {
        accounts: CollectFeeAccounts,
        args: CollectFeeArguments,
    },
    CollectMigrateFee {
        accounts: CollectMigrateFeeAccounts,
        args: CollectMigrateFeeArguments,
    },
    CreateConfig {
        accounts: CreateConfigAccounts,
        args: CreateConfigArguments,
    },
    CreatePlatformConfig {
        accounts: CreatePlatformConfigAccounts,
        args: CreatePlatformConfigArguments,
    },
    CreateVestingAccount {
        accounts: CreateVestingAccountAccounts,
        args: CreateVestingAccountArguments,
    },
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArguments,
    },
    MigrateToAmm {
        accounts: MigrateToAmmAccounts,
        args: MigrateToAmmArguments,
    },
    MigrateToCpswap {
        accounts: MigrateToCpswapAccounts,
        args: MigrateToCpswapArguments,
    },
    SellExactIn {
        accounts: SellExactInAccounts,
        args: SellExactInArguments,
    },
    SellExactOut {
        accounts: SellExactOutAccounts,
        args: SellExactOutArguments,
    },
    UpdateConfig {
        accounts: UpdateConfigAccounts,
        args: UpdateConfigArguments,
    },
    UpdatePlatformConfig {
        accounts: UpdatePlatformConfigAccounts,
        args: UpdatePlatformConfigArguments,
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
            [250u8, 234u8, 13u8, 123u8, 213u8, 156u8, 19u8, 236u8] => {
                let mut rdr: &[u8] = rest;
                let args = BuyExactInArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let user_base_token = keys.next().unwrap_or(&"".to_string()).clone();
                let user_quote_token = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let event_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = BuyExactInAccounts {
                    payer,
                    authority,
                    global_config,
                    platform_config,
                    pool_state,
                    user_base_token,
                    user_quote_token,
                    base_vault,
                    quote_vault,
                    base_token_mint,
                    quote_token_mint,
                    base_token_program,
                    quote_token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::BuyExactIn { accounts, args });
            }
            [24u8, 211u8, 116u8, 40u8, 105u8, 3u8, 153u8, 56u8] => {
                let mut rdr: &[u8] = rest;
                let args = BuyExactOutArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let user_base_token = keys.next().unwrap_or(&"".to_string()).clone();
                let user_quote_token = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let event_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = BuyExactOutAccounts {
                    payer,
                    authority,
                    global_config,
                    platform_config,
                    pool_state,
                    user_base_token,
                    user_quote_token,
                    base_vault,
                    quote_vault,
                    base_token_mint,
                    quote_token_mint,
                    base_token_program,
                    quote_token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::BuyExactOut { accounts, args });
            }
            [156u8, 39u8, 208u8, 135u8, 76u8, 237u8, 61u8, 72u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimPlatformFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let platform_fee_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let recipient_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let associated_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimPlatformFeeAccounts {
                    platform_fee_wallet,
                    authority,
                    pool_state,
                    platform_config,
                    quote_vault,
                    recipient_token_account,
                    quote_mint,
                    token_program,
                    system_program,
                    associated_token_program,
                    remaining,
                };
                return Ok(Instruction::ClaimPlatformFee { accounts, args });
            }
            [49u8, 33u8, 104u8, 30u8, 189u8, 157u8, 79u8, 35u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimVestedTokenArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let beneficiary = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let vesting_record = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let user_base_token = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let associated_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimVestedTokenAccounts {
                    beneficiary,
                    authority,
                    pool_state,
                    vesting_record,
                    base_vault,
                    user_base_token,
                    base_token_mint,
                    base_token_program,
                    system_program,
                    associated_token_program,
                    remaining,
                };
                return Ok(Instruction::ClaimVestedToken { accounts, args });
            }
            [60u8, 173u8, 247u8, 103u8, 4u8, 93u8, 130u8, 48u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let recipient_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectFeeAccounts {
                    owner,
                    authority,
                    pool_state,
                    global_config,
                    quote_vault,
                    quote_mint,
                    recipient_token_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::CollectFee { accounts, args });
            }
            [255u8, 186u8, 150u8, 223u8, 235u8, 118u8, 201u8, 186u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectMigrateFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let recipient_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectMigrateFeeAccounts {
                    owner,
                    authority,
                    pool_state,
                    global_config,
                    quote_vault,
                    quote_mint,
                    recipient_token_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::CollectMigrateFee { accounts, args });
            }
            [201u8, 207u8, 243u8, 114u8, 75u8, 111u8, 47u8, 189u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let protocol_fee_owner = keys.next().unwrap_or(&"".to_string()).clone();
                let migrate_fee_owner = keys.next().unwrap_or(&"".to_string()).clone();
                let migrate_to_amm_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let migrate_to_cpswap_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateConfigAccounts {
                    owner,
                    global_config,
                    quote_token_mint,
                    protocol_fee_owner,
                    migrate_fee_owner,
                    migrate_to_amm_wallet,
                    migrate_to_cpswap_wallet,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateConfig { accounts, args });
            }
            [176u8, 90u8, 196u8, 175u8, 253u8, 113u8, 220u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreatePlatformConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let platform_admin = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_fee_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_nft_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreatePlatformConfigAccounts {
                    platform_admin,
                    platform_fee_wallet,
                    platform_nft_wallet,
                    platform_config,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreatePlatformConfig { accounts, args });
            }
            [129u8, 178u8, 2u8, 13u8, 217u8, 172u8, 230u8, 218u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateVestingAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let creator = keys.next().unwrap_or(&"".to_string()).clone();
                let beneficiary = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let vesting_record = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateVestingAccountAccounts {
                    creator,
                    beneficiary,
                    pool_state,
                    vesting_record,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateVestingAccount { accounts, args });
            }
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let creator = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let base_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let metadata_account = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let metadata_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent_program = keys.next().unwrap_or(&"".to_string()).clone();
                let event_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    payer,
                    creator,
                    global_config,
                    platform_config,
                    authority,
                    pool_state,
                    base_mint,
                    quote_mint,
                    base_vault,
                    quote_vault,
                    metadata_account,
                    base_token_program,
                    quote_token_program,
                    metadata_program,
                    system_program,
                    rent_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [207u8, 82u8, 192u8, 145u8, 254u8, 207u8, 145u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigrateToAmmArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let base_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let openbook_program = keys.next().unwrap_or(&"".to_string()).clone();
                let market = keys.next().unwrap_or(&"".to_string()).clone();
                let request_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let bids = keys.next().unwrap_or(&"".to_string()).clone();
                let asks = keys.next().unwrap_or(&"".to_string()).clone();
                let market_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let market_base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let market_quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_lp_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_config = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_create_fee_destination = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_lp_token = keys.next().unwrap_or(&"".to_string()).clone();
                let spl_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let associated_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateToAmmAccounts {
                    payer,
                    base_mint,
                    quote_mint,
                    openbook_program,
                    market,
                    request_queue,
                    event_queue,
                    bids,
                    asks,
                    market_vault_signer,
                    market_base_vault,
                    market_quote_vault,
                    amm_program,
                    amm_pool,
                    amm_authority,
                    amm_open_orders,
                    amm_lp_mint,
                    amm_base_vault,
                    amm_quote_vault,
                    amm_target_orders,
                    amm_config,
                    amm_create_fee_destination,
                    authority,
                    pool_state,
                    global_config,
                    base_vault,
                    quote_vault,
                    pool_lp_token,
                    spl_token_program,
                    associated_token_program,
                    system_program,
                    rent_program,
                    remaining,
                };
                return Ok(Instruction::MigrateToAmm { accounts, args });
            }
            [136u8, 92u8, 200u8, 103u8, 28u8, 218u8, 144u8, 140u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigrateToCpswapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let base_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_program = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_lp_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_config = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_create_pool_fee = keys.next().unwrap_or(&"".to_string()).clone();
                let cpswap_observation = keys.next().unwrap_or(&"".to_string()).clone();
                let lock_program = keys.next().unwrap_or(&"".to_string()).clone();
                let lock_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let lock_lp_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_lp_token = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let associated_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent_program = keys.next().unwrap_or(&"".to_string()).clone();
                let metadata_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateToCpswapAccounts {
                    payer,
                    base_mint,
                    quote_mint,
                    platform_config,
                    cpswap_program,
                    cpswap_pool,
                    cpswap_authority,
                    cpswap_lp_mint,
                    cpswap_base_vault,
                    cpswap_quote_vault,
                    cpswap_config,
                    cpswap_create_pool_fee,
                    cpswap_observation,
                    lock_program,
                    lock_authority,
                    lock_lp_vault,
                    authority,
                    pool_state,
                    global_config,
                    base_vault,
                    quote_vault,
                    pool_lp_token,
                    base_token_program,
                    quote_token_program,
                    associated_token_program,
                    system_program,
                    rent_program,
                    metadata_program,
                    remaining,
                };
                return Ok(Instruction::MigrateToCpswap { accounts, args });
            }
            [149u8, 39u8, 222u8, 155u8, 211u8, 124u8, 152u8, 26u8] => {
                let mut rdr: &[u8] = rest;
                let args = SellExactInArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let user_base_token = keys.next().unwrap_or(&"".to_string()).clone();
                let user_quote_token = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let event_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SellExactInAccounts {
                    payer,
                    authority,
                    global_config,
                    platform_config,
                    pool_state,
                    user_base_token,
                    user_quote_token,
                    base_vault,
                    quote_vault,
                    base_token_mint,
                    quote_token_mint,
                    base_token_program,
                    quote_token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SellExactIn { accounts, args });
            }
            [95u8, 200u8, 71u8, 34u8, 8u8, 9u8, 11u8, 166u8] => {
                let mut rdr: &[u8] = rest;
                let args = SellExactOutArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let authority = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_state = keys.next().unwrap_or(&"".to_string()).clone();
                let user_base_token = keys.next().unwrap_or(&"".to_string()).clone();
                let user_quote_token = keys.next().unwrap_or(&"".to_string()).clone();
                let base_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let base_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let quote_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let event_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SellExactOutAccounts {
                    payer,
                    authority,
                    global_config,
                    platform_config,
                    pool_state,
                    user_base_token,
                    user_quote_token,
                    base_vault,
                    quote_vault,
                    base_token_mint,
                    quote_token_mint,
                    base_token_program,
                    quote_token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SellExactOut { accounts, args });
            }
            [29u8, 158u8, 252u8, 191u8, 10u8, 83u8, 219u8, 99u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap_or(&"".to_string()).clone();
                let global_config = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateConfigAccounts {
                    owner,
                    global_config,
                    remaining,
                };
                return Ok(Instruction::UpdateConfig { accounts, args });
            }
            [195u8, 60u8, 76u8, 129u8, 146u8, 45u8, 67u8, 143u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePlatformConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let platform_admin = keys.next().unwrap_or(&"".to_string()).clone();
                let platform_config = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePlatformConfigAccounts {
                    platform_admin,
                    platform_config,
                    remaining,
                };
                return Ok(Instruction::UpdatePlatformConfig { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    #[derive(Debug, serde :: Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        ClaimVestedEvent { args: ClaimVestedEvent },
        CreateVestingEvent { args: CreateVestingEvent },
        PoolCreateEvent { args: PoolCreateEvent },
        TradeEvent { args: TradeEvent },
    }
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
            if data.len() < 16 {
                anyhow::bail!("Event log too short: {}", data.len());
            }
            let wrapper: [u8; 8] = data[0..8].try_into().unwrap();
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!("Invalid event wrapper: {:?}", wrapper);
            }
            let disc: [u8; 8] = data[8..16].try_into().unwrap();
            match disc {
                [21u8, 194u8, 114u8, 87u8, 120u8, 211u8, 226u8, 32u8] => {
                    let mut rdr = &data[16..];
                    let args = ClaimVestedEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimVestedEvent { args });
                }
                [150u8, 152u8, 11u8, 179u8, 52u8, 210u8, 191u8, 125u8] => {
                    let mut rdr = &data[16..];
                    let args = CreateVestingEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CreateVestingEvent { args });
                }
                [151u8, 215u8, 226u8, 9u8, 118u8, 161u8, 115u8, 174u8] => {
                    let mut rdr = &data[16..];
                    let args = PoolCreateEvent::deserialize(&mut rdr)?;
                    return Ok(Event::PoolCreateEvent { args });
                }
                [189u8, 219u8, 127u8, 211u8, 78u8, 230u8, 97u8, 238u8] => {
                    let mut rdr = &data[16..];
                    let args = TradeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::TradeEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
