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
#[doc = r" Parse an Option<T> in either old‑IDL (no tag) or new‑IDL (0x00/0x01 prefix) form"]
fn parse_option<T: ::borsh::BorshDeserialize>(rdr: &mut &[u8]) -> anyhow::Result<Option<T>> {
    if rdr.is_empty() {
        return Ok(None);
    }
    let tag = rdr[0];
    if tag == 0 {
        *rdr = &rdr[1..];
        return Ok(None);
    } else if tag == 1 {
        *rdr = &rdr[1..];
        let v = T::deserialize(rdr)?;
        return Ok(Some(v));
    }
    let v = T::deserialize(rdr)?;
    Ok(Some(v))
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
    pub struct UpdatePerpMarketSummaryStatsParams {
        pub quote_asset_amount_with_unsettled_lp: Option<i64>,
        pub net_unsettled_funding_pnl: Option<i64>,
        pub update_amm_summary_stats: Option<bool>,
        pub exclude_total_liq_fee: Option<bool>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidatePerpRecord {
        pub market_index: u16,
        pub oracle_price: i64,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lp_shares: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub fill_record_id: u64,
        pub user_order_id: u32,
        pub liquidator_order_id: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidator_fee: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub if_fee: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidateSpotRecord {
        pub asset_market_index: u16,
        pub asset_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub asset_transfer: u128,
        pub liability_market_index: u16,
        pub liability_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liability_transfer: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub if_fee: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidateBorrowForPerpPnlRecord {
        pub perp_market_index: u16,
        pub market_oracle_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pnl_transfer: u128,
        pub liability_market_index: u16,
        pub liability_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liability_transfer: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidatePerpPnlForDepositRecord {
        pub perp_market_index: u16,
        pub market_oracle_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pnl_transfer: u128,
        pub asset_market_index: u16,
        pub asset_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub asset_transfer: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PerpBankruptcyRecord {
        pub market_index: u16,
        pub pnl: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub if_payment: u128,
        #[serde(with = "pubkey_serde_option")]
        pub clawback_user: Option<[u8; 32usize]>,
        pub clawback_user_payment: Option<u128>,
        pub cumulative_funding_rate_delta: i128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SpotBankruptcyRecord {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub borrow_amount: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub if_payment: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cumulative_deposit_interest_delta: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct IfRebalanceConfigParams {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_in_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch_max_in_amount: u64,
        pub epoch_duration: i64,
        pub out_market_index: u16,
        pub in_market_index: u16,
        pub max_slippage_bps: u16,
        pub swap_mode: u8,
        pub status: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MarketIdentifier {
        pub market_type: MarketType,
        pub market_index: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HistoricalOracleData {
        pub last_oracle_price: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_oracle_conf: u64,
        pub last_oracle_delay: i64,
        pub last_oracle_price_twap: i64,
        pub last_oracle_price_twap5min: i64,
        pub last_oracle_price_twap_ts: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HistoricalIndexData {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_index_bid_price: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_index_ask_price: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_index_price_twap: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_index_price_twap5min: u64,
        pub last_index_price_twap_ts: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PrelaunchOracleParams {
        pub perp_market_index: u16,
        pub price: Option<i64>,
        pub max_price: Option<i64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OrderParams {
        pub order_type: OrderType,
        pub market_type: MarketType,
        pub direction: PositionDirection,
        pub user_order_id: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_asset_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub price: u64,
        pub market_index: u16,
        pub reduce_only: bool,
        pub post_only: PostOnlyParam,
        pub bit_flags: u8,
        pub max_ts: Option<i64>,
        pub trigger_price: Option<u64>,
        pub trigger_condition: OrderTriggerCondition,
        pub oracle_price_offset: Option<i32>,
        pub auction_duration: Option<u8>,
        pub auction_start_price: Option<i64>,
        pub auction_end_price: Option<i64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SignedMsgOrderParamsMessage {
        pub signed_msg_order_params: OrderParams,
        pub sub_account_id: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slot: u64,
        pub uuid: [u8; 8usize],
        pub take_profit_order_params: Option<SignedMsgTriggerOrderParams>,
        pub stop_loss_order_params: Option<SignedMsgTriggerOrderParams>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SignedMsgOrderParamsDelegateMessage {
        pub signed_msg_order_params: OrderParams,
        #[serde(with = "pubkey_serde")]
        pub taker_pubkey: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slot: u64,
        pub uuid: [u8; 8usize],
        pub take_profit_order_params: Option<SignedMsgTriggerOrderParams>,
        pub stop_loss_order_params: Option<SignedMsgTriggerOrderParams>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SignedMsgTriggerOrderParams {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trigger_price: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_asset_amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ModifyOrderParams {
        pub direction: Option<PositionDirection>,
        pub base_asset_amount: Option<u64>,
        pub price: Option<u64>,
        pub reduce_only: Option<bool>,
        pub post_only: Option<PostOnlyParam>,
        pub bit_flags: Option<u8>,
        pub max_ts: Option<i64>,
        pub trigger_price: Option<u64>,
        pub trigger_condition: Option<OrderTriggerCondition>,
        pub oracle_price_offset: Option<i32>,
        pub auction_duration: Option<u8>,
        pub auction_start_price: Option<i64>,
        pub auction_end_price: Option<i64>,
        pub policy: Option<u8>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InsuranceClaim {
        pub revenue_withdraw_since_last_settle: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_revenue_withdraw_per_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_max_insurance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_settled_insurance: u64,
        pub last_revenue_withdraw_ts: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PoolBalance {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub scaled_balance: u128,
        pub market_index: u16,
        pub padding: [u8; 6usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Amm {
        #[serde(with = "pubkey_serde")]
        pub oracle: [u8; 32usize],
        pub historical_oracle_data: HistoricalOracleData,
        pub base_asset_amount_per_lp: i128,
        pub quote_asset_amount_per_lp: i128,
        pub fee_pool: PoolBalance,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub concentration_coef: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_k: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub peg_multiplier: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub terminal_quote_asset_reserve: u128,
        pub base_asset_amount_long: i128,
        pub base_asset_amount_short: i128,
        pub base_asset_amount_with_amm: i128,
        pub base_asset_amount_with_unsettled_lp: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_open_interest: u128,
        pub quote_asset_amount: i128,
        pub quote_entry_amount_long: i128,
        pub quote_entry_amount_short: i128,
        pub quote_break_even_amount_long: i128,
        pub quote_break_even_amount_short: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub user_lp_shares: u128,
        pub last_funding_rate: i64,
        pub last_funding_rate_long: i64,
        pub last_funding_rate_short: i64,
        pub last24h_avg_funding_rate: i64,
        pub total_fee: i128,
        pub total_mm_fee: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_exchange_fee: u128,
        pub total_fee_minus_distributions: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fee_withdrawn: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_liquidation_fee: u128,
        pub cumulative_funding_rate_long: i128,
        pub cumulative_funding_rate_short: i128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_social_loss: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub ask_base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub ask_quote_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub bid_base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub bid_quote_asset_reserve: u128,
        pub last_oracle_normalised_price: i64,
        pub last_oracle_reserve_price_spread_pct: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_bid_price_twap: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_ask_price_twap: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_mark_price_twap: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_mark_price_twap5min: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_update_slot: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_oracle_conf_pct: u64,
        pub net_revenue_since_last_funding: i64,
        pub last_funding_rate_ts: i64,
        pub funding_period: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_step_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_tick_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_order_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_position_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub volume24h: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub long_intensity_volume: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub short_intensity_volume: u64,
        pub last_trade_ts: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub mark_std: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub oracle_std: u64,
        pub last_mark_price_twap_ts: i64,
        pub base_spread: u32,
        pub max_spread: u32,
        pub long_spread: u32,
        pub short_spread: u32,
        pub long_intensity_count: u32,
        pub short_intensity_count: u32,
        pub max_fill_reserve_fraction: u16,
        pub max_slippage_ratio: u16,
        pub curve_update_intensity: u8,
        pub amm_jit_intensity: u8,
        pub oracle_source: OracleSource,
        pub last_oracle_valid: bool,
        pub target_base_asset_amount_per_lp: i32,
        pub per_lp_base: i8,
        pub taker_speed_bump_override: i8,
        pub amm_spread_adjustment: i8,
        pub oracle_slot_delay_override: i8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fee_earned_per_lp: u64,
        pub net_unsettled_funding_pnl: i64,
        pub quote_asset_amount_with_unsettled_lp: i64,
        pub reference_price_offset: i32,
        pub amm_inventory_spread_adjustment: i8,
        pub padding: [u8; 11usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SignedMsgOrderId {
        pub uuid: [u8; 8usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_slot: u64,
        pub order_id: u32,
        pub padding: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SignedMsgUserOrdersFixed {
        #[serde(with = "pubkey_serde")]
        pub user_pubkey: [u8; 32usize],
        pub padding: u32,
        pub len: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InsuranceFund {
        #[serde(with = "pubkey_serde")]
        pub vault: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_shares: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub user_shares: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_base: u128,
        pub unstaking_period: i64,
        pub last_revenue_settle_ts: i64,
        pub revenue_settle_period: i64,
        pub total_factor: u32,
        pub user_factor: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OracleGuardRails {
        pub price_divergence: PriceDivergenceGuardRails,
        pub validity: ValidityGuardRails,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PriceDivergenceGuardRails {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub mark_oracle_percent_divergence: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub oracle_twap5min_percent_divergence: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ValidityGuardRails {
        pub slots_before_stale_for_amm: i64,
        pub slots_before_stale_for_margin: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub confidence_interval_max_size: u64,
        pub too_volatile_ratio: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeStructure {
        pub fee_tiers: [FeeTier; 10usize],
        pub filler_reward_structure: OrderFillerRewardStructure,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub referrer_reward_epoch_upper_bound: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub flat_filler_fee: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeTier {
        pub fee_numerator: u32,
        pub fee_denominator: u32,
        pub maker_rebate_numerator: u32,
        pub maker_rebate_denominator: u32,
        pub referrer_reward_numerator: u32,
        pub referrer_reward_denominator: u32,
        pub referee_fee_numerator: u32,
        pub referee_fee_denominator: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OrderFillerRewardStructure {
        pub reward_numerator: u32,
        pub reward_denominator: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub time_based_reward_lower_bound: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UserFees {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fee_paid: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_fee_rebate: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_token_discount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_referee_discount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_referrer_reward: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub current_epoch_referrer_reward: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SpotPosition {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub scaled_balance: u64,
        pub open_bids: i64,
        pub open_asks: i64,
        pub cumulative_deposits: i64,
        pub market_index: u16,
        pub balance_type: SpotBalanceType,
        pub open_orders: u8,
        pub padding: [u8; 4usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PerpPosition {
        pub last_cumulative_funding_rate: i64,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
        pub quote_break_even_amount: i64,
        pub quote_entry_amount: i64,
        pub open_bids: i64,
        pub open_asks: i64,
        pub settled_pnl: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lp_shares: u64,
        pub last_base_asset_amount_per_lp: i64,
        pub last_quote_asset_amount_per_lp: i64,
        pub remainder_base_asset_amount: i32,
        pub market_index: u16,
        pub open_orders: u8,
        pub per_lp_base: i8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Order {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slot: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub price: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_asset_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_asset_amount_filled: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_asset_amount_filled: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub trigger_price: u64,
        pub auction_start_price: i64,
        pub auction_end_price: i64,
        pub max_ts: i64,
        pub oracle_price_offset: i32,
        pub order_id: u32,
        pub market_index: u16,
        pub status: OrderStatus,
        pub order_type: OrderType,
        pub market_type: MarketType,
        pub user_order_id: u8,
        pub existing_position_direction: PositionDirection,
        pub direction: PositionDirection,
        pub reduce_only: bool,
        pub post_only: bool,
        pub immediate_or_cancel: bool,
        pub trigger_condition: OrderTriggerCondition,
        pub auction_duration: u8,
        pub posted_slot_tail: u8,
        pub bit_flags: u8,
        pub padding: [u8; 1usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SwapDirection {
        Add,
        Remove,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ModifyOrderId {
        UserOrderId(u8),
        OrderId(u32),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PositionDirection {
        Long,
        Short,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SpotFulfillmentType {
        SerumV3,
        Match,
        PhoenixV1,
        OpenbookV2,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SwapReduceOnly {
        In,
        Out,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TwapPeriod {
        FundingPeriod,
        FiveMin,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LiquidationMultiplierType {
        Discount,
        Premium,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MarginRequirementType {
        Initial,
        Fill,
        Maintenance,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OracleValidity {
        NonPositive,
        TooVolatile,
        TooUncertain,
        StaleForMargin,
        InsufficientDataPoints,
        StaleForAmm,
        Valid,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DriftAction {
        UpdateFunding,
        SettlePnl,
        TriggerOrder,
        FillOrderMatch,
        FillOrderAmm,
        Liquidate,
        MarginCalc,
        UpdateTwap,
        UpdateAmmCurve,
        OracleOrderPrice,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PositionUpdateType {
        Open,
        Increase,
        Reduce,
        Close,
        Flip,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DepositExplanation {
        None,
        Transfer,
        Borrow,
        RepayBorrow,
        Reward,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DepositDirection {
        Deposit,
        Withdraw,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderAction {
        Place,
        Cancel,
        Fill,
        Trigger,
        Expire,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderActionExplanation {
        None,
        InsufficientFreeCollateral,
        OraclePriceBreachedLimitPrice,
        MarketOrderFilledToLimitPrice,
        OrderExpired,
        Liquidation,
        OrderFilledWithAmm,
        OrderFilledWithAmmJit,
        OrderFilledWithMatch,
        OrderFilledWithMatchJit,
        MarketExpired,
        RiskingIncreasingOrder,
        ReduceOnlyOrderIncreasedPosition,
        OrderFillWithSerum,
        NoBorrowLiquidity,
        OrderFillWithPhoenix,
        OrderFilledWithAmmJitLpSplit,
        OrderFilledWithLpJit,
        DeriskLp,
        OrderFilledWithOpenbookV2,
        TransferPerpPosition,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LpAction {
        AddLiquidity,
        RemoveLiquidity,
        SettleLiquidity,
        RemoveLiquidityDerisk,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LiquidationType {
        LiquidatePerp,
        LiquidateSpot,
        LiquidateBorrowForPerpPnl,
        LiquidatePerpPnlForDeposit,
        PerpBankruptcy,
        SpotBankruptcy,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SettlePnlExplanation {
        None,
        ExpiredPosition,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StakeAction {
        Stake,
        UnstakeRequest,
        UnstakeCancelRequest,
        Unstake,
        UnstakeTransfer,
        StakeTransfer,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum FillMode {
        Fill,
        PlaceAndMake,
        PlaceAndTake(bool, u8),
        Liquidation,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PerpFulfillmentMethod {
        Amm(Option<u64>),
        Match([u8; 32usize], u16, u64),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SpotFulfillmentMethod {
        ExternalMarket,
        Match([u8; 32usize], u16),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MarginCalculationMode {
        Standard {
            track_open_orders_fraction: bool,
        },
        Liquidation {
            market_to_track_margin_requirement: Option<MarketIdentifier>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OracleSource {
        Pyth,
        Switchboard,
        QuoteAsset,
        Pyth1K,
        Pyth1M,
        PythStableCoin,
        Prelaunch,
        PythPull,
        Pyth1KPull,
        Pyth1MPull,
        PythStableCoinPull,
        SwitchboardOnDemand,
        PythLazer,
        PythLazer1K,
        PythLazer1M,
        PythLazerStableCoin,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderParamsBitFlag {
        ImmediateOrCancel,
        UpdateHighLeverageMode,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PostOnlyParam {
        None,
        MustPostOnly,
        TryPostOnly,
        Slide,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ModifyOrderPolicy {
        MustModify,
        ExcludePreviousFill,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PlaceAndTakeOrderSuccessCondition {
        PartialFill,
        FullFill,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PerpOperation {
        UpdateFunding,
        AmmFill,
        Fill,
        SettlePnl,
        SettlePnlWithPosition,
        Liquidation,
        AmmImmediateFill,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SpotOperation {
        UpdateCumulativeInterest,
        Fill,
        Deposit,
        Withdraw,
        Liquidation,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum InsuranceFundOperation {
        Init,
        Add,
        RequestRemove,
        Remove,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MarketStatus {
        Initialized,
        Active,
        FundingPaused,
        AmmPaused,
        FillPaused,
        WithdrawPaused,
        ReduceOnly,
        Settlement,
        Delisted,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ContractType {
        Perpetual,
        Future,
        Prediction,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ContractTier {
        A,
        B,
        C,
        Speculative,
        HighlySpeculative,
        Isolated,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AmmLiquiditySplit {
        ProtocolOwned,
        LpOwned,
        Shared,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AmmAvailability {
        Immediate,
        AfterMinDuration,
        Unavailable,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SettlePnlMode {
        MustSettle,
        TrySettle,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SpotBalanceType {
        Deposit,
        Borrow,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SpotFulfillmentConfigStatus {
        Enabled,
        Disabled,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AssetTier {
        Collateral,
        Protected,
        Cross,
        Isolated,
        Unlisted,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExchangeStatus {
        DepositPaused,
        WithdrawPaused,
        AmmPaused,
        FillPaused,
        LiqPaused,
        FundingPaused,
        SettlePnlPaused,
        AmmImmediateFillPaused,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UserStatus {
        BeingLiquidated,
        Bankrupt,
        ReduceOnly,
        AdvancedLp,
        ProtectedMakerOrders,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AssetType {
        Base,
        Quote,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderStatus {
        Init,
        Open,
        Filled,
        Canceled,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderType {
        Market,
        Limit,
        TriggerMarket,
        TriggerLimit,
        Oracle,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderTriggerCondition {
        Above,
        Below,
        TriggeredAbove,
        TriggeredBelow,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MarketType {
        Spot,
        Perp,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderBitFlag {
        SignedMessage,
        OracleTriggerMarket,
        SafeTriggerOrder,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ReferrerStatus {
        IsReferrer,
        IsReferred,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MarginMode {
        Default,
        HighLeverage,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum FuelOverflowStatus {
        Exists,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SignatureVerificationError {
        InvalidEd25519InstructionProgramId,
        InvalidEd25519InstructionDataLength,
        InvalidSignatureIndex,
        InvalidSignatureOffset,
        InvalidPublicKeyOffset,
        InvalidMessageOffset,
        InvalidMessageDataSize,
        InvalidInstructionIndex,
        MessageOffsetOverflow,
        InvalidMessageHex,
        InvalidMessageData,
        LoadInstructionAtFailed,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeUserAccounts {
        pub user: String,
        pub userStats: String,
        pub state: String,
        pub authority: String,
        pub payer: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeUserStatsAccounts {
        pub userStats: String,
        pub state: String,
        pub authority: String,
        pub payer: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeSignedMsgUserOrdersAccounts {
        pub signedMsgUserOrders: String,
        pub authority: String,
        pub payer: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResizeSignedMsgUserOrdersAccounts {
        pub signedMsgUserOrders: String,
        pub authority: String,
        pub user: String,
        pub payer: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeSignedMsgWsDelegatesAccounts {
        pub signedMsgWsDelegates: String,
        pub authority: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ChangeSignedMsgWsDelegateStatusAccounts {
        pub signedMsgWsDelegates: String,
        pub authority: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeFuelOverflowAccounts {
        pub fuelOverflow: String,
        pub userStats: String,
        pub authority: String,
        pub payer: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SweepFuelAccounts {
        pub fuelOverflow: String,
        pub userStats: String,
        pub authority: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResetFuelSeasonAccounts {
        pub userStats: String,
        pub authority: String,
        pub state: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeReferrerNameAccounts {
        pub referrerName: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub payer: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub driftSigner: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferDepositAccounts {
        pub fromUser: String,
        pub toUser: String,
        pub userStats: String,
        pub authority: String,
        pub state: String,
        pub spotMarketVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferPoolsAccounts {
        pub fromUser: String,
        pub toUser: String,
        pub userStats: String,
        pub authority: String,
        pub state: String,
        pub depositFromSpotMarketVault: String,
        pub depositToSpotMarketVault: String,
        pub borrowFromSpotMarketVault: String,
        pub borrowToSpotMarketVault: String,
        pub driftSigner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferPerpPositionAccounts {
        pub fromUser: String,
        pub toUser: String,
        pub userStats: String,
        pub authority: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlacePerpOrderAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelOrderAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelOrderByUserIdAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelOrdersAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelOrdersByIdsAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ModifyOrderAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ModifyOrderByUserIdAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceAndTakePerpOrderAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceAndMakePerpOrderAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub taker: String,
        pub takerStats: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceAndMakeSignedMsgPerpOrderAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub taker: String,
        pub takerStats: String,
        pub takerSignedMsgUserOrders: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceSignedMsgTakerOrderAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub signedMsgUserOrders: String,
        pub authority: String,
        pub ixSysvar: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceSpotOrderAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceAndTakeSpotOrderAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceAndMakeSpotOrderAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub taker: String,
        pub takerStats: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlaceOrdersAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BeginSwapAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub outSpotMarketVault: String,
        pub inSpotMarketVault: String,
        pub outTokenAccount: String,
        pub inTokenAccount: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EndSwapAccounts {
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub authority: String,
        pub outSpotMarketVault: String,
        pub inSpotMarketVault: String,
        pub outTokenAccount: String,
        pub inTokenAccount: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddPerpLpSharesAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemovePerpLpSharesAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemovePerpLpSharesInExpiringMarketAccounts {
        pub state: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserNameAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserCustomMarginRatioAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserMarginTradingEnabledAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserPoolIdAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserDelegateAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserReduceOnlyAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserAdvancedLpAccounts {
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserProtectedMakerOrdersAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub protectedMakerModeConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeleteUserAccounts {
        pub user: String,
        pub userStats: String,
        pub state: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ForceDeleteUserAccounts {
        pub user: String,
        pub userStats: String,
        pub state: String,
        pub authority: String,
        pub keeper: String,
        pub driftSigner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeleteSignedMsgUserOrdersAccounts {
        pub signedMsgUserOrders: String,
        pub state: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReclaimRentAccounts {
        pub user: String,
        pub userStats: String,
        pub state: String,
        pub authority: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EnableUserHighLeverageModeAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub highLeverageModeConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FillPerpOrderAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub fillerStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RevertFillAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub fillerStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FillSpotOrderAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub fillerStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TriggerOrderAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ForceCancelOrdersAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserIdleAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LogUserBalancesAccounts {
        pub state: String,
        pub authority: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DisableUserHighLeverageModeAccounts {
        pub state: String,
        pub authority: String,
        pub user: String,
        pub highLeverageModeConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserFuelBonusAccounts {
        pub state: String,
        pub authority: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserStatsReferrerStatusAccounts {
        pub state: String,
        pub authority: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserOpenOrdersCountAccounts {
        pub state: String,
        pub authority: String,
        pub filler: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AdminDisableUpdatePerpBidAskTwapAccounts {
        pub admin: String,
        pub state: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettlePnlAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettleMultiplePnlsAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettleFundingPaymentAccounts {
        pub state: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettleLpAccounts {
        pub state: String,
        pub user: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettleExpiredMarketAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidatePerpAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidatePerpWithFillAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateSpotAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateSpotWithSwapBeginAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub liabilitySpotMarketVault: String,
        pub assetSpotMarketVault: String,
        pub liabilityTokenAccount: String,
        pub assetTokenAccount: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateSpotWithSwapEndAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub liabilitySpotMarketVault: String,
        pub assetSpotMarketVault: String,
        pub liabilityTokenAccount: String,
        pub assetTokenAccount: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateBorrowForPerpPnlAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidatePerpPnlForDepositAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetUserStatusToBeingLiquidatedAccounts {
        pub state: String,
        pub user: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResolvePerpPnlDeficitAccounts {
        pub state: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResolvePerpBankruptcyAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub spotMarketVault: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResolveSpotBankruptcyAccounts {
        pub state: String,
        pub authority: String,
        pub liquidator: String,
        pub liquidatorStats: String,
        pub user: String,
        pub userStats: String,
        pub spotMarketVault: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettleRevenueToInsuranceFundAccounts {
        pub state: String,
        pub spotMarket: String,
        pub spotMarketVault: String,
        pub driftSigner: String,
        pub insuranceFundVault: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFundingRateAccounts {
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePrelaunchOracleAccounts {
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpBidAskTwapAccounts {
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub keeperStats: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketCumulativeInterestAccounts {
        pub state: String,
        pub spotMarket: String,
        pub oracle: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub spotMarketVault: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAmmsAccounts {
        pub state: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketExpiryAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserQuoteAssetInsuranceStakeAccounts {
        pub state: String,
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub signer: String,
        pub insuranceFundVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserGovTokenInsuranceStakeAccounts {
        pub state: String,
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub signer: String,
        pub insuranceFundVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateUserGovTokenInsuranceStakeDevnetAccounts {
        pub userStats: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeInsuranceFundStakeAccounts {
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub state: String,
        pub authority: String,
        pub payer: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddInsuranceFundStakeAccounts {
        pub state: String,
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RequestRemoveInsuranceFundStakeAccounts {
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub authority: String,
        pub insuranceFundVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelRequestRemoveInsuranceFundStakeAccounts {
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub authority: String,
        pub insuranceFundVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveInsuranceFundStakeAccounts {
        pub state: String,
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub authority: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferProtocolIfSharesAccounts {
        pub signer: String,
        pub transferConfig: String,
        pub state: String,
        pub spotMarket: String,
        pub insuranceFundStake: String,
        pub userStats: String,
        pub authority: String,
        pub insuranceFundVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BeginInsuranceFundSwapAccounts {
        pub state: String,
        pub authority: String,
        pub outInsuranceFundVault: String,
        pub inInsuranceFundVault: String,
        pub outTokenAccount: String,
        pub inTokenAccount: String,
        pub ifRebalanceConfig: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EndInsuranceFundSwapAccounts {
        pub state: String,
        pub authority: String,
        pub outInsuranceFundVault: String,
        pub inInsuranceFundVault: String,
        pub outTokenAccount: String,
        pub inTokenAccount: String,
        pub ifRebalanceConfig: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub instructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferProtocolIfSharesToRevenuePoolAccounts {
        pub state: String,
        pub authority: String,
        pub insuranceFundVault: String,
        pub spotMarketVault: String,
        pub ifRebalanceConfig: String,
        pub tokenProgram: String,
        pub driftSigner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePythPullOracleAccounts {
        pub keeper: String,
        pub pythSolanaReceiver: String,
        pub encodedVaa: String,
        pub priceFeed: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PostPythPullOracleUpdateAtomicAccounts {
        pub keeper: String,
        pub pythSolanaReceiver: String,
        pub guardianSet: String,
        pub priceFeed: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PostMultiPythPullOracleUpdatesAtomicAccounts {
        pub keeper: String,
        pub pythSolanaReceiver: String,
        pub guardianSet: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PauseSpotMarketDepositWithdrawAccounts {
        pub state: String,
        pub keeper: String,
        pub spotMarket: String,
        pub spotMarketVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub admin: String,
        pub state: String,
        pub quoteAssetMint: String,
        pub driftSigner: String,
        pub rent: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeSpotMarketAccounts {
        pub spotMarket: String,
        pub spotMarketMint: String,
        pub spotMarketVault: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub state: String,
        pub oracle: String,
        pub admin: String,
        pub rent: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeleteInitializedSpotMarketAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub spotMarketVault: String,
        pub insuranceFundVault: String,
        pub driftSigner: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeSerumFulfillmentConfigAccounts {
        pub baseSpotMarket: String,
        pub quoteSpotMarket: String,
        pub state: String,
        pub serumProgram: String,
        pub serumMarket: String,
        pub serumOpenOrders: String,
        pub driftSigner: String,
        pub serumFulfillmentConfig: String,
        pub admin: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSerumFulfillmentConfigStatusAccounts {
        pub state: String,
        pub serumFulfillmentConfig: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeOpenbookV2FulfillmentConfigAccounts {
        pub baseSpotMarket: String,
        pub quoteSpotMarket: String,
        pub state: String,
        pub openbookV2Program: String,
        pub openbookV2Market: String,
        pub driftSigner: String,
        pub openbookV2FulfillmentConfig: String,
        pub admin: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenbookV2FulfillmentConfigStatusAccounts {
        pub state: String,
        pub openbookV2FulfillmentConfig: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePhoenixFulfillmentConfigAccounts {
        pub baseSpotMarket: String,
        pub quoteSpotMarket: String,
        pub state: String,
        pub phoenixProgram: String,
        pub phoenixMarket: String,
        pub driftSigner: String,
        pub phoenixFulfillmentConfig: String,
        pub admin: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PhoenixFulfillmentConfigStatusAccounts {
        pub state: String,
        pub phoenixFulfillmentConfig: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSerumVaultAccounts {
        pub state: String,
        pub admin: String,
        pub srmVault: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePerpMarketAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePredictionMarketAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeleteInitializedPerpMarketAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MoveAmmPriceAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RecenterPerpMarketAmmAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RecenterPerpMarketAmmCrankAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub spotMarket: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketAmmSummaryStatsAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub spotMarket: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketExpiryAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SettleExpiredMarketPoolsToRevenuePoolAccounts {
        pub state: String,
        pub admin: String,
        pub spotMarket: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositIntoPerpMarketFeePoolAccounts {
        pub state: String,
        pub perpMarket: String,
        pub admin: String,
        pub sourceVault: String,
        pub driftSigner: String,
        pub quoteSpotMarket: String,
        pub spotMarketVault: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositIntoSpotMarketVaultAccounts {
        pub state: String,
        pub spotMarket: String,
        pub admin: String,
        pub sourceVault: String,
        pub spotMarketVault: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositIntoSpotMarketRevenuePoolAccounts {
        pub state: String,
        pub spotMarket: String,
        pub authority: String,
        pub spotMarketVault: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RepegAmmCurveAccounts {
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketAmmOracleTwapAccounts {
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResetPerpMarketAmmOracleTwapAccounts {
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateKAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMarginRatioAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketHighLeverageMarginRatioAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketFundingPeriodAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMaxImbalancesAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketLiquidationFeeAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateInsuranceFundUnstakingPeriodAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketPoolIdAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketLiquidationFeeAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateWithdrawGuardThresholdAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketIfFactorAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketRevenueSettlePeriodAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketStatusAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketPausedOperationsAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketAssetTierAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketMarginWeightsAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketBorrowRateAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketMaxTokenDepositsAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketMaxTokenBorrowsAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStartAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketOracleAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub oracle: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub oldOracle: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketStepSizeAndTickSizeAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketMinOrderSizeAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketOrdersEnabledAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketIfPausedOperationsAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketNameAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketStatusAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketPausedOperationsAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketContractTierAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketImfFactorAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketUnrealizedAssetWeightAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketConcentrationCoefAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketCurveUpdateIntensityAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketPerLpBaseAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateLpCooldownTimeAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpFeeStructureAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotFeeStructureAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateInitialPctToLiquidateAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateLiquidationDurationAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateLiquidationMarginBufferRatioAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateOracleGuardRailsAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStateSettlementDurationAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStateMaxNumberOfSubAccountsAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStateMaxInitializeUserFeeAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketOracleAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub oracle: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub oldOracle: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketBaseSpreadAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAmmJitIntensityAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMaxSpreadAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketStepSizeAndTickSizeAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketNameAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMinOrderSizeAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMaxSlippageRatioAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMaxFillReserveFractionAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketMaxOpenInterestAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketNumberOfUsersAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketFeeAdjustmentAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketFeeAdjustmentAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketFuelAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketProtectedMakerParamsAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketTakerSpeedBumpOverrideAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketAmmSpreadAdjustmentAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpMarketOracleSlotDelayOverrideAccounts {
        pub admin: String,
        pub state: String,
        pub perpMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotMarketFuelAccounts {
        pub admin: String,
        pub state: String,
        pub spotMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitUserFuelAccounts {
        pub admin: String,
        pub state: String,
        pub user: String,
        pub userStats: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAdminAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateWhitelistMintAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateDiscountMintAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateExchangeStatusAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePerpAuctionDurationAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSpotAuctionDurationAccounts {
        pub admin: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeProtocolIfSharesTransferConfigAccounts {
        pub admin: String,
        pub protocolIfSharesTransferConfig: String,
        pub state: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateProtocolIfSharesTransferConfigAccounts {
        pub admin: String,
        pub protocolIfSharesTransferConfig: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePrelaunchOracleAccounts {
        pub admin: String,
        pub prelaunchOracle: String,
        pub state: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePrelaunchOracleParamsAccounts {
        pub admin: String,
        pub prelaunchOracle: String,
        pub perpMarket: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeletePrelaunchOracleAccounts {
        pub admin: String,
        pub prelaunchOracle: String,
        pub perpMarket: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePythPullOracleAccounts {
        pub admin: String,
        pub pythSolanaReceiver: String,
        pub priceFeed: String,
        pub systemProgram: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePythLazerOracleAccounts {
        pub admin: String,
        pub lazerOracle: String,
        pub state: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PostPythLazerOracleUpdateAccounts {
        pub keeper: String,
        pub pythLazerStorage: String,
        pub ixSysvar: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeHighLeverageModeConfigAccounts {
        pub admin: String,
        pub highLeverageModeConfig: String,
        pub state: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateHighLeverageModeConfigAccounts {
        pub admin: String,
        pub highLeverageModeConfig: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeProtectedMakerModeConfigAccounts {
        pub admin: String,
        pub protectedMakerModeConfig: String,
        pub state: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateProtectedMakerModeConfigAccounts {
        pub admin: String,
        pub protectedMakerModeConfig: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AdminDepositAccounts {
        pub state: String,
        pub user: String,
        pub admin: String,
        pub spotMarketVault: String,
        pub adminTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeIfRebalanceConfigAccounts {
        pub admin: String,
        pub ifRebalanceConfig: String,
        pub state: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateIfRebalanceConfigAccounts {
        pub admin: String,
        pub ifRebalanceConfig: String,
        pub state: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeUserArguments {
        pub sub_account_id: u16,
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeUserStatsArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeSignedMsgUserOrdersArguments {
        pub num_orders: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ResizeSignedMsgUserOrdersArguments {
        pub num_orders: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeSignedMsgWsDelegatesArguments {
        pub delegates: Vec<[u8; 32usize]>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ChangeSignedMsgWsDelegateStatusArguments {
        #[serde(with = "pubkey_serde")]
        pub delegate: [u8; 32usize],
        pub add: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeFuelOverflowArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct SweepFuelArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct ResetFuelSeasonArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeReferrerNameArguments {
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub reduce_only: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct WithdrawArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub reduce_only: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct TransferDepositArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct TransferPoolsArguments {
        pub deposit_from_market_index: u16,
        pub deposit_to_market_index: u16,
        pub borrow_from_market_index: u16,
        pub borrow_to_market_index: u16,
        pub deposit_amount: Option<u64>,
        pub borrow_amount: Option<u64>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct TransferPerpPositionArguments {
        pub market_index: u16,
        pub amount: Option<i64>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlacePerpOrderArguments {
        pub params: OrderParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CancelOrderArguments {
        pub order_id: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CancelOrderByUserIdArguments {
        pub user_order_id: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CancelOrdersArguments {
        pub market_type: Option<MarketType>,
        pub market_index: Option<u16>,
        pub direction: Option<PositionDirection>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CancelOrdersByIdsArguments {
        pub order_ids: Vec<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ModifyOrderArguments {
        pub order_id: Option<u32>,
        pub modify_order_params: ModifyOrderParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ModifyOrderByUserIdArguments {
        pub user_order_id: u8,
        pub modify_order_params: ModifyOrderParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceAndTakePerpOrderArguments {
        pub params: OrderParams,
        pub success_condition: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceAndMakePerpOrderArguments {
        pub params: OrderParams,
        pub taker_order_id: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceAndMakeSignedMsgPerpOrderArguments {
        pub params: OrderParams,
        pub signed_msg_order_uuid: [u8; 8usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceSignedMsgTakerOrderArguments {
        pub signed_msg_order_params_message_bytes: Vec<u8>,
        pub is_delegate_signer: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceSpotOrderArguments {
        pub params: OrderParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceAndTakeSpotOrderArguments {
        pub params: OrderParams,
        pub fulfillment_type: Option<SpotFulfillmentType>,
        pub maker_order_id: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceAndMakeSpotOrderArguments {
        pub params: OrderParams,
        pub taker_order_id: u32,
        pub fulfillment_type: Option<SpotFulfillmentType>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PlaceOrdersArguments {
        pub params: Vec<OrderParams>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct BeginSwapArguments {
        pub in_market_index: u16,
        pub out_market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct EndSwapArguments {
        pub in_market_index: u16,
        pub out_market_index: u16,
        pub limit_price: Option<u64>,
        pub reduce_only: Option<SwapReduceOnly>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct AddPerpLpSharesArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub n_shares: u64,
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RemovePerpLpSharesArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_to_burn: u64,
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RemovePerpLpSharesInExpiringMarketArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_to_burn: u64,
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserNameArguments {
        pub sub_account_id: u16,
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserCustomMarginRatioArguments {
        pub sub_account_id: u16,
        pub margin_ratio: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserMarginTradingEnabledArguments {
        pub sub_account_id: u16,
        pub margin_trading_enabled: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserPoolIdArguments {
        pub sub_account_id: u16,
        pub pool_id: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserDelegateArguments {
        pub sub_account_id: u16,
        #[serde(with = "pubkey_serde")]
        pub delegate: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserReduceOnlyArguments {
        pub sub_account_id: u16,
        pub reduce_only: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserAdvancedLpArguments {
        pub sub_account_id: u16,
        pub advanced_lp: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserProtectedMakerOrdersArguments {
        pub sub_account_id: u16,
        pub protected_maker_orders: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DeleteUserArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct ForceDeleteUserArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct DeleteSignedMsgUserOrdersArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct ReclaimRentArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct EnableUserHighLeverageModeArguments {
        pub sub_account_id: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct FillPerpOrderArguments {
        pub order_id: Option<u32>,
        pub maker_order_id: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RevertFillArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct FillSpotOrderArguments {
        pub order_id: Option<u32>,
        pub fulfillment_type: Option<SpotFulfillmentType>,
        pub maker_order_id: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct TriggerOrderArguments {
        pub order_id: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ForceCancelOrdersArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserIdleArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct LogUserBalancesArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct DisableUserHighLeverageModeArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserFuelBonusArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserStatsReferrerStatusArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserOpenOrdersCountArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct AdminDisableUpdatePerpBidAskTwapArguments {
        pub disable: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SettlePnlArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SettleMultiplePnlsArguments {
        pub market_indexes: Vec<u16>,
        pub mode: SettlePnlMode,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SettleFundingPaymentArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct SettleLpArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SettleExpiredMarketArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidatePerpArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidator_max_base_asset_amount: u64,
        pub limit_price: Option<u64>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidatePerpWithFillArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidateSpotArguments {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidateSpotWithSwapBeginArguments {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidateSpotWithSwapEndArguments {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidateBorrowForPerpPnlArguments {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct LiquidatePerpPnlForDepositArguments {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidator_max_pnl_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SetUserStatusToBeingLiquidatedArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct ResolvePerpPnlDeficitArguments {
        pub spot_market_index: u16,
        pub perp_market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ResolvePerpBankruptcyArguments {
        pub quote_spot_market_index: u16,
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct ResolveSpotBankruptcyArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SettleRevenueToInsuranceFundArguments {
        pub spot_market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateFundingRateArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePrelaunchOracleArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpBidAskTwapArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketCumulativeInterestArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateAmmsArguments {
        pub market_indexes: [u16; 5usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketExpiryArguments {
        pub expiry_ts: i64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserQuoteAssetInsuranceStakeArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserGovTokenInsuranceStakeArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateUserGovTokenInsuranceStakeDevnetArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub gov_stake_amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeInsuranceFundStakeArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct AddInsuranceFundStakeArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RequestRemoveInsuranceFundStakeArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CancelRequestRemoveInsuranceFundStakeArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RemoveInsuranceFundStakeArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct TransferProtocolIfSharesArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct BeginInsuranceFundSwapArguments {
        pub in_market_index: u16,
        pub out_market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct EndInsuranceFundSwapArguments {
        pub in_market_index: u16,
        pub out_market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct TransferProtocolIfSharesToRevenuePoolArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePythPullOracleArguments {
        pub feed_id: [u8; 32usize],
        pub params: Vec<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PostPythPullOracleUpdateAtomicArguments {
        pub feed_id: [u8; 32usize],
        pub params: Vec<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PostMultiPythPullOracleUpdatesAtomicArguments {
        pub params: Vec<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PauseSpotMarketDepositWithdrawArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeSpotMarketArguments {
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
        pub oracle_source: OracleSource,
        pub initial_asset_weight: u32,
        pub maintenance_asset_weight: u32,
        pub initial_liability_weight: u32,
        pub maintenance_liability_weight: u32,
        pub imf_factor: u32,
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
        pub active_status: bool,
        pub asset_tier: AssetTier,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub scale_initial_asset_weight_start: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_guard_threshold: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_tick_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_step_size: u64,
        pub if_total_factor: u32,
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DeleteInitializedSpotMarketArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeSerumFulfillmentConfigArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSerumFulfillmentConfigStatusArguments {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeOpenbookV2FulfillmentConfigArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct OpenbookV2FulfillmentConfigStatusArguments {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializePhoenixFulfillmentConfigArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PhoenixFulfillmentConfigStatusArguments {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSerumVaultArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializePerpMarketArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amm_base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amm_quote_asset_reserve: u128,
        pub amm_periodicity: i64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amm_peg_multiplier: u128,
        pub oracle_source: OracleSource,
        pub contract_tier: ContractTier,
        pub margin_ratio_initial: u32,
        pub margin_ratio_maintenance: u32,
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
        pub imf_factor: u32,
        pub active_status: bool,
        pub base_spread: u32,
        pub max_spread: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_open_interest: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_revenue_withdraw_per_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_max_insurance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_step_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_tick_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_order_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub concentration_coef_scale: u128,
        pub curve_update_intensity: u8,
        pub amm_jit_intensity: u8,
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializePredictionMarketArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct DeleteInitializedPerpMarketArguments {
        pub market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct MoveAmmPriceArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub base_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_asset_reserve: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_k: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RecenterPerpMarketAmmArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub peg_multiplier: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_k: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RecenterPerpMarketAmmCrankArguments {
        pub depth: Option<u128>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketAmmSummaryStatsArguments {
        pub params: UpdatePerpMarketSummaryStatsParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketExpiryArguments {
        pub expiry_ts: i64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SettleExpiredMarketPoolsToRevenuePoolArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositIntoPerpMarketFeePoolArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositIntoSpotMarketVaultArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositIntoSpotMarketRevenuePoolArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RepegAmmCurveArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub new_peg_candidate: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketAmmOracleTwapArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct ResetPerpMarketAmmOracleTwapArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateKArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_k: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMarginRatioArguments {
        pub margin_ratio_initial: u32,
        pub margin_ratio_maintenance: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketHighLeverageMarginRatioArguments {
        pub margin_ratio_initial: u16,
        pub margin_ratio_maintenance: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketFundingPeriodArguments {
        pub funding_period: i64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMaxImbalancesArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unrealized_max_imbalance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_revenue_withdraw_per_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quote_max_insurance: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketLiquidationFeeArguments {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateInsuranceFundUnstakingPeriodArguments {
        pub insurance_fund_unstaking_period: i64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketPoolIdArguments {
        pub pool_id: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketLiquidationFeeArguments {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateWithdrawGuardThresholdArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_guard_threshold: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketIfFactorArguments {
        pub spot_market_index: u16,
        pub user_if_factor: u32,
        pub total_if_factor: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketRevenueSettlePeriodArguments {
        pub revenue_settle_period: i64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketStatusArguments {
        pub status: MarketStatus,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketPausedOperationsArguments {
        pub paused_operations: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketAssetTierArguments {
        pub asset_tier: AssetTier,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketMarginWeightsArguments {
        pub initial_asset_weight: u32,
        pub maintenance_asset_weight: u32,
        pub initial_liability_weight: u32,
        pub maintenance_liability_weight: u32,
        pub imf_factor: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketBorrowRateArguments {
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
        pub min_borrow_rate: Option<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketMaxTokenDepositsArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_token_deposits: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketMaxTokenBorrowsArguments {
        pub max_token_borrows_fraction: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStartArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub scale_initial_asset_weight_start: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketOracleArguments {
        #[serde(with = "pubkey_serde")]
        pub oracle: [u8; 32usize],
        pub oracle_source: OracleSource,
        pub skip_invariant_check: Option<bool>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketStepSizeAndTickSizeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub step_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tick_size: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketMinOrderSizeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_size: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketOrdersEnabledArguments {
        pub orders_enabled: bool,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketIfPausedOperationsArguments {
        pub paused_operations: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketNameArguments {
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketStatusArguments {
        pub status: MarketStatus,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketPausedOperationsArguments {
        pub paused_operations: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketContractTierArguments {
        pub contract_tier: ContractTier,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketImfFactorArguments {
        pub imf_factor: u32,
        pub unrealized_pnl_imf_factor: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketUnrealizedAssetWeightArguments {
        pub unrealized_initial_asset_weight: u32,
        pub unrealized_maintenance_asset_weight: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketConcentrationCoefArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub concentration_scale: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketCurveUpdateIntensityArguments {
        pub curve_update_intensity: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpArguments {
        pub target_base_asset_amount_per_lp: i32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketPerLpBaseArguments {
        pub per_lp_base: i8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateLpCooldownTimeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lp_cooldown_time: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpFeeStructureArguments {
        pub fee_structure: FeeStructure,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotFeeStructureArguments {
        pub fee_structure: FeeStructure,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateInitialPctToLiquidateArguments {
        pub initial_pct_to_liquidate: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateLiquidationDurationArguments {
        pub liquidation_duration: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateLiquidationMarginBufferRatioArguments {
        pub liquidation_margin_buffer_ratio: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateOracleGuardRailsArguments {
        pub oracle_guard_rails: OracleGuardRails,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateStateSettlementDurationArguments {
        pub settlement_duration: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateStateMaxNumberOfSubAccountsArguments {
        pub max_number_of_sub_accounts: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateStateMaxInitializeUserFeeArguments {
        pub max_initialize_user_fee: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketOracleArguments {
        #[serde(with = "pubkey_serde")]
        pub oracle: [u8; 32usize],
        pub oracle_source: OracleSource,
        pub skip_invariant_check: Option<bool>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketBaseSpreadArguments {
        pub base_spread: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateAmmJitIntensityArguments {
        pub amm_jit_intensity: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMaxSpreadArguments {
        pub max_spread: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketStepSizeAndTickSizeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub step_size: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tick_size: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketNameArguments {
        pub name: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMinOrderSizeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub order_size: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMaxSlippageRatioArguments {
        pub max_slippage_ratio: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMaxFillReserveFractionArguments {
        pub max_fill_reserve_fraction: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketMaxOpenInterestArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_open_interest: u128,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketNumberOfUsersArguments {
        pub number_of_users: Option<u32>,
        pub number_of_users_with_base: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketFeeAdjustmentArguments {
        pub fee_adjustment: i16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketFeeAdjustmentArguments {
        pub fee_adjustment: i16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketFuelArguments {
        pub fuel_boost_taker: Option<u8>,
        pub fuel_boost_maker: Option<u8>,
        pub fuel_boost_position: Option<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketProtectedMakerParamsArguments {
        pub protected_maker_limit_price_divisor: Option<u8>,
        pub protected_maker_dynamic_divisor: Option<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketTakerSpeedBumpOverrideArguments {
        pub taker_speed_bump_override: i8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketAmmSpreadAdjustmentArguments {
        pub amm_spread_adjustment: i8,
        pub amm_inventory_spread_adjustment: i8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpMarketOracleSlotDelayOverrideArguments {
        pub oracle_slot_delay_override: i8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotMarketFuelArguments {
        pub fuel_boost_deposits: Option<u8>,
        pub fuel_boost_borrows: Option<u8>,
        pub fuel_boost_taker: Option<u8>,
        pub fuel_boost_maker: Option<u8>,
        pub fuel_boost_insurance: Option<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitUserFuelArguments {
        pub fuel_boost_deposits: Option<i32>,
        pub fuel_boost_borrows: Option<u32>,
        pub fuel_boost_taker: Option<u32>,
        pub fuel_boost_maker: Option<u32>,
        pub fuel_boost_insurance: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateAdminArguments {
        #[serde(with = "pubkey_serde")]
        pub admin: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateWhitelistMintArguments {
        #[serde(with = "pubkey_serde")]
        pub whitelist_mint: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateDiscountMintArguments {
        #[serde(with = "pubkey_serde")]
        pub discount_mint: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateExchangeStatusArguments {
        pub exchange_status: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePerpAuctionDurationArguments {
        pub min_perp_auction_duration: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateSpotAuctionDurationArguments {
        pub default_spot_auction_duration: u8,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeProtocolIfSharesTransferConfigArguments {}
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateProtocolIfSharesTransferConfigArguments {
        pub whitelisted_signers: Option<[[u8; 32usize]; 4usize]>,
        pub max_transfer_per_epoch: Option<u128>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializePrelaunchOracleArguments {
        pub params: PrelaunchOracleParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdatePrelaunchOracleParamsArguments {
        pub params: PrelaunchOracleParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DeletePrelaunchOracleArguments {
        pub perp_market_index: u16,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializePythPullOracleArguments {
        pub feed_id: [u8; 32usize],
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializePythLazerOracleArguments {
        pub feed_id: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct PostPythLazerOracleUpdateArguments {
        pub pyth_message: Vec<u8>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeHighLeverageModeConfigArguments {
        pub max_users: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateHighLeverageModeConfigArguments {
        pub max_users: u32,
        pub reduce_only: bool,
        pub current_users: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeProtectedMakerModeConfigArguments {
        pub max_users: u32,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateProtectedMakerModeConfigArguments {
        pub max_users: u32,
        pub reduce_only: bool,
        pub current_users: Option<u32>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct AdminDepositArguments {
        pub market_index: u16,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeIfRebalanceConfigArguments {
        pub params: IfRebalanceConfigParams,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateIfRebalanceConfigArguments {
        pub params: IfRebalanceConfigParams,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeUser {
        accounts: InitializeUserAccounts,
        args: InitializeUserArguments,
    },
    InitializeUserStats {
        accounts: InitializeUserStatsAccounts,
        args: InitializeUserStatsArguments,
    },
    InitializeSignedMsgUserOrders {
        accounts: InitializeSignedMsgUserOrdersAccounts,
        args: InitializeSignedMsgUserOrdersArguments,
    },
    ResizeSignedMsgUserOrders {
        accounts: ResizeSignedMsgUserOrdersAccounts,
        args: ResizeSignedMsgUserOrdersArguments,
    },
    InitializeSignedMsgWsDelegates {
        accounts: InitializeSignedMsgWsDelegatesAccounts,
        args: InitializeSignedMsgWsDelegatesArguments,
    },
    ChangeSignedMsgWsDelegateStatus {
        accounts: ChangeSignedMsgWsDelegateStatusAccounts,
        args: ChangeSignedMsgWsDelegateStatusArguments,
    },
    InitializeFuelOverflow {
        accounts: InitializeFuelOverflowAccounts,
        args: InitializeFuelOverflowArguments,
    },
    SweepFuel {
        accounts: SweepFuelAccounts,
        args: SweepFuelArguments,
    },
    ResetFuelSeason {
        accounts: ResetFuelSeasonAccounts,
        args: ResetFuelSeasonArguments,
    },
    InitializeReferrerName {
        accounts: InitializeReferrerNameAccounts,
        args: InitializeReferrerNameArguments,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArguments,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArguments,
    },
    TransferDeposit {
        accounts: TransferDepositAccounts,
        args: TransferDepositArguments,
    },
    TransferPools {
        accounts: TransferPoolsAccounts,
        args: TransferPoolsArguments,
    },
    TransferPerpPosition {
        accounts: TransferPerpPositionAccounts,
        args: TransferPerpPositionArguments,
    },
    PlacePerpOrder {
        accounts: PlacePerpOrderAccounts,
        args: PlacePerpOrderArguments,
    },
    CancelOrder {
        accounts: CancelOrderAccounts,
        args: CancelOrderArguments,
    },
    CancelOrderByUserId {
        accounts: CancelOrderByUserIdAccounts,
        args: CancelOrderByUserIdArguments,
    },
    CancelOrders {
        accounts: CancelOrdersAccounts,
        args: CancelOrdersArguments,
    },
    CancelOrdersByIds {
        accounts: CancelOrdersByIdsAccounts,
        args: CancelOrdersByIdsArguments,
    },
    ModifyOrder {
        accounts: ModifyOrderAccounts,
        args: ModifyOrderArguments,
    },
    ModifyOrderByUserId {
        accounts: ModifyOrderByUserIdAccounts,
        args: ModifyOrderByUserIdArguments,
    },
    PlaceAndTakePerpOrder {
        accounts: PlaceAndTakePerpOrderAccounts,
        args: PlaceAndTakePerpOrderArguments,
    },
    PlaceAndMakePerpOrder {
        accounts: PlaceAndMakePerpOrderAccounts,
        args: PlaceAndMakePerpOrderArguments,
    },
    PlaceAndMakeSignedMsgPerpOrder {
        accounts: PlaceAndMakeSignedMsgPerpOrderAccounts,
        args: PlaceAndMakeSignedMsgPerpOrderArguments,
    },
    PlaceSignedMsgTakerOrder {
        accounts: PlaceSignedMsgTakerOrderAccounts,
        args: PlaceSignedMsgTakerOrderArguments,
    },
    PlaceSpotOrder {
        accounts: PlaceSpotOrderAccounts,
        args: PlaceSpotOrderArguments,
    },
    PlaceAndTakeSpotOrder {
        accounts: PlaceAndTakeSpotOrderAccounts,
        args: PlaceAndTakeSpotOrderArguments,
    },
    PlaceAndMakeSpotOrder {
        accounts: PlaceAndMakeSpotOrderAccounts,
        args: PlaceAndMakeSpotOrderArguments,
    },
    PlaceOrders {
        accounts: PlaceOrdersAccounts,
        args: PlaceOrdersArguments,
    },
    BeginSwap {
        accounts: BeginSwapAccounts,
        args: BeginSwapArguments,
    },
    EndSwap {
        accounts: EndSwapAccounts,
        args: EndSwapArguments,
    },
    AddPerpLpShares {
        accounts: AddPerpLpSharesAccounts,
        args: AddPerpLpSharesArguments,
    },
    RemovePerpLpShares {
        accounts: RemovePerpLpSharesAccounts,
        args: RemovePerpLpSharesArguments,
    },
    RemovePerpLpSharesInExpiringMarket {
        accounts: RemovePerpLpSharesInExpiringMarketAccounts,
        args: RemovePerpLpSharesInExpiringMarketArguments,
    },
    UpdateUserName {
        accounts: UpdateUserNameAccounts,
        args: UpdateUserNameArguments,
    },
    UpdateUserCustomMarginRatio {
        accounts: UpdateUserCustomMarginRatioAccounts,
        args: UpdateUserCustomMarginRatioArguments,
    },
    UpdateUserMarginTradingEnabled {
        accounts: UpdateUserMarginTradingEnabledAccounts,
        args: UpdateUserMarginTradingEnabledArguments,
    },
    UpdateUserPoolId {
        accounts: UpdateUserPoolIdAccounts,
        args: UpdateUserPoolIdArguments,
    },
    UpdateUserDelegate {
        accounts: UpdateUserDelegateAccounts,
        args: UpdateUserDelegateArguments,
    },
    UpdateUserReduceOnly {
        accounts: UpdateUserReduceOnlyAccounts,
        args: UpdateUserReduceOnlyArguments,
    },
    UpdateUserAdvancedLp {
        accounts: UpdateUserAdvancedLpAccounts,
        args: UpdateUserAdvancedLpArguments,
    },
    UpdateUserProtectedMakerOrders {
        accounts: UpdateUserProtectedMakerOrdersAccounts,
        args: UpdateUserProtectedMakerOrdersArguments,
    },
    DeleteUser {
        accounts: DeleteUserAccounts,
        args: DeleteUserArguments,
    },
    ForceDeleteUser {
        accounts: ForceDeleteUserAccounts,
        args: ForceDeleteUserArguments,
    },
    DeleteSignedMsgUserOrders {
        accounts: DeleteSignedMsgUserOrdersAccounts,
        args: DeleteSignedMsgUserOrdersArguments,
    },
    ReclaimRent {
        accounts: ReclaimRentAccounts,
        args: ReclaimRentArguments,
    },
    EnableUserHighLeverageMode {
        accounts: EnableUserHighLeverageModeAccounts,
        args: EnableUserHighLeverageModeArguments,
    },
    FillPerpOrder {
        accounts: FillPerpOrderAccounts,
        args: FillPerpOrderArguments,
    },
    RevertFill {
        accounts: RevertFillAccounts,
        args: RevertFillArguments,
    },
    FillSpotOrder {
        accounts: FillSpotOrderAccounts,
        args: FillSpotOrderArguments,
    },
    TriggerOrder {
        accounts: TriggerOrderAccounts,
        args: TriggerOrderArguments,
    },
    ForceCancelOrders {
        accounts: ForceCancelOrdersAccounts,
        args: ForceCancelOrdersArguments,
    },
    UpdateUserIdle {
        accounts: UpdateUserIdleAccounts,
        args: UpdateUserIdleArguments,
    },
    LogUserBalances {
        accounts: LogUserBalancesAccounts,
        args: LogUserBalancesArguments,
    },
    DisableUserHighLeverageMode {
        accounts: DisableUserHighLeverageModeAccounts,
        args: DisableUserHighLeverageModeArguments,
    },
    UpdateUserFuelBonus {
        accounts: UpdateUserFuelBonusAccounts,
        args: UpdateUserFuelBonusArguments,
    },
    UpdateUserStatsReferrerStatus {
        accounts: UpdateUserStatsReferrerStatusAccounts,
        args: UpdateUserStatsReferrerStatusArguments,
    },
    UpdateUserOpenOrdersCount {
        accounts: UpdateUserOpenOrdersCountAccounts,
        args: UpdateUserOpenOrdersCountArguments,
    },
    AdminDisableUpdatePerpBidAskTwap {
        accounts: AdminDisableUpdatePerpBidAskTwapAccounts,
        args: AdminDisableUpdatePerpBidAskTwapArguments,
    },
    SettlePnl {
        accounts: SettlePnlAccounts,
        args: SettlePnlArguments,
    },
    SettleMultiplePnls {
        accounts: SettleMultiplePnlsAccounts,
        args: SettleMultiplePnlsArguments,
    },
    SettleFundingPayment {
        accounts: SettleFundingPaymentAccounts,
        args: SettleFundingPaymentArguments,
    },
    SettleLp {
        accounts: SettleLpAccounts,
        args: SettleLpArguments,
    },
    SettleExpiredMarket {
        accounts: SettleExpiredMarketAccounts,
        args: SettleExpiredMarketArguments,
    },
    LiquidatePerp {
        accounts: LiquidatePerpAccounts,
        args: LiquidatePerpArguments,
    },
    LiquidatePerpWithFill {
        accounts: LiquidatePerpWithFillAccounts,
        args: LiquidatePerpWithFillArguments,
    },
    LiquidateSpot {
        accounts: LiquidateSpotAccounts,
        args: LiquidateSpotArguments,
    },
    LiquidateSpotWithSwapBegin {
        accounts: LiquidateSpotWithSwapBeginAccounts,
        args: LiquidateSpotWithSwapBeginArguments,
    },
    LiquidateSpotWithSwapEnd {
        accounts: LiquidateSpotWithSwapEndAccounts,
        args: LiquidateSpotWithSwapEndArguments,
    },
    LiquidateBorrowForPerpPnl {
        accounts: LiquidateBorrowForPerpPnlAccounts,
        args: LiquidateBorrowForPerpPnlArguments,
    },
    LiquidatePerpPnlForDeposit {
        accounts: LiquidatePerpPnlForDepositAccounts,
        args: LiquidatePerpPnlForDepositArguments,
    },
    SetUserStatusToBeingLiquidated {
        accounts: SetUserStatusToBeingLiquidatedAccounts,
        args: SetUserStatusToBeingLiquidatedArguments,
    },
    ResolvePerpPnlDeficit {
        accounts: ResolvePerpPnlDeficitAccounts,
        args: ResolvePerpPnlDeficitArguments,
    },
    ResolvePerpBankruptcy {
        accounts: ResolvePerpBankruptcyAccounts,
        args: ResolvePerpBankruptcyArguments,
    },
    ResolveSpotBankruptcy {
        accounts: ResolveSpotBankruptcyAccounts,
        args: ResolveSpotBankruptcyArguments,
    },
    SettleRevenueToInsuranceFund {
        accounts: SettleRevenueToInsuranceFundAccounts,
        args: SettleRevenueToInsuranceFundArguments,
    },
    UpdateFundingRate {
        accounts: UpdateFundingRateAccounts,
        args: UpdateFundingRateArguments,
    },
    UpdatePrelaunchOracle {
        accounts: UpdatePrelaunchOracleAccounts,
        args: UpdatePrelaunchOracleArguments,
    },
    UpdatePerpBidAskTwap {
        accounts: UpdatePerpBidAskTwapAccounts,
        args: UpdatePerpBidAskTwapArguments,
    },
    UpdateSpotMarketCumulativeInterest {
        accounts: UpdateSpotMarketCumulativeInterestAccounts,
        args: UpdateSpotMarketCumulativeInterestArguments,
    },
    UpdateAmms {
        accounts: UpdateAmmsAccounts,
        args: UpdateAmmsArguments,
    },
    UpdateSpotMarketExpiry {
        accounts: UpdateSpotMarketExpiryAccounts,
        args: UpdateSpotMarketExpiryArguments,
    },
    UpdateUserQuoteAssetInsuranceStake {
        accounts: UpdateUserQuoteAssetInsuranceStakeAccounts,
        args: UpdateUserQuoteAssetInsuranceStakeArguments,
    },
    UpdateUserGovTokenInsuranceStake {
        accounts: UpdateUserGovTokenInsuranceStakeAccounts,
        args: UpdateUserGovTokenInsuranceStakeArguments,
    },
    UpdateUserGovTokenInsuranceStakeDevnet {
        accounts: UpdateUserGovTokenInsuranceStakeDevnetAccounts,
        args: UpdateUserGovTokenInsuranceStakeDevnetArguments,
    },
    InitializeInsuranceFundStake {
        accounts: InitializeInsuranceFundStakeAccounts,
        args: InitializeInsuranceFundStakeArguments,
    },
    AddInsuranceFundStake {
        accounts: AddInsuranceFundStakeAccounts,
        args: AddInsuranceFundStakeArguments,
    },
    RequestRemoveInsuranceFundStake {
        accounts: RequestRemoveInsuranceFundStakeAccounts,
        args: RequestRemoveInsuranceFundStakeArguments,
    },
    CancelRequestRemoveInsuranceFundStake {
        accounts: CancelRequestRemoveInsuranceFundStakeAccounts,
        args: CancelRequestRemoveInsuranceFundStakeArguments,
    },
    RemoveInsuranceFundStake {
        accounts: RemoveInsuranceFundStakeAccounts,
        args: RemoveInsuranceFundStakeArguments,
    },
    TransferProtocolIfShares {
        accounts: TransferProtocolIfSharesAccounts,
        args: TransferProtocolIfSharesArguments,
    },
    BeginInsuranceFundSwap {
        accounts: BeginInsuranceFundSwapAccounts,
        args: BeginInsuranceFundSwapArguments,
    },
    EndInsuranceFundSwap {
        accounts: EndInsuranceFundSwapAccounts,
        args: EndInsuranceFundSwapArguments,
    },
    TransferProtocolIfSharesToRevenuePool {
        accounts: TransferProtocolIfSharesToRevenuePoolAccounts,
        args: TransferProtocolIfSharesToRevenuePoolArguments,
    },
    UpdatePythPullOracle {
        accounts: UpdatePythPullOracleAccounts,
        args: UpdatePythPullOracleArguments,
    },
    PostPythPullOracleUpdateAtomic {
        accounts: PostPythPullOracleUpdateAtomicAccounts,
        args: PostPythPullOracleUpdateAtomicArguments,
    },
    PostMultiPythPullOracleUpdatesAtomic {
        accounts: PostMultiPythPullOracleUpdatesAtomicAccounts,
        args: PostMultiPythPullOracleUpdatesAtomicArguments,
    },
    PauseSpotMarketDepositWithdraw {
        accounts: PauseSpotMarketDepositWithdrawAccounts,
        args: PauseSpotMarketDepositWithdrawArguments,
    },
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArguments,
    },
    InitializeSpotMarket {
        accounts: InitializeSpotMarketAccounts,
        args: InitializeSpotMarketArguments,
    },
    DeleteInitializedSpotMarket {
        accounts: DeleteInitializedSpotMarketAccounts,
        args: DeleteInitializedSpotMarketArguments,
    },
    InitializeSerumFulfillmentConfig {
        accounts: InitializeSerumFulfillmentConfigAccounts,
        args: InitializeSerumFulfillmentConfigArguments,
    },
    UpdateSerumFulfillmentConfigStatus {
        accounts: UpdateSerumFulfillmentConfigStatusAccounts,
        args: UpdateSerumFulfillmentConfigStatusArguments,
    },
    InitializeOpenbookV2FulfillmentConfig {
        accounts: InitializeOpenbookV2FulfillmentConfigAccounts,
        args: InitializeOpenbookV2FulfillmentConfigArguments,
    },
    OpenbookV2FulfillmentConfigStatus {
        accounts: OpenbookV2FulfillmentConfigStatusAccounts,
        args: OpenbookV2FulfillmentConfigStatusArguments,
    },
    InitializePhoenixFulfillmentConfig {
        accounts: InitializePhoenixFulfillmentConfigAccounts,
        args: InitializePhoenixFulfillmentConfigArguments,
    },
    PhoenixFulfillmentConfigStatus {
        accounts: PhoenixFulfillmentConfigStatusAccounts,
        args: PhoenixFulfillmentConfigStatusArguments,
    },
    UpdateSerumVault {
        accounts: UpdateSerumVaultAccounts,
        args: UpdateSerumVaultArguments,
    },
    InitializePerpMarket {
        accounts: InitializePerpMarketAccounts,
        args: InitializePerpMarketArguments,
    },
    InitializePredictionMarket {
        accounts: InitializePredictionMarketAccounts,
        args: InitializePredictionMarketArguments,
    },
    DeleteInitializedPerpMarket {
        accounts: DeleteInitializedPerpMarketAccounts,
        args: DeleteInitializedPerpMarketArguments,
    },
    MoveAmmPrice {
        accounts: MoveAmmPriceAccounts,
        args: MoveAmmPriceArguments,
    },
    RecenterPerpMarketAmm {
        accounts: RecenterPerpMarketAmmAccounts,
        args: RecenterPerpMarketAmmArguments,
    },
    RecenterPerpMarketAmmCrank {
        accounts: RecenterPerpMarketAmmCrankAccounts,
        args: RecenterPerpMarketAmmCrankArguments,
    },
    UpdatePerpMarketAmmSummaryStats {
        accounts: UpdatePerpMarketAmmSummaryStatsAccounts,
        args: UpdatePerpMarketAmmSummaryStatsArguments,
    },
    UpdatePerpMarketExpiry {
        accounts: UpdatePerpMarketExpiryAccounts,
        args: UpdatePerpMarketExpiryArguments,
    },
    SettleExpiredMarketPoolsToRevenuePool {
        accounts: SettleExpiredMarketPoolsToRevenuePoolAccounts,
        args: SettleExpiredMarketPoolsToRevenuePoolArguments,
    },
    DepositIntoPerpMarketFeePool {
        accounts: DepositIntoPerpMarketFeePoolAccounts,
        args: DepositIntoPerpMarketFeePoolArguments,
    },
    DepositIntoSpotMarketVault {
        accounts: DepositIntoSpotMarketVaultAccounts,
        args: DepositIntoSpotMarketVaultArguments,
    },
    DepositIntoSpotMarketRevenuePool {
        accounts: DepositIntoSpotMarketRevenuePoolAccounts,
        args: DepositIntoSpotMarketRevenuePoolArguments,
    },
    RepegAmmCurve {
        accounts: RepegAmmCurveAccounts,
        args: RepegAmmCurveArguments,
    },
    UpdatePerpMarketAmmOracleTwap {
        accounts: UpdatePerpMarketAmmOracleTwapAccounts,
        args: UpdatePerpMarketAmmOracleTwapArguments,
    },
    ResetPerpMarketAmmOracleTwap {
        accounts: ResetPerpMarketAmmOracleTwapAccounts,
        args: ResetPerpMarketAmmOracleTwapArguments,
    },
    UpdateK {
        accounts: UpdateKAccounts,
        args: UpdateKArguments,
    },
    UpdatePerpMarketMarginRatio {
        accounts: UpdatePerpMarketMarginRatioAccounts,
        args: UpdatePerpMarketMarginRatioArguments,
    },
    UpdatePerpMarketHighLeverageMarginRatio {
        accounts: UpdatePerpMarketHighLeverageMarginRatioAccounts,
        args: UpdatePerpMarketHighLeverageMarginRatioArguments,
    },
    UpdatePerpMarketFundingPeriod {
        accounts: UpdatePerpMarketFundingPeriodAccounts,
        args: UpdatePerpMarketFundingPeriodArguments,
    },
    UpdatePerpMarketMaxImbalances {
        accounts: UpdatePerpMarketMaxImbalancesAccounts,
        args: UpdatePerpMarketMaxImbalancesArguments,
    },
    UpdatePerpMarketLiquidationFee {
        accounts: UpdatePerpMarketLiquidationFeeAccounts,
        args: UpdatePerpMarketLiquidationFeeArguments,
    },
    UpdateInsuranceFundUnstakingPeriod {
        accounts: UpdateInsuranceFundUnstakingPeriodAccounts,
        args: UpdateInsuranceFundUnstakingPeriodArguments,
    },
    UpdateSpotMarketPoolId {
        accounts: UpdateSpotMarketPoolIdAccounts,
        args: UpdateSpotMarketPoolIdArguments,
    },
    UpdateSpotMarketLiquidationFee {
        accounts: UpdateSpotMarketLiquidationFeeAccounts,
        args: UpdateSpotMarketLiquidationFeeArguments,
    },
    UpdateWithdrawGuardThreshold {
        accounts: UpdateWithdrawGuardThresholdAccounts,
        args: UpdateWithdrawGuardThresholdArguments,
    },
    UpdateSpotMarketIfFactor {
        accounts: UpdateSpotMarketIfFactorAccounts,
        args: UpdateSpotMarketIfFactorArguments,
    },
    UpdateSpotMarketRevenueSettlePeriod {
        accounts: UpdateSpotMarketRevenueSettlePeriodAccounts,
        args: UpdateSpotMarketRevenueSettlePeriodArguments,
    },
    UpdateSpotMarketStatus {
        accounts: UpdateSpotMarketStatusAccounts,
        args: UpdateSpotMarketStatusArguments,
    },
    UpdateSpotMarketPausedOperations {
        accounts: UpdateSpotMarketPausedOperationsAccounts,
        args: UpdateSpotMarketPausedOperationsArguments,
    },
    UpdateSpotMarketAssetTier {
        accounts: UpdateSpotMarketAssetTierAccounts,
        args: UpdateSpotMarketAssetTierArguments,
    },
    UpdateSpotMarketMarginWeights {
        accounts: UpdateSpotMarketMarginWeightsAccounts,
        args: UpdateSpotMarketMarginWeightsArguments,
    },
    UpdateSpotMarketBorrowRate {
        accounts: UpdateSpotMarketBorrowRateAccounts,
        args: UpdateSpotMarketBorrowRateArguments,
    },
    UpdateSpotMarketMaxTokenDeposits {
        accounts: UpdateSpotMarketMaxTokenDepositsAccounts,
        args: UpdateSpotMarketMaxTokenDepositsArguments,
    },
    UpdateSpotMarketMaxTokenBorrows {
        accounts: UpdateSpotMarketMaxTokenBorrowsAccounts,
        args: UpdateSpotMarketMaxTokenBorrowsArguments,
    },
    UpdateSpotMarketScaleInitialAssetWeightStart {
        accounts: UpdateSpotMarketScaleInitialAssetWeightStartAccounts,
        args: UpdateSpotMarketScaleInitialAssetWeightStartArguments,
    },
    UpdateSpotMarketOracle {
        accounts: UpdateSpotMarketOracleAccounts,
        args: UpdateSpotMarketOracleArguments,
    },
    UpdateSpotMarketStepSizeAndTickSize {
        accounts: UpdateSpotMarketStepSizeAndTickSizeAccounts,
        args: UpdateSpotMarketStepSizeAndTickSizeArguments,
    },
    UpdateSpotMarketMinOrderSize {
        accounts: UpdateSpotMarketMinOrderSizeAccounts,
        args: UpdateSpotMarketMinOrderSizeArguments,
    },
    UpdateSpotMarketOrdersEnabled {
        accounts: UpdateSpotMarketOrdersEnabledAccounts,
        args: UpdateSpotMarketOrdersEnabledArguments,
    },
    UpdateSpotMarketIfPausedOperations {
        accounts: UpdateSpotMarketIfPausedOperationsAccounts,
        args: UpdateSpotMarketIfPausedOperationsArguments,
    },
    UpdateSpotMarketName {
        accounts: UpdateSpotMarketNameAccounts,
        args: UpdateSpotMarketNameArguments,
    },
    UpdatePerpMarketStatus {
        accounts: UpdatePerpMarketStatusAccounts,
        args: UpdatePerpMarketStatusArguments,
    },
    UpdatePerpMarketPausedOperations {
        accounts: UpdatePerpMarketPausedOperationsAccounts,
        args: UpdatePerpMarketPausedOperationsArguments,
    },
    UpdatePerpMarketContractTier {
        accounts: UpdatePerpMarketContractTierAccounts,
        args: UpdatePerpMarketContractTierArguments,
    },
    UpdatePerpMarketImfFactor {
        accounts: UpdatePerpMarketImfFactorAccounts,
        args: UpdatePerpMarketImfFactorArguments,
    },
    UpdatePerpMarketUnrealizedAssetWeight {
        accounts: UpdatePerpMarketUnrealizedAssetWeightAccounts,
        args: UpdatePerpMarketUnrealizedAssetWeightArguments,
    },
    UpdatePerpMarketConcentrationCoef {
        accounts: UpdatePerpMarketConcentrationCoefAccounts,
        args: UpdatePerpMarketConcentrationCoefArguments,
    },
    UpdatePerpMarketCurveUpdateIntensity {
        accounts: UpdatePerpMarketCurveUpdateIntensityAccounts,
        args: UpdatePerpMarketCurveUpdateIntensityArguments,
    },
    UpdatePerpMarketTargetBaseAssetAmountPerLp {
        accounts: UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts,
        args: UpdatePerpMarketTargetBaseAssetAmountPerLpArguments,
    },
    UpdatePerpMarketPerLpBase {
        accounts: UpdatePerpMarketPerLpBaseAccounts,
        args: UpdatePerpMarketPerLpBaseArguments,
    },
    UpdateLpCooldownTime {
        accounts: UpdateLpCooldownTimeAccounts,
        args: UpdateLpCooldownTimeArguments,
    },
    UpdatePerpFeeStructure {
        accounts: UpdatePerpFeeStructureAccounts,
        args: UpdatePerpFeeStructureArguments,
    },
    UpdateSpotFeeStructure {
        accounts: UpdateSpotFeeStructureAccounts,
        args: UpdateSpotFeeStructureArguments,
    },
    UpdateInitialPctToLiquidate {
        accounts: UpdateInitialPctToLiquidateAccounts,
        args: UpdateInitialPctToLiquidateArguments,
    },
    UpdateLiquidationDuration {
        accounts: UpdateLiquidationDurationAccounts,
        args: UpdateLiquidationDurationArguments,
    },
    UpdateLiquidationMarginBufferRatio {
        accounts: UpdateLiquidationMarginBufferRatioAccounts,
        args: UpdateLiquidationMarginBufferRatioArguments,
    },
    UpdateOracleGuardRails {
        accounts: UpdateOracleGuardRailsAccounts,
        args: UpdateOracleGuardRailsArguments,
    },
    UpdateStateSettlementDuration {
        accounts: UpdateStateSettlementDurationAccounts,
        args: UpdateStateSettlementDurationArguments,
    },
    UpdateStateMaxNumberOfSubAccounts {
        accounts: UpdateStateMaxNumberOfSubAccountsAccounts,
        args: UpdateStateMaxNumberOfSubAccountsArguments,
    },
    UpdateStateMaxInitializeUserFee {
        accounts: UpdateStateMaxInitializeUserFeeAccounts,
        args: UpdateStateMaxInitializeUserFeeArguments,
    },
    UpdatePerpMarketOracle {
        accounts: UpdatePerpMarketOracleAccounts,
        args: UpdatePerpMarketOracleArguments,
    },
    UpdatePerpMarketBaseSpread {
        accounts: UpdatePerpMarketBaseSpreadAccounts,
        args: UpdatePerpMarketBaseSpreadArguments,
    },
    UpdateAmmJitIntensity {
        accounts: UpdateAmmJitIntensityAccounts,
        args: UpdateAmmJitIntensityArguments,
    },
    UpdatePerpMarketMaxSpread {
        accounts: UpdatePerpMarketMaxSpreadAccounts,
        args: UpdatePerpMarketMaxSpreadArguments,
    },
    UpdatePerpMarketStepSizeAndTickSize {
        accounts: UpdatePerpMarketStepSizeAndTickSizeAccounts,
        args: UpdatePerpMarketStepSizeAndTickSizeArguments,
    },
    UpdatePerpMarketName {
        accounts: UpdatePerpMarketNameAccounts,
        args: UpdatePerpMarketNameArguments,
    },
    UpdatePerpMarketMinOrderSize {
        accounts: UpdatePerpMarketMinOrderSizeAccounts,
        args: UpdatePerpMarketMinOrderSizeArguments,
    },
    UpdatePerpMarketMaxSlippageRatio {
        accounts: UpdatePerpMarketMaxSlippageRatioAccounts,
        args: UpdatePerpMarketMaxSlippageRatioArguments,
    },
    UpdatePerpMarketMaxFillReserveFraction {
        accounts: UpdatePerpMarketMaxFillReserveFractionAccounts,
        args: UpdatePerpMarketMaxFillReserveFractionArguments,
    },
    UpdatePerpMarketMaxOpenInterest {
        accounts: UpdatePerpMarketMaxOpenInterestAccounts,
        args: UpdatePerpMarketMaxOpenInterestArguments,
    },
    UpdatePerpMarketNumberOfUsers {
        accounts: UpdatePerpMarketNumberOfUsersAccounts,
        args: UpdatePerpMarketNumberOfUsersArguments,
    },
    UpdatePerpMarketFeeAdjustment {
        accounts: UpdatePerpMarketFeeAdjustmentAccounts,
        args: UpdatePerpMarketFeeAdjustmentArguments,
    },
    UpdateSpotMarketFeeAdjustment {
        accounts: UpdateSpotMarketFeeAdjustmentAccounts,
        args: UpdateSpotMarketFeeAdjustmentArguments,
    },
    UpdatePerpMarketFuel {
        accounts: UpdatePerpMarketFuelAccounts,
        args: UpdatePerpMarketFuelArguments,
    },
    UpdatePerpMarketProtectedMakerParams {
        accounts: UpdatePerpMarketProtectedMakerParamsAccounts,
        args: UpdatePerpMarketProtectedMakerParamsArguments,
    },
    UpdatePerpMarketTakerSpeedBumpOverride {
        accounts: UpdatePerpMarketTakerSpeedBumpOverrideAccounts,
        args: UpdatePerpMarketTakerSpeedBumpOverrideArguments,
    },
    UpdatePerpMarketAmmSpreadAdjustment {
        accounts: UpdatePerpMarketAmmSpreadAdjustmentAccounts,
        args: UpdatePerpMarketAmmSpreadAdjustmentArguments,
    },
    UpdatePerpMarketOracleSlotDelayOverride {
        accounts: UpdatePerpMarketOracleSlotDelayOverrideAccounts,
        args: UpdatePerpMarketOracleSlotDelayOverrideArguments,
    },
    UpdateSpotMarketFuel {
        accounts: UpdateSpotMarketFuelAccounts,
        args: UpdateSpotMarketFuelArguments,
    },
    InitUserFuel {
        accounts: InitUserFuelAccounts,
        args: InitUserFuelArguments,
    },
    UpdateAdmin {
        accounts: UpdateAdminAccounts,
        args: UpdateAdminArguments,
    },
    UpdateWhitelistMint {
        accounts: UpdateWhitelistMintAccounts,
        args: UpdateWhitelistMintArguments,
    },
    UpdateDiscountMint {
        accounts: UpdateDiscountMintAccounts,
        args: UpdateDiscountMintArguments,
    },
    UpdateExchangeStatus {
        accounts: UpdateExchangeStatusAccounts,
        args: UpdateExchangeStatusArguments,
    },
    UpdatePerpAuctionDuration {
        accounts: UpdatePerpAuctionDurationAccounts,
        args: UpdatePerpAuctionDurationArguments,
    },
    UpdateSpotAuctionDuration {
        accounts: UpdateSpotAuctionDurationAccounts,
        args: UpdateSpotAuctionDurationArguments,
    },
    InitializeProtocolIfSharesTransferConfig {
        accounts: InitializeProtocolIfSharesTransferConfigAccounts,
        args: InitializeProtocolIfSharesTransferConfigArguments,
    },
    UpdateProtocolIfSharesTransferConfig {
        accounts: UpdateProtocolIfSharesTransferConfigAccounts,
        args: UpdateProtocolIfSharesTransferConfigArguments,
    },
    InitializePrelaunchOracle {
        accounts: InitializePrelaunchOracleAccounts,
        args: InitializePrelaunchOracleArguments,
    },
    UpdatePrelaunchOracleParams {
        accounts: UpdatePrelaunchOracleParamsAccounts,
        args: UpdatePrelaunchOracleParamsArguments,
    },
    DeletePrelaunchOracle {
        accounts: DeletePrelaunchOracleAccounts,
        args: DeletePrelaunchOracleArguments,
    },
    InitializePythPullOracle {
        accounts: InitializePythPullOracleAccounts,
        args: InitializePythPullOracleArguments,
    },
    InitializePythLazerOracle {
        accounts: InitializePythLazerOracleAccounts,
        args: InitializePythLazerOracleArguments,
    },
    PostPythLazerOracleUpdate {
        accounts: PostPythLazerOracleUpdateAccounts,
        args: PostPythLazerOracleUpdateArguments,
    },
    InitializeHighLeverageModeConfig {
        accounts: InitializeHighLeverageModeConfigAccounts,
        args: InitializeHighLeverageModeConfigArguments,
    },
    UpdateHighLeverageModeConfig {
        accounts: UpdateHighLeverageModeConfigAccounts,
        args: UpdateHighLeverageModeConfigArguments,
    },
    InitializeProtectedMakerModeConfig {
        accounts: InitializeProtectedMakerModeConfigAccounts,
        args: InitializeProtectedMakerModeConfigArguments,
    },
    UpdateProtectedMakerModeConfig {
        accounts: UpdateProtectedMakerModeConfigAccounts,
        args: UpdateProtectedMakerModeConfigArguments,
    },
    AdminDeposit {
        accounts: AdminDepositAccounts,
        args: AdminDepositArguments,
    },
    InitializeIfRebalanceConfig {
        accounts: InitializeIfRebalanceConfigAccounts,
        args: InitializeIfRebalanceConfigArguments,
    },
    UpdateIfRebalanceConfig {
        accounts: UpdateIfRebalanceConfigAccounts,
        args: UpdateIfRebalanceConfigArguments,
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
            [111u8, 17u8, 185u8, 250u8, 60u8, 122u8, 38u8, 254u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeUserArguments {
                    sub_account_id,
                    name,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeUserAccounts {
                    user,
                    userStats,
                    state,
                    authority,
                    payer,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeUser { accounts, args });
            }
            [254u8, 243u8, 72u8, 98u8, 251u8, 130u8, 168u8, 213u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeUserStatsArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let userStats = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeUserStatsAccounts {
                    userStats,
                    state,
                    authority,
                    payer,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeUserStats { accounts, args });
            }
            [164u8, 99u8, 156u8, 126u8, 156u8, 57u8, 99u8, 180u8] => {
                let mut rdr: &[u8] = rest;
                let num_orders: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeSignedMsgUserOrdersArguments { num_orders };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let signedMsgUserOrders = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeSignedMsgUserOrdersAccounts {
                    signedMsgUserOrders,
                    authority,
                    payer,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeSignedMsgUserOrders { accounts, args });
            }
            [137u8, 10u8, 87u8, 150u8, 18u8, 115u8, 79u8, 168u8] => {
                let mut rdr: &[u8] = rest;
                let num_orders: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ResizeSignedMsgUserOrdersArguments { num_orders };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let signedMsgUserOrders = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResizeSignedMsgUserOrdersAccounts {
                    signedMsgUserOrders,
                    authority,
                    user,
                    payer,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::ResizeSignedMsgUserOrders { accounts, args });
            }
            [40u8, 132u8, 96u8, 219u8, 184u8, 193u8, 80u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let delegates: Vec<[u8; 32usize]> =
                    <Vec<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeSignedMsgWsDelegatesArguments { delegates };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let signedMsgWsDelegates = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeSignedMsgWsDelegatesAccounts {
                    signedMsgWsDelegates,
                    authority,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeSignedMsgWsDelegates { accounts, args });
            }
            [252u8, 202u8, 252u8, 219u8, 179u8, 27u8, 84u8, 138u8] => {
                let mut rdr: &[u8] = rest;
                let delegate: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let add: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ChangeSignedMsgWsDelegateStatusArguments { delegate, add };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let signedMsgWsDelegates = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ChangeSignedMsgWsDelegateStatusAccounts {
                    signedMsgWsDelegates,
                    authority,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::ChangeSignedMsgWsDelegateStatus { accounts, args });
            }
            [88u8, 223u8, 132u8, 161u8, 208u8, 88u8, 142u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeFuelOverflowArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let fuelOverflow = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeFuelOverflowAccounts {
                    fuelOverflow,
                    userStats,
                    authority,
                    payer,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeFuelOverflow { accounts, args });
            }
            [175u8, 107u8, 19u8, 56u8, 165u8, 241u8, 43u8, 69u8] => {
                let mut rdr: &[u8] = rest;
                let args = SweepFuelArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let fuelOverflow = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SweepFuelAccounts {
                    fuelOverflow,
                    userStats,
                    authority,
                    signer,
                    remaining,
                };
                return Ok(Instruction::SweepFuel { accounts, args });
            }
            [199u8, 122u8, 192u8, 255u8, 32u8, 99u8, 63u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResetFuelSeasonArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResetFuelSeasonAccounts {
                    userStats,
                    authority,
                    state,
                    admin,
                    remaining,
                };
                return Ok(Instruction::ResetFuelSeason { accounts, args });
            }
            [235u8, 126u8, 231u8, 10u8, 42u8, 164u8, 26u8, 61u8] => {
                let mut rdr: &[u8] = rest;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeReferrerNameArguments { name };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let referrerName = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeReferrerNameAccounts {
                    referrerName,
                    user,
                    userStats,
                    authority,
                    payer,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeReferrerName { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositArguments {
                    market_index,
                    amount,
                    reduce_only,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    state,
                    user,
                    userStats,
                    authority,
                    spotMarketVault,
                    userTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawArguments {
                    market_index,
                    amount,
                    reduce_only,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    state,
                    user,
                    userStats,
                    authority,
                    spotMarketVault,
                    driftSigner,
                    userTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            [20u8, 20u8, 147u8, 223u8, 41u8, 63u8, 204u8, 111u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = TransferDepositArguments {
                    market_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let fromUser = keys.next().unwrap().clone();
                let toUser = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferDepositAccounts {
                    fromUser,
                    toUser,
                    userStats,
                    authority,
                    state,
                    spotMarketVault,
                    remaining,
                };
                return Ok(Instruction::TransferDeposit { accounts, args });
            }
            [197u8, 103u8, 154u8, 25u8, 107u8, 90u8, 60u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let deposit_from_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let deposit_to_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let borrow_from_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let borrow_to_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let deposit_amount: Option<u64> = parse_option(&mut rdr)?;
                let borrow_amount: Option<u64> = parse_option(&mut rdr)?;
                let args = TransferPoolsArguments {
                    deposit_from_market_index,
                    deposit_to_market_index,
                    borrow_from_market_index,
                    borrow_to_market_index,
                    deposit_amount,
                    borrow_amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let fromUser = keys.next().unwrap().clone();
                let toUser = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let depositFromSpotMarketVault = keys.next().unwrap().clone();
                let depositToSpotMarketVault = keys.next().unwrap().clone();
                let borrowFromSpotMarketVault = keys.next().unwrap().clone();
                let borrowToSpotMarketVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferPoolsAccounts {
                    fromUser,
                    toUser,
                    userStats,
                    authority,
                    state,
                    depositFromSpotMarketVault,
                    depositToSpotMarketVault,
                    borrowFromSpotMarketVault,
                    borrowToSpotMarketVault,
                    driftSigner,
                    remaining,
                };
                return Ok(Instruction::TransferPools { accounts, args });
            }
            [23u8, 172u8, 188u8, 168u8, 134u8, 210u8, 3u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: Option<i64> = parse_option(&mut rdr)?;
                let args = TransferPerpPositionArguments {
                    market_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let fromUser = keys.next().unwrap().clone();
                let toUser = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferPerpPositionAccounts {
                    fromUser,
                    toUser,
                    userStats,
                    authority,
                    state,
                    remaining,
                };
                return Ok(Instruction::TransferPerpPosition { accounts, args });
            }
            [69u8, 161u8, 93u8, 202u8, 120u8, 126u8, 76u8, 185u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let args = PlacePerpOrderArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlacePerpOrderAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlacePerpOrder { accounts, args });
            }
            [95u8, 129u8, 237u8, 240u8, 8u8, 49u8, 223u8, 132u8] => {
                let mut rdr: &[u8] = rest;
                let order_id: Option<u32> = parse_option(&mut rdr)?;
                let args = CancelOrderArguments { order_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelOrderAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::CancelOrder { accounts, args });
            }
            [107u8, 211u8, 250u8, 133u8, 18u8, 37u8, 57u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CancelOrderByUserIdArguments { user_order_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelOrderByUserIdAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::CancelOrderByUserId { accounts, args });
            }
            [238u8, 225u8, 95u8, 158u8, 227u8, 103u8, 8u8, 194u8] => {
                let mut rdr: &[u8] = rest;
                let market_type: Option<MarketType> = parse_option(&mut rdr)?;
                let market_index: Option<u16> = parse_option(&mut rdr)?;
                let direction: Option<PositionDirection> = parse_option(&mut rdr)?;
                let args = CancelOrdersArguments {
                    market_type,
                    market_index,
                    direction,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelOrdersAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::CancelOrders { accounts, args });
            }
            [134u8, 19u8, 144u8, 165u8, 94u8, 240u8, 210u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let order_ids: Vec<u32> =
                    <Vec<u32> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CancelOrdersByIdsArguments { order_ids };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelOrdersByIdsAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::CancelOrdersByIds { accounts, args });
            }
            [47u8, 124u8, 117u8, 255u8, 201u8, 197u8, 130u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let order_id: Option<u32> = parse_option(&mut rdr)?;
                let direction: Option<PositionDirection> = parse_option(&mut rdr)?;
                let base_asset_amount: Option<u64> = parse_option(&mut rdr)?;
                let price: Option<u64> = parse_option(&mut rdr)?;
                let reduce_only: Option<bool> = parse_option(&mut rdr)?;
                let post_only: Option<PostOnlyParam> = parse_option(&mut rdr)?;
                let bit_flags: Option<u8> = parse_option(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: Option<OrderTriggerCondition> = parse_option(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let policy: Option<u8> = parse_option(&mut rdr)?;
                let modify_order_params = ModifyOrderParams {
                    direction,
                    base_asset_amount,
                    price,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                    policy,
                };
                let args = ModifyOrderArguments {
                    order_id,
                    modify_order_params,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ModifyOrderAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::ModifyOrder { accounts, args });
            }
            [158u8, 77u8, 4u8, 253u8, 252u8, 194u8, 161u8, 179u8] => {
                let mut rdr: &[u8] = rest;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: Option<PositionDirection> = parse_option(&mut rdr)?;
                let base_asset_amount: Option<u64> = parse_option(&mut rdr)?;
                let price: Option<u64> = parse_option(&mut rdr)?;
                let reduce_only: Option<bool> = parse_option(&mut rdr)?;
                let post_only: Option<PostOnlyParam> = parse_option(&mut rdr)?;
                let bit_flags: Option<u8> = parse_option(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: Option<OrderTriggerCondition> = parse_option(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let policy: Option<u8> = parse_option(&mut rdr)?;
                let modify_order_params = ModifyOrderParams {
                    direction,
                    base_asset_amount,
                    price,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                    policy,
                };
                let args = ModifyOrderByUserIdArguments {
                    user_order_id,
                    modify_order_params,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ModifyOrderByUserIdAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::ModifyOrderByUserId { accounts, args });
            }
            [213u8, 51u8, 1u8, 187u8, 108u8, 220u8, 230u8, 224u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let success_condition: Option<u32> = parse_option(&mut rdr)?;
                let args = PlaceAndTakePerpOrderArguments {
                    params,
                    success_condition,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceAndTakePerpOrderAccounts {
                    state,
                    user,
                    userStats,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceAndTakePerpOrder { accounts, args });
            }
            [149u8, 117u8, 11u8, 237u8, 47u8, 95u8, 89u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let taker_order_id: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PlaceAndMakePerpOrderArguments {
                    params,
                    taker_order_id,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let taker = keys.next().unwrap().clone();
                let takerStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceAndMakePerpOrderAccounts {
                    state,
                    user,
                    userStats,
                    taker,
                    takerStats,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceAndMakePerpOrder { accounts, args });
            }
            [16u8, 26u8, 123u8, 131u8, 94u8, 29u8, 175u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let signed_msg_order_uuid: [u8; 8usize] =
                    <[u8; 8usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PlaceAndMakeSignedMsgPerpOrderArguments {
                    params,
                    signed_msg_order_uuid,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let taker = keys.next().unwrap().clone();
                let takerStats = keys.next().unwrap().clone();
                let takerSignedMsgUserOrders = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceAndMakeSignedMsgPerpOrderAccounts {
                    state,
                    user,
                    userStats,
                    taker,
                    takerStats,
                    takerSignedMsgUserOrders,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceAndMakeSignedMsgPerpOrder { accounts, args });
            }
            [32u8, 79u8, 101u8, 139u8, 25u8, 6u8, 98u8, 15u8] => {
                let mut rdr: &[u8] = rest;
                let signed_msg_order_params_message_bytes: Vec<u8> =
                    <Vec<u8> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let is_delegate_signer: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PlaceSignedMsgTakerOrderArguments {
                    signed_msg_order_params_message_bytes,
                    is_delegate_signer,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let signedMsgUserOrders = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let ixSysvar = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceSignedMsgTakerOrderAccounts {
                    state,
                    user,
                    userStats,
                    signedMsgUserOrders,
                    authority,
                    ixSysvar,
                    remaining,
                };
                return Ok(Instruction::PlaceSignedMsgTakerOrder { accounts, args });
            }
            [45u8, 79u8, 81u8, 160u8, 248u8, 90u8, 91u8, 220u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let args = PlaceSpotOrderArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceSpotOrderAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceSpotOrder { accounts, args });
            }
            [191u8, 3u8, 138u8, 71u8, 114u8, 198u8, 202u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let fulfillment_type: Option<SpotFulfillmentType> = parse_option(&mut rdr)?;
                let maker_order_id: Option<u32> = parse_option(&mut rdr)?;
                let args = PlaceAndTakeSpotOrderArguments {
                    params,
                    fulfillment_type,
                    maker_order_id,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceAndTakeSpotOrderAccounts {
                    state,
                    user,
                    userStats,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceAndTakeSpotOrder { accounts, args });
            }
            [149u8, 158u8, 85u8, 66u8, 239u8, 9u8, 243u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let order_type: OrderType =
                    <OrderType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_type: MarketType =
                    <MarketType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let direction: PositionDirection =
                    <PositionDirection as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_order_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let post_only: PostOnlyParam =
                    <PostOnlyParam as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bit_flags: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_ts: Option<i64> = parse_option(&mut rdr)?;
                let trigger_price: Option<u64> = parse_option(&mut rdr)?;
                let trigger_condition: OrderTriggerCondition =
                    <OrderTriggerCondition as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_price_offset: Option<i32> = parse_option(&mut rdr)?;
                let auction_duration: Option<u8> = parse_option(&mut rdr)?;
                let auction_start_price: Option<i64> = parse_option(&mut rdr)?;
                let auction_end_price: Option<i64> = parse_option(&mut rdr)?;
                let params = OrderParams {
                    order_type,
                    market_type,
                    direction,
                    user_order_id,
                    base_asset_amount,
                    price,
                    market_index,
                    reduce_only,
                    post_only,
                    bit_flags,
                    max_ts,
                    trigger_price,
                    trigger_condition,
                    oracle_price_offset,
                    auction_duration,
                    auction_start_price,
                    auction_end_price,
                };
                let taker_order_id: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let fulfillment_type: Option<SpotFulfillmentType> = parse_option(&mut rdr)?;
                let args = PlaceAndMakeSpotOrderArguments {
                    params,
                    taker_order_id,
                    fulfillment_type,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let taker = keys.next().unwrap().clone();
                let takerStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceAndMakeSpotOrderAccounts {
                    state,
                    user,
                    userStats,
                    taker,
                    takerStats,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceAndMakeSpotOrder { accounts, args });
            }
            [60u8, 63u8, 50u8, 123u8, 12u8, 197u8, 60u8, 190u8] => {
                let mut rdr: &[u8] = rest;
                let params: Vec<OrderParams> =
                    <Vec<OrderParams> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PlaceOrdersArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PlaceOrdersAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::PlaceOrders { accounts, args });
            }
            [174u8, 109u8, 228u8, 1u8, 242u8, 105u8, 232u8, 105u8] => {
                let mut rdr: &[u8] = rest;
                let in_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let out_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount_in: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = BeginSwapArguments {
                    in_market_index,
                    out_market_index,
                    amount_in,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 11usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let outSpotMarketVault = keys.next().unwrap().clone();
                let inSpotMarketVault = keys.next().unwrap().clone();
                let outTokenAccount = keys.next().unwrap().clone();
                let inTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BeginSwapAccounts {
                    state,
                    user,
                    userStats,
                    authority,
                    outSpotMarketVault,
                    inSpotMarketVault,
                    outTokenAccount,
                    inTokenAccount,
                    tokenProgram,
                    driftSigner,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::BeginSwap { accounts, args });
            }
            [177u8, 184u8, 27u8, 193u8, 34u8, 13u8, 210u8, 145u8] => {
                let mut rdr: &[u8] = rest;
                let in_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let out_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let limit_price: Option<u64> = parse_option(&mut rdr)?;
                let reduce_only: Option<SwapReduceOnly> = parse_option(&mut rdr)?;
                let args = EndSwapArguments {
                    in_market_index,
                    out_market_index,
                    limit_price,
                    reduce_only,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 11usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let outSpotMarketVault = keys.next().unwrap().clone();
                let inSpotMarketVault = keys.next().unwrap().clone();
                let outTokenAccount = keys.next().unwrap().clone();
                let inTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EndSwapAccounts {
                    state,
                    user,
                    userStats,
                    authority,
                    outSpotMarketVault,
                    inSpotMarketVault,
                    outTokenAccount,
                    inTokenAccount,
                    tokenProgram,
                    driftSigner,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::EndSwap { accounts, args });
            }
            [56u8, 209u8, 56u8, 197u8, 119u8, 254u8, 188u8, 117u8] => {
                let mut rdr: &[u8] = rest;
                let n_shares: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AddPerpLpSharesArguments {
                    n_shares,
                    market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddPerpLpSharesAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::AddPerpLpShares { accounts, args });
            }
            [213u8, 89u8, 217u8, 18u8, 160u8, 55u8, 53u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let shares_to_burn: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RemovePerpLpSharesArguments {
                    shares_to_burn,
                    market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemovePerpLpSharesAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::RemovePerpLpShares { accounts, args });
            }
            [83u8, 254u8, 253u8, 137u8, 59u8, 122u8, 68u8, 156u8] => {
                let mut rdr: &[u8] = rest;
                let shares_to_burn: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RemovePerpLpSharesInExpiringMarketArguments {
                    shares_to_burn,
                    market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemovePerpLpSharesInExpiringMarketAccounts {
                    state,
                    user,
                    remaining,
                };
                return Ok(Instruction::RemovePerpLpSharesInExpiringMarket { accounts, args });
            }
            [135u8, 25u8, 185u8, 56u8, 165u8, 53u8, 34u8, 136u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserNameArguments {
                    sub_account_id,
                    name,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserNameAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserName { accounts, args });
            }
            [21u8, 221u8, 140u8, 187u8, 32u8, 129u8, 11u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let margin_ratio: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserCustomMarginRatioArguments {
                    sub_account_id,
                    margin_ratio,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserCustomMarginRatioAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserCustomMarginRatio { accounts, args });
            }
            [194u8, 92u8, 204u8, 223u8, 246u8, 188u8, 31u8, 203u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let margin_trading_enabled: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserMarginTradingEnabledArguments {
                    sub_account_id,
                    margin_trading_enabled,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserMarginTradingEnabledAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserMarginTradingEnabled { accounts, args });
            }
            [219u8, 86u8, 73u8, 106u8, 56u8, 218u8, 128u8, 109u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let pool_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserPoolIdArguments {
                    sub_account_id,
                    pool_id,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserPoolIdAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserPoolId { accounts, args });
            }
            [139u8, 205u8, 141u8, 141u8, 113u8, 36u8, 94u8, 187u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let delegate: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserDelegateArguments {
                    sub_account_id,
                    delegate,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserDelegateAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserDelegate { accounts, args });
            }
            [199u8, 71u8, 42u8, 67u8, 144u8, 19u8, 86u8, 109u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserReduceOnlyArguments {
                    sub_account_id,
                    reduce_only,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserReduceOnlyAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserReduceOnly { accounts, args });
            }
            [66u8, 80u8, 107u8, 186u8, 27u8, 242u8, 66u8, 95u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let advanced_lp: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserAdvancedLpArguments {
                    sub_account_id,
                    advanced_lp,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserAdvancedLpAccounts {
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateUserAdvancedLp { accounts, args });
            }
            [114u8, 39u8, 123u8, 198u8, 187u8, 25u8, 90u8, 219u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let protected_maker_orders: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserProtectedMakerOrdersArguments {
                    sub_account_id,
                    protected_maker_orders,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let protectedMakerModeConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserProtectedMakerOrdersAccounts {
                    state,
                    user,
                    authority,
                    protectedMakerModeConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateUserProtectedMakerOrders { accounts, args });
            }
            [186u8, 85u8, 17u8, 249u8, 219u8, 231u8, 98u8, 251u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeleteUserArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeleteUserAccounts {
                    user,
                    userStats,
                    state,
                    authority,
                    remaining,
                };
                return Ok(Instruction::DeleteUser { accounts, args });
            }
            [2u8, 241u8, 195u8, 172u8, 227u8, 24u8, 254u8, 158u8] => {
                let mut rdr: &[u8] = rest;
                let args = ForceDeleteUserArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let keeper = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ForceDeleteUserAccounts {
                    user,
                    userStats,
                    state,
                    authority,
                    keeper,
                    driftSigner,
                    remaining,
                };
                return Ok(Instruction::ForceDeleteUser { accounts, args });
            }
            [221u8, 247u8, 128u8, 253u8, 212u8, 254u8, 46u8, 153u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeleteSignedMsgUserOrdersArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let signedMsgUserOrders = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeleteSignedMsgUserOrdersAccounts {
                    signedMsgUserOrders,
                    state,
                    authority,
                    remaining,
                };
                return Ok(Instruction::DeleteSignedMsgUserOrders { accounts, args });
            }
            [218u8, 200u8, 19u8, 197u8, 227u8, 89u8, 192u8, 22u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReclaimRentArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReclaimRentAccounts {
                    user,
                    userStats,
                    state,
                    authority,
                    rent,
                    remaining,
                };
                return Ok(Instruction::ReclaimRent { accounts, args });
            }
            [231u8, 24u8, 230u8, 112u8, 201u8, 173u8, 73u8, 184u8] => {
                let mut rdr: &[u8] = rest;
                let sub_account_id: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = EnableUserHighLeverageModeArguments { sub_account_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let highLeverageModeConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableUserHighLeverageModeAccounts {
                    state,
                    user,
                    authority,
                    highLeverageModeConfig,
                    remaining,
                };
                return Ok(Instruction::EnableUserHighLeverageMode { accounts, args });
            }
            [13u8, 188u8, 248u8, 103u8, 134u8, 217u8, 106u8, 240u8] => {
                let mut rdr: &[u8] = rest;
                let order_id: Option<u32> = parse_option(&mut rdr)?;
                let maker_order_id: Option<u32> = parse_option(&mut rdr)?;
                let args = FillPerpOrderArguments {
                    order_id,
                    maker_order_id,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let fillerStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FillPerpOrderAccounts {
                    state,
                    authority,
                    filler,
                    fillerStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::FillPerpOrder { accounts, args });
            }
            [236u8, 238u8, 176u8, 69u8, 239u8, 10u8, 181u8, 193u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevertFillArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let fillerStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevertFillAccounts {
                    state,
                    authority,
                    filler,
                    fillerStats,
                    remaining,
                };
                return Ok(Instruction::RevertFill { accounts, args });
            }
            [212u8, 206u8, 130u8, 173u8, 21u8, 34u8, 199u8, 40u8] => {
                let mut rdr: &[u8] = rest;
                let order_id: Option<u32> = parse_option(&mut rdr)?;
                let fulfillment_type: Option<SpotFulfillmentType> = parse_option(&mut rdr)?;
                let maker_order_id: Option<u32> = parse_option(&mut rdr)?;
                let args = FillSpotOrderArguments {
                    order_id,
                    fulfillment_type,
                    maker_order_id,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let fillerStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FillSpotOrderAccounts {
                    state,
                    authority,
                    filler,
                    fillerStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::FillSpotOrder { accounts, args });
            }
            [63u8, 112u8, 51u8, 233u8, 232u8, 47u8, 240u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let order_id: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = TriggerOrderArguments { order_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TriggerOrderAccounts {
                    state,
                    authority,
                    filler,
                    user,
                    remaining,
                };
                return Ok(Instruction::TriggerOrder { accounts, args });
            }
            [64u8, 181u8, 196u8, 63u8, 222u8, 72u8, 64u8, 232u8] => {
                let mut rdr: &[u8] = rest;
                let args = ForceCancelOrdersArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ForceCancelOrdersAccounts {
                    state,
                    authority,
                    filler,
                    user,
                    remaining,
                };
                return Ok(Instruction::ForceCancelOrders { accounts, args });
            }
            [253u8, 133u8, 67u8, 22u8, 103u8, 161u8, 20u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateUserIdleArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserIdleAccounts {
                    state,
                    authority,
                    filler,
                    user,
                    remaining,
                };
                return Ok(Instruction::UpdateUserIdle { accounts, args });
            }
            [162u8, 21u8, 35u8, 251u8, 32u8, 57u8, 161u8, 210u8] => {
                let mut rdr: &[u8] = rest;
                let args = LogUserBalancesArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LogUserBalancesAccounts {
                    state,
                    authority,
                    user,
                    remaining,
                };
                return Ok(Instruction::LogUserBalances { accounts, args });
            }
            [183u8, 155u8, 45u8, 0u8, 226u8, 85u8, 213u8, 69u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableUserHighLeverageModeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let highLeverageModeConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableUserHighLeverageModeAccounts {
                    state,
                    authority,
                    user,
                    highLeverageModeConfig,
                    remaining,
                };
                return Ok(Instruction::DisableUserHighLeverageMode { accounts, args });
            }
            [88u8, 175u8, 201u8, 190u8, 222u8, 100u8, 143u8, 57u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateUserFuelBonusArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserFuelBonusAccounts {
                    state,
                    authority,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::UpdateUserFuelBonus { accounts, args });
            }
            [174u8, 154u8, 72u8, 42u8, 191u8, 148u8, 145u8, 205u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateUserStatsReferrerStatusArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserStatsReferrerStatusAccounts {
                    state,
                    authority,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::UpdateUserStatsReferrerStatus { accounts, args });
            }
            [104u8, 39u8, 65u8, 210u8, 250u8, 163u8, 100u8, 134u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateUserOpenOrdersCountArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserOpenOrdersCountAccounts {
                    state,
                    authority,
                    filler,
                    user,
                    remaining,
                };
                return Ok(Instruction::UpdateUserOpenOrdersCount { accounts, args });
            }
            [17u8, 164u8, 82u8, 45u8, 183u8, 86u8, 191u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let disable: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AdminDisableUpdatePerpBidAskTwapArguments { disable };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AdminDisableUpdatePerpBidAskTwapAccounts {
                    admin,
                    state,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::AdminDisableUpdatePerpBidAskTwap { accounts, args });
            }
            [43u8, 61u8, 234u8, 45u8, 15u8, 95u8, 152u8, 153u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SettlePnlArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettlePnlAccounts {
                    state,
                    user,
                    authority,
                    spotMarketVault,
                    remaining,
                };
                return Ok(Instruction::SettlePnl { accounts, args });
            }
            [127u8, 66u8, 117u8, 57u8, 40u8, 50u8, 152u8, 127u8] => {
                let mut rdr: &[u8] = rest;
                let market_indexes: Vec<u16> =
                    <Vec<u16> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let mode: SettlePnlMode =
                    <SettlePnlMode as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SettleMultiplePnlsArguments {
                    market_indexes,
                    mode,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettleMultiplePnlsAccounts {
                    state,
                    user,
                    authority,
                    spotMarketVault,
                    remaining,
                };
                return Ok(Instruction::SettleMultiplePnls { accounts, args });
            }
            [222u8, 90u8, 202u8, 94u8, 28u8, 45u8, 115u8, 183u8] => {
                let mut rdr: &[u8] = rest;
                let args = SettleFundingPaymentArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettleFundingPaymentAccounts {
                    state,
                    user,
                    remaining,
                };
                return Ok(Instruction::SettleFundingPayment { accounts, args });
            }
            [155u8, 231u8, 116u8, 113u8, 97u8, 229u8, 139u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SettleLpArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettleLpAccounts {
                    state,
                    user,
                    remaining,
                };
                return Ok(Instruction::SettleLp { accounts, args });
            }
            [120u8, 89u8, 11u8, 25u8, 122u8, 77u8, 72u8, 193u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SettleExpiredMarketArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettleExpiredMarketAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::SettleExpiredMarket { accounts, args });
            }
            [75u8, 35u8, 119u8, 247u8, 191u8, 18u8, 139u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liquidator_max_base_asset_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let limit_price: Option<u64> = parse_option(&mut rdr)?;
                let args = LiquidatePerpArguments {
                    market_index,
                    liquidator_max_base_asset_amount,
                    limit_price,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidatePerpAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::LiquidatePerp { accounts, args });
            }
            [95u8, 111u8, 124u8, 105u8, 86u8, 169u8, 187u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = LiquidatePerpWithFillArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidatePerpWithFillAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::LiquidatePerpWithFill { accounts, args });
            }
            [107u8, 0u8, 128u8, 41u8, 35u8, 229u8, 251u8, 18u8] => {
                let mut rdr: &[u8] = rest;
                let asset_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liability_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liquidator_max_liability_transfer: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let limit_price: Option<u64> = parse_option(&mut rdr)?;
                let args = LiquidateSpotArguments {
                    asset_market_index,
                    liability_market_index,
                    liquidator_max_liability_transfer,
                    limit_price,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateSpotAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::LiquidateSpot { accounts, args });
            }
            [12u8, 43u8, 176u8, 83u8, 156u8, 251u8, 117u8, 13u8] => {
                let mut rdr: &[u8] = rest;
                let asset_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liability_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let swap_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = LiquidateSpotWithSwapBeginArguments {
                    asset_market_index,
                    liability_market_index,
                    swap_amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 13usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let liabilitySpotMarketVault = keys.next().unwrap().clone();
                let assetSpotMarketVault = keys.next().unwrap().clone();
                let liabilityTokenAccount = keys.next().unwrap().clone();
                let assetTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateSpotWithSwapBeginAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    liabilitySpotMarketVault,
                    assetSpotMarketVault,
                    liabilityTokenAccount,
                    assetTokenAccount,
                    tokenProgram,
                    driftSigner,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::LiquidateSpotWithSwapBegin { accounts, args });
            }
            [142u8, 88u8, 163u8, 160u8, 223u8, 75u8, 55u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let asset_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liability_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = LiquidateSpotWithSwapEndArguments {
                    asset_market_index,
                    liability_market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 13usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let liabilitySpotMarketVault = keys.next().unwrap().clone();
                let assetSpotMarketVault = keys.next().unwrap().clone();
                let liabilityTokenAccount = keys.next().unwrap().clone();
                let assetTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateSpotWithSwapEndAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    liabilitySpotMarketVault,
                    assetSpotMarketVault,
                    liabilityTokenAccount,
                    assetTokenAccount,
                    tokenProgram,
                    driftSigner,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::LiquidateSpotWithSwapEnd { accounts, args });
            }
            [169u8, 17u8, 32u8, 90u8, 207u8, 148u8, 209u8, 27u8] => {
                let mut rdr: &[u8] = rest;
                let perp_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let spot_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liquidator_max_liability_transfer: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let limit_price: Option<u64> = parse_option(&mut rdr)?;
                let args = LiquidateBorrowForPerpPnlArguments {
                    perp_market_index,
                    spot_market_index,
                    liquidator_max_liability_transfer,
                    limit_price,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateBorrowForPerpPnlAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::LiquidateBorrowForPerpPnl { accounts, args });
            }
            [237u8, 75u8, 198u8, 235u8, 233u8, 186u8, 75u8, 35u8] => {
                let mut rdr: &[u8] = rest;
                let perp_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let spot_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liquidator_max_pnl_transfer: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let limit_price: Option<u64> = parse_option(&mut rdr)?;
                let args = LiquidatePerpPnlForDepositArguments {
                    perp_market_index,
                    spot_market_index,
                    liquidator_max_pnl_transfer,
                    limit_price,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidatePerpPnlForDepositAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::LiquidatePerpPnlForDeposit { accounts, args });
            }
            [106u8, 133u8, 160u8, 206u8, 193u8, 171u8, 192u8, 194u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetUserStatusToBeingLiquidatedArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetUserStatusToBeingLiquidatedAccounts {
                    state,
                    user,
                    authority,
                    remaining,
                };
                return Ok(Instruction::SetUserStatusToBeingLiquidated { accounts, args });
            }
            [168u8, 204u8, 68u8, 150u8, 159u8, 126u8, 95u8, 148u8] => {
                let mut rdr: &[u8] = rest;
                let spot_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let perp_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ResolvePerpPnlDeficitArguments {
                    spot_market_index,
                    perp_market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResolvePerpPnlDeficitAccounts {
                    state,
                    authority,
                    spotMarketVault,
                    insuranceFundVault,
                    driftSigner,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::ResolvePerpPnlDeficit { accounts, args });
            }
            [224u8, 16u8, 176u8, 214u8, 162u8, 213u8, 183u8, 222u8] => {
                let mut rdr: &[u8] = rest;
                let quote_spot_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ResolvePerpBankruptcyArguments {
                    quote_spot_market_index,
                    market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResolvePerpBankruptcyAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    spotMarketVault,
                    insuranceFundVault,
                    driftSigner,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::ResolvePerpBankruptcy { accounts, args });
            }
            [124u8, 194u8, 240u8, 254u8, 198u8, 213u8, 52u8, 122u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ResolveSpotBankruptcyArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidator = keys.next().unwrap().clone();
                let liquidatorStats = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResolveSpotBankruptcyAccounts {
                    state,
                    authority,
                    liquidator,
                    liquidatorStats,
                    user,
                    userStats,
                    spotMarketVault,
                    insuranceFundVault,
                    driftSigner,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::ResolveSpotBankruptcy { accounts, args });
            }
            [200u8, 120u8, 93u8, 136u8, 69u8, 38u8, 199u8, 159u8] => {
                let mut rdr: &[u8] = rest;
                let spot_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SettleRevenueToInsuranceFundArguments { spot_market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettleRevenueToInsuranceFundAccounts {
                    state,
                    spotMarket,
                    spotMarketVault,
                    driftSigner,
                    insuranceFundVault,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::SettleRevenueToInsuranceFund { accounts, args });
            }
            [201u8, 178u8, 116u8, 212u8, 166u8, 144u8, 72u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateFundingRateArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFundingRateAccounts {
                    state,
                    perpMarket,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::UpdateFundingRate { accounts, args });
            }
            [220u8, 132u8, 27u8, 27u8, 233u8, 220u8, 61u8, 219u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePrelaunchOracleArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePrelaunchOracleAccounts {
                    state,
                    perpMarket,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::UpdatePrelaunchOracle { accounts, args });
            }
            [247u8, 23u8, 255u8, 65u8, 212u8, 90u8, 221u8, 194u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePerpBidAskTwapArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let keeperStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpBidAskTwapAccounts {
                    state,
                    perpMarket,
                    oracle,
                    keeperStats,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpBidAskTwap { accounts, args });
            }
            [39u8, 166u8, 139u8, 243u8, 158u8, 165u8, 155u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateSpotMarketCumulativeInterestArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let spotMarketVault = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketCumulativeInterestAccounts {
                    state,
                    spotMarket,
                    oracle,
                    spotMarketVault,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketCumulativeInterest { accounts, args });
            }
            [201u8, 106u8, 217u8, 253u8, 4u8, 175u8, 228u8, 97u8] => {
                let mut rdr: &[u8] = rest;
                let market_indexes: [u16; 5usize] =
                    <[u16; 5usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateAmmsArguments { market_indexes };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAmmsAccounts {
                    state,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateAmms { accounts, args });
            }
            [208u8, 11u8, 211u8, 159u8, 226u8, 24u8, 11u8, 247u8] => {
                let mut rdr: &[u8] = rest;
                let expiry_ts: i64 = <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketExpiryArguments { expiry_ts };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketExpiryAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketExpiry { accounts, args });
            }
            [251u8, 101u8, 156u8, 7u8, 2u8, 63u8, 30u8, 23u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateUserQuoteAssetInsuranceStakeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserQuoteAssetInsuranceStakeAccounts {
                    state,
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    signer,
                    insuranceFundVault,
                    remaining,
                };
                return Ok(Instruction::UpdateUserQuoteAssetInsuranceStake { accounts, args });
            }
            [143u8, 99u8, 235u8, 187u8, 20u8, 159u8, 184u8, 84u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateUserGovTokenInsuranceStakeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserGovTokenInsuranceStakeAccounts {
                    state,
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    signer,
                    insuranceFundVault,
                    remaining,
                };
                return Ok(Instruction::UpdateUserGovTokenInsuranceStake { accounts, args });
            }
            [129u8, 185u8, 243u8, 183u8, 228u8, 111u8, 64u8, 175u8] => {
                let mut rdr: &[u8] = rest;
                let gov_stake_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateUserGovTokenInsuranceStakeDevnetArguments { gov_stake_amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let userStats = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateUserGovTokenInsuranceStakeDevnetAccounts {
                    userStats,
                    signer,
                    remaining,
                };
                return Ok(Instruction::UpdateUserGovTokenInsuranceStakeDevnet { accounts, args });
            }
            [187u8, 179u8, 243u8, 70u8, 248u8, 90u8, 92u8, 147u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeInsuranceFundStakeArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeInsuranceFundStakeAccounts {
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    state,
                    authority,
                    payer,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeInsuranceFundStake { accounts, args });
            }
            [251u8, 144u8, 115u8, 11u8, 222u8, 47u8, 62u8, 236u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AddInsuranceFundStakeArguments {
                    market_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddInsuranceFundStakeAccounts {
                    state,
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    authority,
                    spotMarketVault,
                    insuranceFundVault,
                    driftSigner,
                    userTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AddInsuranceFundStake { accounts, args });
            }
            [142u8, 70u8, 204u8, 92u8, 73u8, 106u8, 180u8, 52u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RequestRemoveInsuranceFundStakeArguments {
                    market_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RequestRemoveInsuranceFundStakeAccounts {
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    authority,
                    insuranceFundVault,
                    remaining,
                };
                return Ok(Instruction::RequestRemoveInsuranceFundStake { accounts, args });
            }
            [97u8, 235u8, 78u8, 62u8, 212u8, 42u8, 241u8, 127u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CancelRequestRemoveInsuranceFundStakeArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelRequestRemoveInsuranceFundStakeAccounts {
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    authority,
                    insuranceFundVault,
                    remaining,
                };
                return Ok(Instruction::CancelRequestRemoveInsuranceFundStake { accounts, args });
            }
            [128u8, 166u8, 142u8, 9u8, 254u8, 187u8, 143u8, 174u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RemoveInsuranceFundStakeArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 9usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveInsuranceFundStakeAccounts {
                    state,
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    authority,
                    insuranceFundVault,
                    driftSigner,
                    userTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::RemoveInsuranceFundStake { accounts, args });
            }
            [94u8, 93u8, 226u8, 240u8, 195u8, 201u8, 184u8, 109u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let shares: u128 = <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = TransferProtocolIfSharesArguments {
                    market_index,
                    shares,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let signer = keys.next().unwrap().clone();
                let transferConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let insuranceFundStake = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferProtocolIfSharesAccounts {
                    signer,
                    transferConfig,
                    state,
                    spotMarket,
                    insuranceFundStake,
                    userStats,
                    authority,
                    insuranceFundVault,
                    remaining,
                };
                return Ok(Instruction::TransferProtocolIfShares { accounts, args });
            }
            [176u8, 69u8, 143u8, 205u8, 32u8, 132u8, 163u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let in_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let out_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount_in: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = BeginInsuranceFundSwapArguments {
                    in_market_index,
                    out_market_index,
                    amount_in,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let outInsuranceFundVault = keys.next().unwrap().clone();
                let inInsuranceFundVault = keys.next().unwrap().clone();
                let outTokenAccount = keys.next().unwrap().clone();
                let inTokenAccount = keys.next().unwrap().clone();
                let ifRebalanceConfig = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BeginInsuranceFundSwapAccounts {
                    state,
                    authority,
                    outInsuranceFundVault,
                    inInsuranceFundVault,
                    outTokenAccount,
                    inTokenAccount,
                    ifRebalanceConfig,
                    tokenProgram,
                    driftSigner,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::BeginInsuranceFundSwap { accounts, args });
            }
            [206u8, 230u8, 98u8, 8u8, 249u8, 158u8, 169u8, 167u8] => {
                let mut rdr: &[u8] = rest;
                let in_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let out_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = EndInsuranceFundSwapArguments {
                    in_market_index,
                    out_market_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let outInsuranceFundVault = keys.next().unwrap().clone();
                let inInsuranceFundVault = keys.next().unwrap().clone();
                let outTokenAccount = keys.next().unwrap().clone();
                let inTokenAccount = keys.next().unwrap().clone();
                let ifRebalanceConfig = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EndInsuranceFundSwapAccounts {
                    state,
                    authority,
                    outInsuranceFundVault,
                    inInsuranceFundVault,
                    outTokenAccount,
                    inTokenAccount,
                    ifRebalanceConfig,
                    tokenProgram,
                    driftSigner,
                    instructions,
                    remaining,
                };
                return Ok(Instruction::EndInsuranceFundSwap { accounts, args });
            }
            [236u8, 136u8, 147u8, 153u8, 146u8, 205u8, 104u8, 29u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = TransferProtocolIfSharesToRevenuePoolArguments {
                    market_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let state = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let ifRebalanceConfig = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferProtocolIfSharesToRevenuePoolAccounts {
                    state,
                    authority,
                    insuranceFundVault,
                    spotMarketVault,
                    ifRebalanceConfig,
                    tokenProgram,
                    driftSigner,
                    remaining,
                };
                return Ok(Instruction::TransferProtocolIfSharesToRevenuePool { accounts, args });
            }
            [230u8, 191u8, 189u8, 94u8, 108u8, 59u8, 74u8, 197u8] => {
                let mut rdr: &[u8] = rest;
                let feed_id: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let params: Vec<u8> =
                    <Vec<u8> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePythPullOracleArguments { feed_id, params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let keeper = keys.next().unwrap().clone();
                let pythSolanaReceiver = keys.next().unwrap().clone();
                let encodedVaa = keys.next().unwrap().clone();
                let priceFeed = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePythPullOracleAccounts {
                    keeper,
                    pythSolanaReceiver,
                    encodedVaa,
                    priceFeed,
                    remaining,
                };
                return Ok(Instruction::UpdatePythPullOracle { accounts, args });
            }
            [116u8, 122u8, 137u8, 158u8, 224u8, 195u8, 173u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let feed_id: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let params: Vec<u8> =
                    <Vec<u8> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PostPythPullOracleUpdateAtomicArguments { feed_id, params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let keeper = keys.next().unwrap().clone();
                let pythSolanaReceiver = keys.next().unwrap().clone();
                let guardianSet = keys.next().unwrap().clone();
                let priceFeed = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PostPythPullOracleUpdateAtomicAccounts {
                    keeper,
                    pythSolanaReceiver,
                    guardianSet,
                    priceFeed,
                    remaining,
                };
                return Ok(Instruction::PostPythPullOracleUpdateAtomic { accounts, args });
            }
            [243u8, 79u8, 204u8, 228u8, 227u8, 208u8, 100u8, 244u8] => {
                let mut rdr: &[u8] = rest;
                let params: Vec<u8> =
                    <Vec<u8> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PostMultiPythPullOracleUpdatesAtomicArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let keeper = keys.next().unwrap().clone();
                let pythSolanaReceiver = keys.next().unwrap().clone();
                let guardianSet = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PostMultiPythPullOracleUpdatesAtomicAccounts {
                    keeper,
                    pythSolanaReceiver,
                    guardianSet,
                    remaining,
                };
                return Ok(Instruction::PostMultiPythPullOracleUpdatesAtomic { accounts, args });
            }
            [183u8, 119u8, 59u8, 170u8, 137u8, 35u8, 242u8, 86u8] => {
                let mut rdr: &[u8] = rest;
                let args = PauseSpotMarketDepositWithdrawArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let keeper = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PauseSpotMarketDepositWithdrawAccounts {
                    state,
                    keeper,
                    spotMarket,
                    spotMarketVault,
                    remaining,
                };
                return Ok(Instruction::PauseSpotMarketDepositWithdraw { accounts, args });
            }
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let quoteAssetMint = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    admin,
                    state,
                    quoteAssetMint,
                    driftSigner,
                    rent,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [234u8, 196u8, 128u8, 44u8, 94u8, 15u8, 48u8, 201u8] => {
                let mut rdr: &[u8] = rest;
                let optimal_utilization: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let optimal_borrow_rate: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_borrow_rate: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_source: OracleSource =
                    <OracleSource as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let initial_asset_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let maintenance_asset_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let initial_liability_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let maintenance_liability_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let imf_factor: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liquidator_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let if_liquidation_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let active_status: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let asset_tier: AssetTier =
                    <AssetTier as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let scale_initial_asset_weight_start: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let withdraw_guard_threshold: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let order_tick_size: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let order_step_size: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let if_total_factor: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeSpotMarketArguments {
                    optimal_utilization,
                    optimal_borrow_rate,
                    max_borrow_rate,
                    oracle_source,
                    initial_asset_weight,
                    maintenance_asset_weight,
                    initial_liability_weight,
                    maintenance_liability_weight,
                    imf_factor,
                    liquidator_fee,
                    if_liquidation_fee,
                    active_status,
                    asset_tier,
                    scale_initial_asset_weight_start,
                    withdraw_guard_threshold,
                    order_tick_size,
                    order_step_size,
                    if_total_factor,
                    name,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 11usize;
                let spotMarket = keys.next().unwrap().clone();
                let spotMarketMint = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeSpotMarketAccounts {
                    spotMarket,
                    spotMarketMint,
                    spotMarketVault,
                    insuranceFundVault,
                    driftSigner,
                    state,
                    oracle,
                    admin,
                    rent,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeSpotMarket { accounts, args });
            }
            [31u8, 140u8, 67u8, 191u8, 189u8, 20u8, 101u8, 221u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DeleteInitializedSpotMarketArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 7usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let insuranceFundVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeleteInitializedSpotMarketAccounts {
                    admin,
                    state,
                    spotMarket,
                    spotMarketVault,
                    insuranceFundVault,
                    driftSigner,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DeleteInitializedSpotMarket { accounts, args });
            }
            [193u8, 211u8, 132u8, 172u8, 70u8, 171u8, 7u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeSerumFulfillmentConfigArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 11usize;
                let baseSpotMarket = keys.next().unwrap().clone();
                let quoteSpotMarket = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let serumProgram = keys.next().unwrap().clone();
                let serumMarket = keys.next().unwrap().clone();
                let serumOpenOrders = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let serumFulfillmentConfig = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeSerumFulfillmentConfigAccounts {
                    baseSpotMarket,
                    quoteSpotMarket,
                    state,
                    serumProgram,
                    serumMarket,
                    serumOpenOrders,
                    driftSigner,
                    serumFulfillmentConfig,
                    admin,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeSerumFulfillmentConfig { accounts, args });
            }
            [171u8, 109u8, 240u8, 251u8, 95u8, 1u8, 149u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let status: SpotFulfillmentConfigStatus =
                    <SpotFulfillmentConfigStatus as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )?;
                let args = UpdateSerumFulfillmentConfigStatusArguments { status };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let serumFulfillmentConfig = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSerumFulfillmentConfigStatusAccounts {
                    state,
                    serumFulfillmentConfig,
                    admin,
                    remaining,
                };
                return Ok(Instruction::UpdateSerumFulfillmentConfigStatus { accounts, args });
            }
            [7u8, 221u8, 103u8, 153u8, 107u8, 57u8, 27u8, 197u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeOpenbookV2FulfillmentConfigArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let baseSpotMarket = keys.next().unwrap().clone();
                let quoteSpotMarket = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let openbookV2Program = keys.next().unwrap().clone();
                let openbookV2Market = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let openbookV2FulfillmentConfig = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeOpenbookV2FulfillmentConfigAccounts {
                    baseSpotMarket,
                    quoteSpotMarket,
                    state,
                    openbookV2Program,
                    openbookV2Market,
                    driftSigner,
                    openbookV2FulfillmentConfig,
                    admin,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeOpenbookV2FulfillmentConfig { accounts, args });
            }
            [25u8, 173u8, 19u8, 189u8, 4u8, 211u8, 64u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let status: SpotFulfillmentConfigStatus =
                    <SpotFulfillmentConfigStatus as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )?;
                let args = OpenbookV2FulfillmentConfigStatusArguments { status };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let openbookV2FulfillmentConfig = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OpenbookV2FulfillmentConfigStatusAccounts {
                    state,
                    openbookV2FulfillmentConfig,
                    admin,
                    remaining,
                };
                return Ok(Instruction::OpenbookV2FulfillmentConfigStatus { accounts, args });
            }
            [135u8, 132u8, 110u8, 107u8, 185u8, 160u8, 169u8, 154u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializePhoenixFulfillmentConfigArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 10usize;
                let baseSpotMarket = keys.next().unwrap().clone();
                let quoteSpotMarket = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let phoenixProgram = keys.next().unwrap().clone();
                let phoenixMarket = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let phoenixFulfillmentConfig = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePhoenixFulfillmentConfigAccounts {
                    baseSpotMarket,
                    quoteSpotMarket,
                    state,
                    phoenixProgram,
                    phoenixMarket,
                    driftSigner,
                    phoenixFulfillmentConfig,
                    admin,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePhoenixFulfillmentConfig { accounts, args });
            }
            [96u8, 31u8, 113u8, 32u8, 12u8, 203u8, 7u8, 154u8] => {
                let mut rdr: &[u8] = rest;
                let status: SpotFulfillmentConfigStatus =
                    <SpotFulfillmentConfigStatus as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )?;
                let args = PhoenixFulfillmentConfigStatusArguments { status };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let phoenixFulfillmentConfig = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PhoenixFulfillmentConfigStatusAccounts {
                    state,
                    phoenixFulfillmentConfig,
                    admin,
                    remaining,
                };
                return Ok(Instruction::PhoenixFulfillmentConfigStatus { accounts, args });
            }
            [219u8, 8u8, 246u8, 96u8, 169u8, 121u8, 91u8, 110u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateSerumVaultArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let state = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let srmVault = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSerumVaultAccounts {
                    state,
                    admin,
                    srmVault,
                    remaining,
                };
                return Ok(Instruction::UpdateSerumVault { accounts, args });
            }
            [132u8, 9u8, 229u8, 118u8, 117u8, 118u8, 117u8, 62u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amm_base_asset_reserve: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amm_quote_asset_reserve: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amm_periodicity: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amm_peg_multiplier: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_source: OracleSource =
                    <OracleSource as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let contract_tier: ContractTier =
                    <ContractTier as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let margin_ratio_initial: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let margin_ratio_maintenance: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let liquidator_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let if_liquidation_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let imf_factor: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let active_status: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let base_spread: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_spread: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_open_interest: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_revenue_withdraw_per_period: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let quote_max_insurance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let order_step_size: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let order_tick_size: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_order_size: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let concentration_coef_scale: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let curve_update_intensity: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amm_jit_intensity: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializePerpMarketArguments {
                    market_index,
                    amm_base_asset_reserve,
                    amm_quote_asset_reserve,
                    amm_periodicity,
                    amm_peg_multiplier,
                    oracle_source,
                    contract_tier,
                    margin_ratio_initial,
                    margin_ratio_maintenance,
                    liquidator_fee,
                    if_liquidation_fee,
                    imf_factor,
                    active_status,
                    base_spread,
                    max_spread,
                    max_open_interest,
                    max_revenue_withdraw_per_period,
                    quote_max_insurance,
                    order_step_size,
                    order_tick_size,
                    min_order_size,
                    concentration_coef_scale,
                    curve_update_intensity,
                    amm_jit_intensity,
                    name,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePerpMarketAccounts {
                    admin,
                    state,
                    perpMarket,
                    oracle,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePerpMarket { accounts, args });
            }
            [248u8, 70u8, 198u8, 224u8, 224u8, 105u8, 125u8, 195u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePredictionMarketArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePredictionMarketAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::InitializePredictionMarket { accounts, args });
            }
            [91u8, 154u8, 24u8, 87u8, 106u8, 59u8, 190u8, 66u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DeleteInitializedPerpMarketArguments { market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeleteInitializedPerpMarketAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::DeleteInitializedPerpMarket { accounts, args });
            }
            [235u8, 109u8, 2u8, 82u8, 219u8, 118u8, 6u8, 159u8] => {
                let mut rdr: &[u8] = rest;
                let base_asset_reserve: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let quote_asset_reserve: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let sqrt_k: u128 = <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = MoveAmmPriceArguments {
                    base_asset_reserve,
                    quote_asset_reserve,
                    sqrt_k,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MoveAmmPriceAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::MoveAmmPrice { accounts, args });
            }
            [24u8, 87u8, 10u8, 115u8, 165u8, 190u8, 80u8, 139u8] => {
                let mut rdr: &[u8] = rest;
                let peg_multiplier: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let sqrt_k: u128 = <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RecenterPerpMarketAmmArguments {
                    peg_multiplier,
                    sqrt_k,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RecenterPerpMarketAmmAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::RecenterPerpMarketAmm { accounts, args });
            }
            [166u8, 19u8, 64u8, 10u8, 14u8, 51u8, 101u8, 122u8] => {
                let mut rdr: &[u8] = rest;
                let depth: Option<u128> = parse_option(&mut rdr)?;
                let args = RecenterPerpMarketAmmCrankArguments { depth };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RecenterPerpMarketAmmCrankAccounts {
                    admin,
                    state,
                    perpMarket,
                    spotMarket,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::RecenterPerpMarketAmmCrank { accounts, args });
            }
            [122u8, 101u8, 249u8, 238u8, 209u8, 9u8, 241u8, 245u8] => {
                let mut rdr: &[u8] = rest;
                let quote_asset_amount_with_unsettled_lp: Option<i64> = parse_option(&mut rdr)?;
                let net_unsettled_funding_pnl: Option<i64> = parse_option(&mut rdr)?;
                let update_amm_summary_stats: Option<bool> = parse_option(&mut rdr)?;
                let exclude_total_liq_fee: Option<bool> = parse_option(&mut rdr)?;
                let params = UpdatePerpMarketSummaryStatsParams {
                    quote_asset_amount_with_unsettled_lp,
                    net_unsettled_funding_pnl,
                    update_amm_summary_stats,
                    exclude_total_liq_fee,
                };
                let args = UpdatePerpMarketAmmSummaryStatsArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketAmmSummaryStatsAccounts {
                    admin,
                    state,
                    perpMarket,
                    spotMarket,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketAmmSummaryStats { accounts, args });
            }
            [44u8, 221u8, 227u8, 151u8, 131u8, 140u8, 22u8, 110u8] => {
                let mut rdr: &[u8] = rest;
                let expiry_ts: i64 = <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketExpiryArguments { expiry_ts };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketExpiryAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketExpiry { accounts, args });
            }
            [55u8, 19u8, 238u8, 169u8, 227u8, 90u8, 200u8, 184u8] => {
                let mut rdr: &[u8] = rest;
                let args = SettleExpiredMarketPoolsToRevenuePoolArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SettleExpiredMarketPoolsToRevenuePoolAccounts {
                    state,
                    admin,
                    spotMarket,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::SettleExpiredMarketPoolsToRevenuePool { accounts, args });
            }
            [34u8, 58u8, 57u8, 68u8, 97u8, 80u8, 244u8, 6u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositIntoPerpMarketFeePoolArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let sourceVault = keys.next().unwrap().clone();
                let driftSigner = keys.next().unwrap().clone();
                let quoteSpotMarket = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositIntoPerpMarketFeePoolAccounts {
                    state,
                    perpMarket,
                    admin,
                    sourceVault,
                    driftSigner,
                    quoteSpotMarket,
                    spotMarketVault,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DepositIntoPerpMarketFeePool { accounts, args });
            }
            [48u8, 252u8, 119u8, 73u8, 255u8, 205u8, 174u8, 247u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositIntoSpotMarketVaultArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let sourceVault = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositIntoSpotMarketVaultAccounts {
                    state,
                    spotMarket,
                    admin,
                    sourceVault,
                    spotMarketVault,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DepositIntoSpotMarketVault { accounts, args });
            }
            [92u8, 40u8, 151u8, 42u8, 122u8, 254u8, 139u8, 246u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositIntoSpotMarketRevenuePoolArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositIntoSpotMarketRevenuePoolAccounts {
                    state,
                    spotMarket,
                    authority,
                    spotMarketVault,
                    userTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DepositIntoSpotMarketRevenuePool { accounts, args });
            }
            [3u8, 36u8, 102u8, 89u8, 180u8, 128u8, 120u8, 213u8] => {
                let mut rdr: &[u8] = rest;
                let new_peg_candidate: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RepegAmmCurveArguments { new_peg_candidate };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RepegAmmCurveAccounts {
                    state,
                    perpMarket,
                    oracle,
                    admin,
                    remaining,
                };
                return Ok(Instruction::RepegAmmCurve { accounts, args });
            }
            [241u8, 74u8, 114u8, 123u8, 206u8, 153u8, 24u8, 202u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePerpMarketAmmOracleTwapArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketAmmOracleTwapAccounts {
                    state,
                    perpMarket,
                    oracle,
                    admin,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketAmmOracleTwap { accounts, args });
            }
            [127u8, 10u8, 55u8, 164u8, 123u8, 226u8, 47u8, 24u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResetPerpMarketAmmOracleTwapArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResetPerpMarketAmmOracleTwapAccounts {
                    state,
                    perpMarket,
                    oracle,
                    admin,
                    remaining,
                };
                return Ok(Instruction::ResetPerpMarketAmmOracleTwap { accounts, args });
            }
            [72u8, 98u8, 9u8, 139u8, 129u8, 229u8, 172u8, 56u8] => {
                let mut rdr: &[u8] = rest;
                let sqrt_k: u128 = <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateKArguments { sqrt_k };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateKAccounts {
                    admin,
                    state,
                    perpMarket,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::UpdateK { accounts, args });
            }
            [130u8, 173u8, 107u8, 45u8, 119u8, 105u8, 26u8, 113u8] => {
                let mut rdr: &[u8] = rest;
                let margin_ratio_initial: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let margin_ratio_maintenance: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMarginRatioArguments {
                    margin_ratio_initial,
                    margin_ratio_maintenance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMarginRatioAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMarginRatio { accounts, args });
            }
            [88u8, 112u8, 86u8, 49u8, 24u8, 116u8, 74u8, 157u8] => {
                let mut rdr: &[u8] = rest;
                let margin_ratio_initial: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let margin_ratio_maintenance: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketHighLeverageMarginRatioArguments {
                    margin_ratio_initial,
                    margin_ratio_maintenance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketHighLeverageMarginRatioAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketHighLeverageMarginRatio { accounts, args });
            }
            [171u8, 161u8, 69u8, 91u8, 129u8, 139u8, 161u8, 28u8] => {
                let mut rdr: &[u8] = rest;
                let funding_period: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketFundingPeriodArguments { funding_period };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketFundingPeriodAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketFundingPeriod { accounts, args });
            }
            [15u8, 206u8, 73u8, 133u8, 60u8, 8u8, 86u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let unrealized_max_imbalance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_revenue_withdraw_per_period: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let quote_max_insurance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMaxImbalancesArguments {
                    unrealized_max_imbalance,
                    max_revenue_withdraw_per_period,
                    quote_max_insurance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMaxImbalancesAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMaxImbalances { accounts, args });
            }
            [90u8, 137u8, 9u8, 145u8, 41u8, 8u8, 148u8, 117u8] => {
                let mut rdr: &[u8] = rest;
                let liquidator_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let if_liquidation_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketLiquidationFeeArguments {
                    liquidator_fee,
                    if_liquidation_fee,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketLiquidationFeeAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketLiquidationFee { accounts, args });
            }
            [44u8, 69u8, 43u8, 226u8, 204u8, 223u8, 202u8, 52u8] => {
                let mut rdr: &[u8] = rest;
                let insurance_fund_unstaking_period: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateInsuranceFundUnstakingPeriodArguments {
                    insurance_fund_unstaking_period,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateInsuranceFundUnstakingPeriodAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateInsuranceFundUnstakingPeriod { accounts, args });
            }
            [22u8, 213u8, 197u8, 160u8, 139u8, 193u8, 81u8, 149u8] => {
                let mut rdr: &[u8] = rest;
                let pool_id: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketPoolIdArguments { pool_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketPoolIdAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketPoolId { accounts, args });
            }
            [11u8, 13u8, 255u8, 53u8, 56u8, 136u8, 104u8, 177u8] => {
                let mut rdr: &[u8] = rest;
                let liquidator_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let if_liquidation_fee: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketLiquidationFeeArguments {
                    liquidator_fee,
                    if_liquidation_fee,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketLiquidationFeeAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketLiquidationFee { accounts, args });
            }
            [56u8, 18u8, 39u8, 61u8, 155u8, 211u8, 44u8, 133u8] => {
                let mut rdr: &[u8] = rest;
                let withdraw_guard_threshold: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateWithdrawGuardThresholdArguments {
                    withdraw_guard_threshold,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateWithdrawGuardThresholdAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateWithdrawGuardThreshold { accounts, args });
            }
            [147u8, 30u8, 224u8, 34u8, 18u8, 230u8, 105u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let spot_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let user_if_factor: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let total_if_factor: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketIfFactorArguments {
                    spot_market_index,
                    user_if_factor,
                    total_if_factor,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketIfFactorAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketIfFactor { accounts, args });
            }
            [81u8, 92u8, 126u8, 41u8, 250u8, 225u8, 156u8, 219u8] => {
                let mut rdr: &[u8] = rest;
                let revenue_settle_period: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketRevenueSettlePeriodArguments {
                    revenue_settle_period,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketRevenueSettlePeriodAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketRevenueSettlePeriod { accounts, args });
            }
            [78u8, 94u8, 16u8, 188u8, 193u8, 110u8, 231u8, 31u8] => {
                let mut rdr: &[u8] = rest;
                let status: MarketStatus =
                    <MarketStatus as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketStatusArguments { status };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketStatusAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketStatus { accounts, args });
            }
            [100u8, 61u8, 153u8, 81u8, 180u8, 12u8, 6u8, 248u8] => {
                let mut rdr: &[u8] = rest;
                let paused_operations: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketPausedOperationsArguments { paused_operations };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketPausedOperationsAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketPausedOperations { accounts, args });
            }
            [253u8, 209u8, 231u8, 14u8, 242u8, 208u8, 243u8, 130u8] => {
                let mut rdr: &[u8] = rest;
                let asset_tier: AssetTier =
                    <AssetTier as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketAssetTierArguments { asset_tier };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketAssetTierAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketAssetTier { accounts, args });
            }
            [109u8, 33u8, 87u8, 195u8, 255u8, 36u8, 6u8, 81u8] => {
                let mut rdr: &[u8] = rest;
                let initial_asset_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let maintenance_asset_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let initial_liability_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let maintenance_liability_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let imf_factor: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketMarginWeightsArguments {
                    initial_asset_weight,
                    maintenance_asset_weight,
                    initial_liability_weight,
                    maintenance_liability_weight,
                    imf_factor,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketMarginWeightsAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketMarginWeights { accounts, args });
            }
            [71u8, 239u8, 236u8, 153u8, 210u8, 62u8, 254u8, 76u8] => {
                let mut rdr: &[u8] = rest;
                let optimal_utilization: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let optimal_borrow_rate: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_borrow_rate: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_borrow_rate: Option<u8> = parse_option(&mut rdr)?;
                let args = UpdateSpotMarketBorrowRateArguments {
                    optimal_utilization,
                    optimal_borrow_rate,
                    max_borrow_rate,
                    min_borrow_rate,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketBorrowRateAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketBorrowRate { accounts, args });
            }
            [56u8, 191u8, 79u8, 18u8, 26u8, 121u8, 80u8, 208u8] => {
                let mut rdr: &[u8] = rest;
                let max_token_deposits: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketMaxTokenDepositsArguments { max_token_deposits };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketMaxTokenDepositsAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketMaxTokenDeposits { accounts, args });
            }
            [57u8, 102u8, 204u8, 212u8, 253u8, 95u8, 13u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let max_token_borrows_fraction: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketMaxTokenBorrowsArguments {
                    max_token_borrows_fraction,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketMaxTokenBorrowsAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketMaxTokenBorrows { accounts, args });
            }
            [217u8, 204u8, 204u8, 118u8, 204u8, 130u8, 225u8, 147u8] => {
                let mut rdr: &[u8] = rest;
                let scale_initial_asset_weight_start: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketScaleInitialAssetWeightStartArguments {
                    scale_initial_asset_weight_start,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketScaleInitialAssetWeightStartAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketScaleInitialAssetWeightStart {
                    accounts,
                    args,
                });
            }
            [114u8, 184u8, 102u8, 37u8, 246u8, 186u8, 180u8, 99u8] => {
                let mut rdr: &[u8] = rest;
                let oracle: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_source: OracleSource =
                    <OracleSource as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let skip_invariant_check = if !rdr.is_empty() {
                    let tag = rdr[0];
                    rdr = &rdr[1..];
                    Some(tag == 1)
                } else {
                    None
                };
                let args = UpdateSpotMarketOracleArguments {
                    oracle,
                    oracle_source,
                    skip_invariant_check,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let oldOracle = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketOracleAccounts {
                    admin,
                    state,
                    spotMarket,
                    oracle,
                    oldOracle,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketOracle { accounts, args });
            }
            [238u8, 153u8, 137u8, 80u8, 206u8, 59u8, 250u8, 61u8] => {
                let mut rdr: &[u8] = rest;
                let step_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let tick_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketStepSizeAndTickSizeArguments {
                    step_size,
                    tick_size,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketStepSizeAndTickSizeAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketStepSizeAndTickSize { accounts, args });
            }
            [93u8, 128u8, 11u8, 119u8, 26u8, 20u8, 181u8, 50u8] => {
                let mut rdr: &[u8] = rest;
                let order_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketMinOrderSizeArguments { order_size };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketMinOrderSizeAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketMinOrderSize { accounts, args });
            }
            [190u8, 79u8, 206u8, 15u8, 26u8, 229u8, 229u8, 43u8] => {
                let mut rdr: &[u8] = rest;
                let orders_enabled: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketOrdersEnabledArguments { orders_enabled };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketOrdersEnabledAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketOrdersEnabled { accounts, args });
            }
            [101u8, 215u8, 79u8, 74u8, 59u8, 41u8, 79u8, 12u8] => {
                let mut rdr: &[u8] = rest;
                let paused_operations: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketIfPausedOperationsArguments { paused_operations };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketIfPausedOperationsAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketIfPausedOperations { accounts, args });
            }
            [17u8, 208u8, 1u8, 1u8, 162u8, 211u8, 188u8, 224u8] => {
                let mut rdr: &[u8] = rest;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketNameArguments { name };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketNameAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketName { accounts, args });
            }
            [71u8, 201u8, 175u8, 122u8, 255u8, 207u8, 196u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let status: MarketStatus =
                    <MarketStatus as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketStatusArguments { status };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketStatusAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketStatus { accounts, args });
            }
            [53u8, 16u8, 136u8, 132u8, 30u8, 220u8, 121u8, 85u8] => {
                let mut rdr: &[u8] = rest;
                let paused_operations: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketPausedOperationsArguments { paused_operations };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketPausedOperationsAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketPausedOperations { accounts, args });
            }
            [236u8, 128u8, 15u8, 95u8, 203u8, 214u8, 68u8, 117u8] => {
                let mut rdr: &[u8] = rest;
                let contract_tier: ContractTier =
                    <ContractTier as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketContractTierArguments { contract_tier };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketContractTierAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketContractTier { accounts, args });
            }
            [207u8, 194u8, 56u8, 132u8, 35u8, 67u8, 71u8, 244u8] => {
                let mut rdr: &[u8] = rest;
                let imf_factor: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let unrealized_pnl_imf_factor: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketImfFactorArguments {
                    imf_factor,
                    unrealized_pnl_imf_factor,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketImfFactorAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketImfFactor { accounts, args });
            }
            [135u8, 132u8, 205u8, 165u8, 109u8, 150u8, 166u8, 106u8] => {
                let mut rdr: &[u8] = rest;
                let unrealized_initial_asset_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let unrealized_maintenance_asset_weight: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketUnrealizedAssetWeightArguments {
                    unrealized_initial_asset_weight,
                    unrealized_maintenance_asset_weight,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketUnrealizedAssetWeightAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketUnrealizedAssetWeight { accounts, args });
            }
            [24u8, 78u8, 232u8, 126u8, 169u8, 176u8, 230u8, 16u8] => {
                let mut rdr: &[u8] = rest;
                let concentration_scale: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketConcentrationCoefArguments {
                    concentration_scale,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketConcentrationCoefAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketConcentrationCoef { accounts, args });
            }
            [50u8, 131u8, 6u8, 156u8, 226u8, 231u8, 189u8, 72u8] => {
                let mut rdr: &[u8] = rest;
                let curve_update_intensity: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketCurveUpdateIntensityArguments {
                    curve_update_intensity,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketCurveUpdateIntensityAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketCurveUpdateIntensity { accounts, args });
            }
            [62u8, 87u8, 68u8, 115u8, 29u8, 150u8, 150u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let target_base_asset_amount_per_lp: i32 =
                    <i32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketTargetBaseAssetAmountPerLpArguments {
                    target_base_asset_amount_per_lp,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketTargetBaseAssetAmountPerLp {
                    accounts,
                    args,
                });
            }
            [103u8, 152u8, 103u8, 102u8, 89u8, 144u8, 193u8, 71u8] => {
                let mut rdr: &[u8] = rest;
                let per_lp_base: i8 = <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketPerLpBaseArguments { per_lp_base };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketPerLpBaseAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketPerLpBase { accounts, args });
            }
            [198u8, 133u8, 88u8, 41u8, 241u8, 119u8, 61u8, 14u8] => {
                let mut rdr: &[u8] = rest;
                let lp_cooldown_time: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateLpCooldownTimeArguments { lp_cooldown_time };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateLpCooldownTimeAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateLpCooldownTime { accounts, args });
            }
            [23u8, 178u8, 111u8, 203u8, 73u8, 22u8, 140u8, 75u8] => {
                let mut rdr: &[u8] = rest;
                let fee_tiers: [FeeTier; 10usize] =
                    <[FeeTier; 10usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let filler_reward_structure: OrderFillerRewardStructure =
                    <OrderFillerRewardStructure as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )?;
                let referrer_reward_epoch_upper_bound: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let flat_filler_fee: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let fee_structure = FeeStructure {
                    fee_tiers,
                    filler_reward_structure,
                    referrer_reward_epoch_upper_bound,
                    flat_filler_fee,
                };
                let args = UpdatePerpFeeStructureArguments { fee_structure };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpFeeStructureAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpFeeStructure { accounts, args });
            }
            [97u8, 216u8, 105u8, 131u8, 113u8, 246u8, 142u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let fee_tiers: [FeeTier; 10usize] =
                    <[FeeTier; 10usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let filler_reward_structure: OrderFillerRewardStructure =
                    <OrderFillerRewardStructure as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )?;
                let referrer_reward_epoch_upper_bound: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let flat_filler_fee: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let fee_structure = FeeStructure {
                    fee_tiers,
                    filler_reward_structure,
                    referrer_reward_epoch_upper_bound,
                    flat_filler_fee,
                };
                let args = UpdateSpotFeeStructureArguments { fee_structure };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotFeeStructureAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotFeeStructure { accounts, args });
            }
            [210u8, 133u8, 225u8, 128u8, 194u8, 50u8, 13u8, 109u8] => {
                let mut rdr: &[u8] = rest;
                let initial_pct_to_liquidate: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateInitialPctToLiquidateArguments {
                    initial_pct_to_liquidate,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateInitialPctToLiquidateAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateInitialPctToLiquidate { accounts, args });
            }
            [28u8, 154u8, 20u8, 249u8, 102u8, 192u8, 73u8, 71u8] => {
                let mut rdr: &[u8] = rest;
                let liquidation_duration: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateLiquidationDurationArguments {
                    liquidation_duration,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateLiquidationDurationAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateLiquidationDuration { accounts, args });
            }
            [132u8, 224u8, 243u8, 160u8, 154u8, 82u8, 97u8, 215u8] => {
                let mut rdr: &[u8] = rest;
                let liquidation_margin_buffer_ratio: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateLiquidationMarginBufferRatioArguments {
                    liquidation_margin_buffer_ratio,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateLiquidationMarginBufferRatioAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateLiquidationMarginBufferRatio { accounts, args });
            }
            [131u8, 112u8, 10u8, 59u8, 32u8, 54u8, 40u8, 164u8] => {
                let mut rdr: &[u8] = rest;
                let price_divergence: PriceDivergenceGuardRails =
                    <PriceDivergenceGuardRails as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )?;
                let validity: ValidityGuardRails =
                    <ValidityGuardRails as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_guard_rails = OracleGuardRails {
                    price_divergence,
                    validity,
                };
                let args = UpdateOracleGuardRailsArguments { oracle_guard_rails };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateOracleGuardRailsAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateOracleGuardRails { accounts, args });
            }
            [97u8, 68u8, 199u8, 235u8, 131u8, 80u8, 61u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let settlement_duration: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateStateSettlementDurationArguments {
                    settlement_duration,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStateSettlementDurationAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateStateSettlementDuration { accounts, args });
            }
            [155u8, 123u8, 214u8, 2u8, 221u8, 166u8, 204u8, 85u8] => {
                let mut rdr: &[u8] = rest;
                let max_number_of_sub_accounts: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateStateMaxNumberOfSubAccountsArguments {
                    max_number_of_sub_accounts,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStateMaxNumberOfSubAccountsAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateStateMaxNumberOfSubAccounts { accounts, args });
            }
            [237u8, 225u8, 25u8, 237u8, 193u8, 45u8, 77u8, 97u8] => {
                let mut rdr: &[u8] = rest;
                let max_initialize_user_fee: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateStateMaxInitializeUserFeeArguments {
                    max_initialize_user_fee,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStateMaxInitializeUserFeeAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateStateMaxInitializeUserFee { accounts, args });
            }
            [182u8, 113u8, 111u8, 160u8, 67u8, 174u8, 89u8, 191u8] => {
                let mut rdr: &[u8] = rest;
                let oracle: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let oracle_source: OracleSource =
                    <OracleSource as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let skip_invariant_check = if !rdr.is_empty() {
                    let tag = rdr[0];
                    rdr = &rdr[1..];
                    Some(tag == 1)
                } else {
                    None
                };
                let args = UpdatePerpMarketOracleArguments {
                    oracle,
                    oracle_source,
                    skip_invariant_check,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let oldOracle = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketOracleAccounts {
                    admin,
                    state,
                    perpMarket,
                    oracle,
                    oldOracle,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketOracle { accounts, args });
            }
            [71u8, 95u8, 84u8, 168u8, 9u8, 157u8, 198u8, 65u8] => {
                let mut rdr: &[u8] = rest;
                let base_spread: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketBaseSpreadArguments { base_spread };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketBaseSpreadAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketBaseSpread { accounts, args });
            }
            [181u8, 191u8, 53u8, 109u8, 166u8, 249u8, 55u8, 142u8] => {
                let mut rdr: &[u8] = rest;
                let amm_jit_intensity: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateAmmJitIntensityArguments { amm_jit_intensity };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAmmJitIntensityAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateAmmJitIntensity { accounts, args });
            }
            [80u8, 252u8, 122u8, 62u8, 40u8, 218u8, 91u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let max_spread: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMaxSpreadArguments { max_spread };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMaxSpreadAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMaxSpread { accounts, args });
            }
            [231u8, 255u8, 97u8, 25u8, 146u8, 139u8, 174u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let step_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let tick_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketStepSizeAndTickSizeArguments {
                    step_size,
                    tick_size,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketStepSizeAndTickSizeAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketStepSizeAndTickSize { accounts, args });
            }
            [211u8, 31u8, 21u8, 210u8, 64u8, 108u8, 66u8, 201u8] => {
                let mut rdr: &[u8] = rest;
                let name: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketNameArguments { name };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketNameAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketName { accounts, args });
            }
            [226u8, 74u8, 5u8, 89u8, 108u8, 223u8, 46u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let order_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMinOrderSizeArguments { order_size };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMinOrderSizeAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMinOrderSize { accounts, args });
            }
            [235u8, 37u8, 40u8, 196u8, 70u8, 146u8, 54u8, 201u8] => {
                let mut rdr: &[u8] = rest;
                let max_slippage_ratio: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMaxSlippageRatioArguments { max_slippage_ratio };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMaxSlippageRatioAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMaxSlippageRatio { accounts, args });
            }
            [19u8, 172u8, 114u8, 154u8, 42u8, 135u8, 161u8, 133u8] => {
                let mut rdr: &[u8] = rest;
                let max_fill_reserve_fraction: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMaxFillReserveFractionArguments {
                    max_fill_reserve_fraction,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMaxFillReserveFractionAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMaxFillReserveFraction { accounts, args });
            }
            [194u8, 79u8, 149u8, 224u8, 246u8, 102u8, 186u8, 140u8] => {
                let mut rdr: &[u8] = rest;
                let max_open_interest: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketMaxOpenInterestArguments { max_open_interest };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketMaxOpenInterestAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketMaxOpenInterest { accounts, args });
            }
            [35u8, 62u8, 144u8, 177u8, 180u8, 62u8, 215u8, 196u8] => {
                let mut rdr: &[u8] = rest;
                let number_of_users: Option<u32> = parse_option(&mut rdr)?;
                let number_of_users_with_base: Option<u32> = parse_option(&mut rdr)?;
                let args = UpdatePerpMarketNumberOfUsersArguments {
                    number_of_users,
                    number_of_users_with_base,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketNumberOfUsersAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketNumberOfUsers { accounts, args });
            }
            [194u8, 174u8, 87u8, 102u8, 43u8, 148u8, 32u8, 112u8] => {
                let mut rdr: &[u8] = rest;
                let fee_adjustment: i16 =
                    <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketFeeAdjustmentArguments { fee_adjustment };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketFeeAdjustmentAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketFeeAdjustment { accounts, args });
            }
            [148u8, 182u8, 3u8, 126u8, 157u8, 114u8, 220u8, 99u8] => {
                let mut rdr: &[u8] = rest;
                let fee_adjustment: i16 =
                    <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotMarketFeeAdjustmentArguments { fee_adjustment };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketFeeAdjustmentAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketFeeAdjustment { accounts, args });
            }
            [252u8, 141u8, 110u8, 101u8, 27u8, 99u8, 182u8, 21u8] => {
                let mut rdr: &[u8] = rest;
                let fuel_boost_taker: Option<u8> = parse_option(&mut rdr)?;
                let fuel_boost_maker: Option<u8> = parse_option(&mut rdr)?;
                let fuel_boost_position: Option<u8> = parse_option(&mut rdr)?;
                let args = UpdatePerpMarketFuelArguments {
                    fuel_boost_taker,
                    fuel_boost_maker,
                    fuel_boost_position,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketFuelAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketFuel { accounts, args });
            }
            [249u8, 213u8, 115u8, 34u8, 253u8, 239u8, 75u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let protected_maker_limit_price_divisor: Option<u8> = parse_option(&mut rdr)?;
                let protected_maker_dynamic_divisor: Option<u8> = parse_option(&mut rdr)?;
                let args = UpdatePerpMarketProtectedMakerParamsArguments {
                    protected_maker_limit_price_divisor,
                    protected_maker_dynamic_divisor,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketProtectedMakerParamsAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketProtectedMakerParams { accounts, args });
            }
            [31u8, 39u8, 5u8, 25u8, 228u8, 50u8, 1u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let taker_speed_bump_override: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketTakerSpeedBumpOverrideArguments {
                    taker_speed_bump_override,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketTakerSpeedBumpOverrideAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketTakerSpeedBumpOverride { accounts, args });
            }
            [155u8, 195u8, 149u8, 43u8, 220u8, 82u8, 173u8, 205u8] => {
                let mut rdr: &[u8] = rest;
                let amm_spread_adjustment: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amm_inventory_spread_adjustment: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketAmmSpreadAdjustmentArguments {
                    amm_spread_adjustment,
                    amm_inventory_spread_adjustment,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketAmmSpreadAdjustmentAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketAmmSpreadAdjustment { accounts, args });
            }
            [165u8, 91u8, 239u8, 227u8, 63u8, 172u8, 227u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let oracle_slot_delay_override: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpMarketOracleSlotDelayOverrideArguments {
                    oracle_slot_delay_override,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpMarketOracleSlotDelayOverrideAccounts {
                    admin,
                    state,
                    perpMarket,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpMarketOracleSlotDelayOverride { accounts, args });
            }
            [226u8, 253u8, 76u8, 71u8, 17u8, 2u8, 171u8, 169u8] => {
                let mut rdr: &[u8] = rest;
                let fuel_boost_deposits: Option<u8> = parse_option(&mut rdr)?;
                let fuel_boost_borrows: Option<u8> = parse_option(&mut rdr)?;
                let fuel_boost_taker: Option<u8> = parse_option(&mut rdr)?;
                let fuel_boost_maker: Option<u8> = parse_option(&mut rdr)?;
                let fuel_boost_insurance: Option<u8> = parse_option(&mut rdr)?;
                let args = UpdateSpotMarketFuelArguments {
                    fuel_boost_deposits,
                    fuel_boost_borrows,
                    fuel_boost_taker,
                    fuel_boost_maker,
                    fuel_boost_insurance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let spotMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotMarketFuelAccounts {
                    admin,
                    state,
                    spotMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotMarketFuel { accounts, args });
            }
            [132u8, 191u8, 228u8, 141u8, 201u8, 138u8, 60u8, 48u8] => {
                let mut rdr: &[u8] = rest;
                let fuel_boost_deposits: Option<i32> = parse_option(&mut rdr)?;
                let fuel_boost_borrows: Option<u32> = parse_option(&mut rdr)?;
                let fuel_boost_taker: Option<u32> = parse_option(&mut rdr)?;
                let fuel_boost_maker: Option<u32> = parse_option(&mut rdr)?;
                let fuel_boost_insurance: Option<u32> = parse_option(&mut rdr)?;
                let args = InitUserFuelArguments {
                    fuel_boost_deposits,
                    fuel_boost_borrows,
                    fuel_boost_taker,
                    fuel_boost_maker,
                    fuel_boost_insurance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let userStats = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitUserFuelAccounts {
                    admin,
                    state,
                    user,
                    userStats,
                    remaining,
                };
                return Ok(Instruction::InitUserFuel { accounts, args });
            }
            [161u8, 176u8, 40u8, 213u8, 60u8, 184u8, 179u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let admin: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateAdminArguments { admin };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAdminAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateAdmin { accounts, args });
            }
            [161u8, 15u8, 162u8, 19u8, 148u8, 120u8, 144u8, 151u8] => {
                let mut rdr: &[u8] = rest;
                let whitelist_mint: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateWhitelistMintArguments { whitelist_mint };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateWhitelistMintAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateWhitelistMint { accounts, args });
            }
            [32u8, 252u8, 122u8, 211u8, 66u8, 31u8, 47u8, 241u8] => {
                let mut rdr: &[u8] = rest;
                let discount_mint: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateDiscountMintArguments { discount_mint };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateDiscountMintAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateDiscountMint { accounts, args });
            }
            [83u8, 160u8, 252u8, 250u8, 129u8, 116u8, 49u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let exchange_status: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateExchangeStatusArguments { exchange_status };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateExchangeStatusAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateExchangeStatus { accounts, args });
            }
            [126u8, 110u8, 52u8, 174u8, 30u8, 206u8, 215u8, 90u8] => {
                let mut rdr: &[u8] = rest;
                let min_perp_auction_duration: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePerpAuctionDurationArguments {
                    min_perp_auction_duration,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePerpAuctionDurationAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdatePerpAuctionDuration { accounts, args });
            }
            [182u8, 178u8, 203u8, 72u8, 187u8, 143u8, 157u8, 107u8] => {
                let mut rdr: &[u8] = rest;
                let default_spot_auction_duration: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSpotAuctionDurationArguments {
                    default_spot_auction_duration,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let admin = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSpotAuctionDurationAccounts {
                    admin,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateSpotAuctionDuration { accounts, args });
            }
            [89u8, 131u8, 239u8, 200u8, 178u8, 141u8, 106u8, 194u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeProtocolIfSharesTransferConfigArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let protocolIfSharesTransferConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeProtocolIfSharesTransferConfigAccounts {
                    admin,
                    protocolIfSharesTransferConfig,
                    state,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeProtocolIfSharesTransferConfig {
                    accounts,
                    args,
                });
            }
            [34u8, 135u8, 47u8, 91u8, 220u8, 24u8, 212u8, 53u8] => {
                let mut rdr: &[u8] = rest;
                let whitelisted_signers: Option<[[u8; 32usize]; 4usize]> = parse_option(&mut rdr)?;
                let max_transfer_per_epoch: Option<u128> = parse_option(&mut rdr)?;
                let args = UpdateProtocolIfSharesTransferConfigArguments {
                    whitelisted_signers,
                    max_transfer_per_epoch,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let protocolIfSharesTransferConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateProtocolIfSharesTransferConfigAccounts {
                    admin,
                    protocolIfSharesTransferConfig,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateProtocolIfSharesTransferConfig { accounts, args });
            }
            [169u8, 178u8, 84u8, 25u8, 175u8, 62u8, 29u8, 247u8] => {
                let mut rdr: &[u8] = rest;
                let perp_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: Option<i64> = parse_option(&mut rdr)?;
                let max_price: Option<i64> = parse_option(&mut rdr)?;
                let params = PrelaunchOracleParams {
                    perp_market_index,
                    price,
                    max_price,
                };
                let args = InitializePrelaunchOracleArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let prelaunchOracle = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePrelaunchOracleAccounts {
                    admin,
                    prelaunchOracle,
                    state,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePrelaunchOracle { accounts, args });
            }
            [98u8, 205u8, 147u8, 243u8, 18u8, 75u8, 83u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let perp_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let price: Option<i64> = parse_option(&mut rdr)?;
                let max_price: Option<i64> = parse_option(&mut rdr)?;
                let params = PrelaunchOracleParams {
                    perp_market_index,
                    price,
                    max_price,
                };
                let args = UpdatePrelaunchOracleParamsArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let admin = keys.next().unwrap().clone();
                let prelaunchOracle = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePrelaunchOracleParamsAccounts {
                    admin,
                    prelaunchOracle,
                    perpMarket,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdatePrelaunchOracleParams { accounts, args });
            }
            [59u8, 169u8, 100u8, 49u8, 69u8, 17u8, 173u8, 253u8] => {
                let mut rdr: &[u8] = rest;
                let perp_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DeletePrelaunchOracleArguments { perp_market_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let admin = keys.next().unwrap().clone();
                let prelaunchOracle = keys.next().unwrap().clone();
                let perpMarket = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeletePrelaunchOracleAccounts {
                    admin,
                    prelaunchOracle,
                    perpMarket,
                    state,
                    remaining,
                };
                return Ok(Instruction::DeletePrelaunchOracle { accounts, args });
            }
            [249u8, 140u8, 253u8, 243u8, 248u8, 74u8, 240u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let feed_id: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializePythPullOracleArguments { feed_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let pythSolanaReceiver = keys.next().unwrap().clone();
                let priceFeed = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePythPullOracleAccounts {
                    admin,
                    pythSolanaReceiver,
                    priceFeed,
                    systemProgram,
                    state,
                    remaining,
                };
                return Ok(Instruction::InitializePythPullOracle { accounts, args });
            }
            [140u8, 107u8, 33u8, 214u8, 235u8, 219u8, 103u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let feed_id: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializePythLazerOracleArguments { feed_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let lazerOracle = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePythLazerOracleAccounts {
                    admin,
                    lazerOracle,
                    state,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePythLazerOracle { accounts, args });
            }
            [218u8, 237u8, 170u8, 245u8, 39u8, 143u8, 166u8, 33u8] => {
                let mut rdr: &[u8] = rest;
                let pyth_message: Vec<u8> =
                    <Vec<u8> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = PostPythLazerOracleUpdateArguments { pyth_message };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let keeper = keys.next().unwrap().clone();
                let pythLazerStorage = keys.next().unwrap().clone();
                let ixSysvar = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PostPythLazerOracleUpdateAccounts {
                    keeper,
                    pythLazerStorage,
                    ixSysvar,
                    remaining,
                };
                return Ok(Instruction::PostPythLazerOracleUpdate { accounts, args });
            }
            [213u8, 167u8, 93u8, 246u8, 208u8, 130u8, 90u8, 248u8] => {
                let mut rdr: &[u8] = rest;
                let max_users: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeHighLeverageModeConfigArguments { max_users };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let highLeverageModeConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeHighLeverageModeConfigAccounts {
                    admin,
                    highLeverageModeConfig,
                    state,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeHighLeverageModeConfig { accounts, args });
            }
            [64u8, 122u8, 212u8, 93u8, 141u8, 217u8, 202u8, 55u8] => {
                let mut rdr: &[u8] = rest;
                let max_users: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let current_users: Option<u32> = parse_option(&mut rdr)?;
                let args = UpdateHighLeverageModeConfigArguments {
                    max_users,
                    reduce_only,
                    current_users,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let highLeverageModeConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateHighLeverageModeConfigAccounts {
                    admin,
                    highLeverageModeConfig,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateHighLeverageModeConfig { accounts, args });
            }
            [67u8, 103u8, 220u8, 67u8, 88u8, 32u8, 252u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let max_users: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeProtectedMakerModeConfigArguments { max_users };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let protectedMakerModeConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeProtectedMakerModeConfigAccounts {
                    admin,
                    protectedMakerModeConfig,
                    state,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeProtectedMakerModeConfig { accounts, args });
            }
            [86u8, 166u8, 235u8, 253u8, 67u8, 202u8, 223u8, 17u8] => {
                let mut rdr: &[u8] = rest;
                let max_users: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reduce_only: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let current_users: Option<u32> = parse_option(&mut rdr)?;
                let args = UpdateProtectedMakerModeConfigArguments {
                    max_users,
                    reduce_only,
                    current_users,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let protectedMakerModeConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateProtectedMakerModeConfigAccounts {
                    admin,
                    protectedMakerModeConfig,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateProtectedMakerModeConfig { accounts, args });
            }
            [210u8, 66u8, 65u8, 182u8, 102u8, 214u8, 176u8, 30u8] => {
                let mut rdr: &[u8] = rest;
                let market_index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AdminDepositArguments {
                    market_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let state = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let spotMarketVault = keys.next().unwrap().clone();
                let adminTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AdminDepositAccounts {
                    state,
                    user,
                    admin,
                    spotMarketVault,
                    adminTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AdminDeposit { accounts, args });
            }
            [8u8, 85u8, 184u8, 167u8, 176u8, 61u8, 173u8, 226u8] => {
                let mut rdr: &[u8] = rest;
                let total_in_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let epoch_max_in_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let epoch_duration: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let out_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let in_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_slippage_bps: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let swap_mode: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let status: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let params = IfRebalanceConfigParams {
                    total_in_amount,
                    epoch_max_in_amount,
                    epoch_duration,
                    out_market_index,
                    in_market_index,
                    max_slippage_bps,
                    swap_mode,
                    status,
                };
                let args = InitializeIfRebalanceConfigArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let admin = keys.next().unwrap().clone();
                let ifRebalanceConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeIfRebalanceConfigAccounts {
                    admin,
                    ifRebalanceConfig,
                    state,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeIfRebalanceConfig { accounts, args });
            }
            [142u8, 245u8, 249u8, 66u8, 249u8, 181u8, 22u8, 83u8] => {
                let mut rdr: &[u8] = rest;
                let total_in_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let epoch_max_in_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let epoch_duration: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let out_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let in_market_index: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_slippage_bps: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let swap_mode: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let status: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let params = IfRebalanceConfigParams {
                    total_in_amount,
                    epoch_max_in_amount,
                    epoch_duration,
                    out_market_index,
                    in_market_index,
                    max_slippage_bps,
                    swap_mode,
                    status,
                };
                let args = UpdateIfRebalanceConfigArguments { params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let admin = keys.next().unwrap().clone();
                let ifRebalanceConfig = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateIfRebalanceConfigAccounts {
                    admin,
                    ifRebalanceConfig,
                    state,
                    remaining,
                };
                return Ok(Instruction::UpdateIfRebalanceConfig { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    pub use typedefs::*;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct NewUserRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub sub_account_id: u16,
        pub name: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub referrer: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub direction: DepositDirection,
        pub deposit_record_id: u64,
        pub amount: u64,
        pub market_index: u16,
        pub oracle_price: i64,
        pub market_deposit_balance: u128,
        pub market_withdraw_balance: u128,
        pub market_cumulative_deposit_interest: u128,
        pub market_cumulative_borrow_interest: u128,
        pub total_deposits_after: u64,
        pub total_withdraws_after: u64,
        pub explanation: DepositExplanation,
        #[serde(with = "pubkey_serde_option")]
        pub transfer_user: Option<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SpotInterestRecord {
        pub ts: i64,
        pub market_index: u16,
        pub deposit_balance: u128,
        pub cumulative_deposit_interest: u128,
        pub borrow_balance: u128,
        pub cumulative_borrow_interest: u128,
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundingPaymentRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub market_index: u16,
        pub funding_payment: i64,
        pub base_asset_amount: i64,
        pub user_last_cumulative_funding: i64,
        pub amm_cumulative_funding_long: i128,
        pub amm_cumulative_funding_short: i128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundingRateRecord {
        pub ts: i64,
        pub record_id: u64,
        pub market_index: u16,
        pub funding_rate: i64,
        pub funding_rate_long: i128,
        pub funding_rate_short: i128,
        pub cumulative_funding_rate_long: i128,
        pub cumulative_funding_rate_short: i128,
        pub oracle_price_twap: i64,
        pub mark_price_twap: u64,
        pub period_revenue: i64,
        pub base_asset_amount_with_amm: i128,
        pub base_asset_amount_with_unsettled_lp: i128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CurveRecord {
        pub ts: i64,
        pub record_id: u64,
        pub peg_multiplier_before: u128,
        pub base_asset_reserve_before: u128,
        pub quote_asset_reserve_before: u128,
        pub sqrt_k_before: u128,
        pub peg_multiplier_after: u128,
        pub base_asset_reserve_after: u128,
        pub quote_asset_reserve_after: u128,
        pub sqrt_k_after: u128,
        pub base_asset_amount_long: u128,
        pub base_asset_amount_short: u128,
        pub base_asset_amount_with_amm: i128,
        pub total_fee: i128,
        pub total_fee_minus_distributions: i128,
        pub adjustment_cost: i128,
        pub oracle_price: i64,
        pub fill_record: u128,
        pub number_of_users: u32,
        pub market_index: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SignedMsgOrderRecord {
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub hash: String,
        pub matching_order_params: OrderParams,
        pub user_order_id: u32,
        pub signed_msg_order_max_slot: u64,
        pub signed_msg_order_uuid: [u8; 8usize],
        pub ts: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OrderRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub order: Order,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OrderActionRecord {
        pub ts: i64,
        pub action: OrderAction,
        pub action_explanation: OrderActionExplanation,
        pub market_index: u16,
        pub market_type: MarketType,
        #[serde(with = "pubkey_serde_option")]
        pub filler: Option<[u8; 32usize]>,
        pub filler_reward: Option<u64>,
        pub fill_record_id: Option<u64>,
        pub base_asset_amount_filled: Option<u64>,
        pub quote_asset_amount_filled: Option<u64>,
        pub taker_fee: Option<u64>,
        pub maker_fee: Option<i64>,
        pub referrer_reward: Option<u32>,
        pub quote_asset_amount_surplus: Option<i64>,
        pub spot_fulfillment_method_fee: Option<u64>,
        #[serde(with = "pubkey_serde_option")]
        pub taker: Option<[u8; 32usize]>,
        pub taker_order_id: Option<u32>,
        pub taker_order_direction: Option<PositionDirection>,
        pub taker_order_base_asset_amount: Option<u64>,
        pub taker_order_cumulative_base_asset_amount_filled: Option<u64>,
        pub taker_order_cumulative_quote_asset_amount_filled: Option<u64>,
        #[serde(with = "pubkey_serde_option")]
        pub maker: Option<[u8; 32usize]>,
        pub maker_order_id: Option<u32>,
        pub maker_order_direction: Option<PositionDirection>,
        pub maker_order_base_asset_amount: Option<u64>,
        pub maker_order_cumulative_base_asset_amount_filled: Option<u64>,
        pub maker_order_cumulative_quote_asset_amount_filled: Option<u64>,
        pub oracle_price: i64,
        pub bit_flags: u8,
        pub taker_existing_quote_entry_amount: Option<u64>,
        pub taker_existing_base_asset_amount: Option<u64>,
        pub maker_existing_quote_entry_amount: Option<u64>,
        pub maker_existing_base_asset_amount: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LpRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub action: LpAction,
        pub n_shares: u64,
        pub market_index: u16,
        pub delta_base_asset_amount: i64,
        pub delta_quote_asset_amount: i64,
        pub pnl: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidationRecord {
        pub ts: i64,
        pub liquidation_type: LiquidationType,
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub liquidator: [u8; 32usize],
        pub margin_requirement: u128,
        pub total_collateral: i128,
        pub margin_freed: u64,
        pub liquidation_id: u16,
        pub bankrupt: bool,
        pub canceled_order_ids: Vec<u32>,
        pub liquidate_perp: LiquidatePerpRecord,
        pub liquidate_spot: LiquidateSpotRecord,
        pub liquidate_borrow_for_perp_pnl: LiquidateBorrowForPerpPnlRecord,
        pub liquidate_perp_pnl_for_deposit: LiquidatePerpPnlForDepositRecord,
        pub perp_bankruptcy: PerpBankruptcyRecord,
        pub spot_bankruptcy: SpotBankruptcyRecord,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SettlePnlRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub market_index: u16,
        pub pnl: i128,
        pub base_asset_amount: i64,
        pub quote_asset_amount_after: i64,
        pub quote_entry_amount: i64,
        pub settle_price: i64,
        pub explanation: SettlePnlExplanation,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InsuranceFundRecord {
        pub ts: i64,
        pub spot_market_index: u16,
        pub perp_market_index: u16,
        pub user_if_factor: u32,
        pub total_if_factor: u32,
        pub vault_amount_before: u64,
        pub insurance_vault_amount_before: u64,
        pub total_if_shares_before: u128,
        pub total_if_shares_after: u128,
        pub amount: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InsuranceFundStakeRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user_authority: [u8; 32usize],
        pub action: StakeAction,
        pub amount: u64,
        pub market_index: u16,
        pub insurance_vault_amount_before: u64,
        pub if_shares_before: u128,
        pub user_if_shares_before: u128,
        pub total_if_shares_before: u128,
        pub if_shares_after: u128,
        pub user_if_shares_after: u128,
        pub total_if_shares_after: u128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InsuranceFundSwapRecord {
        #[serde(with = "pubkey_serde")]
        pub rebalance_config: [u8; 32usize],
        pub in_if_total_shares_before: u128,
        pub out_if_total_shares_before: u128,
        pub in_if_user_shares_before: u128,
        pub out_if_user_shares_before: u128,
        pub in_if_total_shares_after: u128,
        pub out_if_total_shares_after: u128,
        pub in_if_user_shares_after: u128,
        pub out_if_user_shares_after: u128,
        pub ts: i64,
        pub in_amount: u64,
        pub out_amount: u64,
        pub out_oracle_price: u64,
        pub out_oracle_price_twap: i64,
        pub in_vault_amount_before: u64,
        pub out_vault_amount_before: u64,
        pub in_fund_vault_amount_after: u64,
        pub out_fund_vault_amount_after: u64,
        pub in_market_index: u16,
        pub out_market_index: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferProtocolIfSharesToRevenuePoolRecord {
        pub ts: i64,
        pub market_index: u16,
        pub amount: u64,
        pub shares: u128,
        pub if_vault_amount_before: u64,
        pub protocol_shares_before: u128,
        pub transfer_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub amount_out: u64,
        pub amount_in: u64,
        pub out_market_index: u16,
        pub in_market_index: u16,
        pub out_oracle_price: i64,
        pub in_oracle_price: i64,
        pub fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SpotMarketVaultDepositRecord {
        pub ts: i64,
        pub market_index: u16,
        pub deposit_balance: u128,
        pub cumulative_deposit_interest_before: u128,
        pub cumulative_deposit_interest_after: u128,
        pub deposit_token_amount_before: u64,
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeleteUserRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub user_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub user: [u8; 32usize],
        pub sub_account_id: u16,
        #[serde(with = "pubkey_serde_option")]
        pub keeper: Option<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FuelSweepRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        pub user_stats_fuel_insurance: u32,
        pub user_stats_fuel_deposits: u32,
        pub user_stats_fuel_borrows: u32,
        pub user_stats_fuel_positions: u32,
        pub user_stats_fuel_taker: u32,
        pub user_stats_fuel_maker: u32,
        pub fuel_overflow_fuel_insurance: u128,
        pub fuel_overflow_fuel_deposits: u128,
        pub fuel_overflow_fuel_borrows: u128,
        pub fuel_overflow_fuel_positions: u128,
        pub fuel_overflow_fuel_taker: u128,
        pub fuel_overflow_fuel_maker: u128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FuelSeasonRecord {
        pub ts: i64,
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        pub fuel_insurance: u128,
        pub fuel_deposits: u128,
        pub fuel_borrows: u128,
        pub fuel_positions: u128,
        pub fuel_taker: u128,
        pub fuel_maker: u128,
        pub fuel_total: u128,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        NewUserRecord {
            args: NewUserRecord,
        },
        DepositRecord {
            args: DepositRecord,
        },
        SpotInterestRecord {
            args: SpotInterestRecord,
        },
        FundingPaymentRecord {
            args: FundingPaymentRecord,
        },
        FundingRateRecord {
            args: FundingRateRecord,
        },
        CurveRecord {
            args: CurveRecord,
        },
        SignedMsgOrderRecord {
            args: SignedMsgOrderRecord,
        },
        OrderRecord {
            args: OrderRecord,
        },
        OrderActionRecord {
            args: OrderActionRecord,
        },
        LpRecord {
            args: LpRecord,
        },
        LiquidationRecord {
            args: LiquidationRecord,
        },
        SettlePnlRecord {
            args: SettlePnlRecord,
        },
        InsuranceFundRecord {
            args: InsuranceFundRecord,
        },
        InsuranceFundStakeRecord {
            args: InsuranceFundStakeRecord,
        },
        InsuranceFundSwapRecord {
            args: InsuranceFundSwapRecord,
        },
        TransferProtocolIfSharesToRevenuePoolRecord {
            args: TransferProtocolIfSharesToRevenuePoolRecord,
        },
        SwapRecord {
            args: SwapRecord,
        },
        SpotMarketVaultDepositRecord {
            args: SpotMarketVaultDepositRecord,
        },
        DeleteUserRecord {
            args: DeleteUserRecord,
        },
        FuelSweepRecord {
            args: FuelSweepRecord,
        },
        FuelSeasonRecord {
            args: FuelSeasonRecord,
        },
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
            let (wrapper, rest) = data.split_at(8);
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
                [236u8, 186u8, 113u8, 219u8, 42u8, 51u8, 149u8, 249u8] => {
                    let mut rdr = &payload[..];
                    let args = NewUserRecord::deserialize(&mut rdr)?;
                    return Ok(Event::NewUserRecord { args });
                }
                [180u8, 241u8, 218u8, 207u8, 102u8, 135u8, 44u8, 134u8] => {
                    let mut rdr = &payload[..];
                    let args = DepositRecord::deserialize(&mut rdr)?;
                    return Ok(Event::DepositRecord { args });
                }
                [183u8, 186u8, 203u8, 186u8, 225u8, 187u8, 95u8, 130u8] => {
                    let mut rdr = &payload[..];
                    let args = SpotInterestRecord::deserialize(&mut rdr)?;
                    return Ok(Event::SpotInterestRecord { args });
                }
                [8u8, 59u8, 96u8, 20u8, 137u8, 201u8, 56u8, 95u8] => {
                    let mut rdr = &payload[..];
                    let args = FundingPaymentRecord::deserialize(&mut rdr)?;
                    return Ok(Event::FundingPaymentRecord { args });
                }
                [68u8, 3u8, 255u8, 26u8, 133u8, 91u8, 147u8, 254u8] => {
                    let mut rdr = &payload[..];
                    let args = FundingRateRecord::deserialize(&mut rdr)?;
                    return Ok(Event::FundingRateRecord { args });
                }
                [101u8, 238u8, 40u8, 228u8, 70u8, 46u8, 61u8, 117u8] => {
                    let mut rdr = &payload[..];
                    let args = CurveRecord::deserialize(&mut rdr)?;
                    return Ok(Event::CurveRecord { args });
                }
                [211u8, 197u8, 25u8, 18u8, 142u8, 86u8, 113u8, 27u8] => {
                    let mut rdr = &payload[..];
                    let args = SignedMsgOrderRecord::deserialize(&mut rdr)?;
                    return Ok(Event::SignedMsgOrderRecord { args });
                }
                [104u8, 19u8, 64u8, 56u8, 89u8, 21u8, 2u8, 90u8] => {
                    let mut rdr = &payload[..];
                    let args = OrderRecord::deserialize(&mut rdr)?;
                    return Ok(Event::OrderRecord { args });
                }
                [224u8, 52u8, 67u8, 71u8, 194u8, 237u8, 109u8, 1u8] => {
                    let mut rdr = &payload[..];
                    let args = OrderActionRecord::deserialize(&mut rdr)?;
                    return Ok(Event::OrderActionRecord { args });
                }
                [101u8, 22u8, 54u8, 38u8, 178u8, 13u8, 142u8, 111u8] => {
                    let mut rdr = &payload[..];
                    let args = LpRecord::deserialize(&mut rdr)?;
                    return Ok(Event::LpRecord { args });
                }
                [127u8, 17u8, 0u8, 108u8, 182u8, 13u8, 231u8, 53u8] => {
                    let mut rdr = &payload[..];
                    let args = LiquidationRecord::deserialize(&mut rdr)?;
                    return Ok(Event::LiquidationRecord { args });
                }
                [57u8, 68u8, 105u8, 26u8, 119u8, 198u8, 213u8, 89u8] => {
                    let mut rdr = &payload[..];
                    let args = SettlePnlRecord::deserialize(&mut rdr)?;
                    return Ok(Event::SettlePnlRecord { args });
                }
                [56u8, 222u8, 215u8, 235u8, 78u8, 197u8, 99u8, 146u8] => {
                    let mut rdr = &payload[..];
                    let args = InsuranceFundRecord::deserialize(&mut rdr)?;
                    return Ok(Event::InsuranceFundRecord { args });
                }
                [68u8, 66u8, 156u8, 7u8, 216u8, 148u8, 250u8, 114u8] => {
                    let mut rdr = &payload[..];
                    let args = InsuranceFundStakeRecord::deserialize(&mut rdr)?;
                    return Ok(Event::InsuranceFundStakeRecord { args });
                }
                [85u8, 190u8, 99u8, 203u8, 237u8, 33u8, 227u8, 100u8] => {
                    let mut rdr = &payload[..];
                    let args = InsuranceFundSwapRecord::deserialize(&mut rdr)?;
                    return Ok(Event::InsuranceFundSwapRecord { args });
                }
                [209u8, 118u8, 142u8, 167u8, 130u8, 46u8, 164u8, 151u8] => {
                    let mut rdr = &payload[..];
                    let args = TransferProtocolIfSharesToRevenuePoolRecord::deserialize(&mut rdr)?;
                    return Ok(Event::TransferProtocolIfSharesToRevenuePoolRecord { args });
                }
                [162u8, 187u8, 123u8, 194u8, 138u8, 56u8, 250u8, 241u8] => {
                    let mut rdr = &payload[..];
                    let args = SwapRecord::deserialize(&mut rdr)?;
                    return Ok(Event::SwapRecord { args });
                }
                [178u8, 217u8, 23u8, 188u8, 127u8, 190u8, 32u8, 73u8] => {
                    let mut rdr = &payload[..];
                    let args = SpotMarketVaultDepositRecord::deserialize(&mut rdr)?;
                    return Ok(Event::SpotMarketVaultDepositRecord { args });
                }
                [71u8, 111u8, 190u8, 118u8, 7u8, 3u8, 132u8, 222u8] => {
                    let mut rdr = &payload[..];
                    let args = DeleteUserRecord::deserialize(&mut rdr)?;
                    return Ok(Event::DeleteUserRecord { args });
                }
                [41u8, 84u8, 37u8, 246u8, 132u8, 240u8, 131u8, 8u8] => {
                    let mut rdr = &payload[..];
                    let args = FuelSweepRecord::deserialize(&mut rdr)?;
                    return Ok(Event::FuelSweepRecord { args });
                }
                [19u8, 137u8, 119u8, 33u8, 224u8, 249u8, 6u8, 87u8] => {
                    let mut rdr = &payload[..];
                    let args = FuelSeasonRecord::deserialize(&mut rdr)?;
                    return Ok(Event::FuelSeasonRecord { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
