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
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PositionRewardInfo {
        pub growth_inside_checkpoint: u128,
        pub amount_owed: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WhirlpoolRewardInfo {
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vault: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        pub emissions_per_second_x64: u128,
        pub growth_global_x64: u128,
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
    pub struct RebalanceRaw {
        #[serde(with = "BigArray")]
        pub params: [u8; 128usize],
        #[serde(with = "BigArray")]
        pub state: [u8; 256usize],
        pub reference_price_type: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CollateralInfo {
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        pub lower_heuristic: u64,
        pub upper_heuristic: u64,
        pub exp_heuristic: u64,
        pub max_twap_divergence_bps: u64,
        pub scope_twap_price_chain: [u16; 4usize],
        pub scope_price_chain: [u16; 4usize],
        pub name: [u8; 32usize],
        pub max_age_price_seconds: u64,
        pub max_age_twap_seconds: u64,
        pub max_ignorable_amount_as_reward: u64,
        pub disabled: u8,
        pub padding0: [u8; 7usize],
        pub scope_staking_rate_chain: [u16; 4usize],
        pub padding: [u64; 8usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CollateralInfoParams {
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        pub lower_heuristic: u64,
        pub upper_heuristic: u64,
        pub exp_heuristic: u64,
        pub max_twap_divergence_bps: u64,
        pub scope_twap_price_chain: [u16; 4usize],
        pub scope_price_chain: [u16; 4usize],
        pub name: [u8; 32usize],
        pub max_age_price_seconds: u64,
        pub max_age_twap_seconds: u64,
        pub max_ignorable_amount_as_reward: u64,
        pub disabled: u8,
        pub scope_staking_rate_chain: [u16; 4usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct KaminoRewardInfo {
        pub decimals: u64,
        #[serde(with = "pubkey_serde")]
        pub reward_vault: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub reward_mint: [u8; 32usize],
        pub reward_collateral_id: u64,
        pub last_issuance_ts: u64,
        pub reward_per_second: u64,
        pub amount_uncollected: u64,
        pub amount_issued_cumulative: u64,
        pub amount_available: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawalCaps {
        pub config_capacity: i64,
        pub current_total: i64,
        pub last_interval_start_timestamp: u64,
        pub config_interval_length_seconds: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Price {
        pub value: u64,
        pub exp: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceAutodriftParams {
        pub init_drift_ticks_per_epoch: u32,
        pub ticks_below_mid: i32,
        pub ticks_above_mid: i32,
        pub frontrun_multiplier_bps: u16,
        pub staking_rate_a_source: StakingRateSource,
        pub staking_rate_b_source: StakingRateSource,
        pub init_drift_direction: DriftDirection,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceAutodriftWindow {
        pub staking_rate_a: Option<Price>,
        pub staking_rate_b: Option<Price>,
        pub epoch: u64,
        pub theoretical_tick: i32,
        pub strat_mid_tick: i32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceAutodriftState {
        pub last_window: RebalanceAutodriftWindow,
        pub current_window: RebalanceAutodriftWindow,
        pub step: RebalanceAutodriftStep,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceDriftParams {
        pub start_mid_tick: i32,
        pub ticks_below_mid: i32,
        pub ticks_above_mid: i32,
        pub seconds_per_tick: u64,
        pub direction: DriftDirection,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceDriftState {
        pub step: RebalanceDriftStep,
        pub last_drift_timestamp: u64,
        pub last_mid_tick: i32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceExpanderState {
        pub initial_pool_price: u128,
        pub expansion_count: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceManualState {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PeriodicRebalanceState {
        pub last_rebalance_timestamp: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalancePricePercentageWithResetState {
        pub last_rebalance_lower_reset_pool_price: u128,
        pub last_rebalance_upper_reset_pool_price: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalancePricePercentageState {
        pub last_rebalance_lower_pool_price: u128,
        pub last_rebalance_upper_pool_price: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RebalanceTakeProfitState {
        pub step: RebalanceTakeProfitStep,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum BinAddLiquidityStrategy {
        Uniform {
            current_bin_index: i32,
            lower_bin_index: i32,
            upper_bin_index: i32,
            amount_x_to_deposit: u64,
            amount_y_to_deposit: u64,
            x_current_bin: u64,
            y_current_bin: u64,
        },
        CurrentTick(i32),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SimulationPrice {
        PoolPrice,
        SqrtPrice(u128),
        TickIndex(i32),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DexSpecificPrice {
        SqrtPrice(u128),
        Q6464(u128),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RemoveLiquidityMode {
        Liquidity(u128),
        Bps(u16),
        All,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum WithdrawalCapAccumulatorAction {
        KeepAccumulator,
        ResetAccumulator,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceEffects {
        NewRange(i32, i32),
        WithdrawAndFreeze,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SwapLimit {
        Bps(u64),
        Absolute {
            src_amount_to_swap: u64,
            dst_amount_to_vault: u64,
            a_to_b: bool,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MintingMethod {
        PriceBased,
        Proportional,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum GlobalConfigOption {
        EmergencyMode,
        BlockDeposit,
        BlockInvest,
        BlockWithdraw,
        BlockCollectFees,
        BlockCollectRewards,
        BlockSwapRewards,
        BlockSwapUnevenVaults,
        WithdrawalFeeBps,
        SwapDiscountBps,
        ActionsAuthority,
        DeprecatedTreasuryFeeVaults,
        AdminAuthority,
        BlockEmergencySwap,
        BlockLocalAdmin,
        UpdateTokenInfos,
        ScopeProgramId,
        ScopePriceId,
        MinPerformanceFeeBps,
        MinSwapUnevenSlippageToleranceBps,
        MinReferencePriceSlippageToleranceBps,
        ActionsAfterRebalanceDelaySeconds,
        TreasuryFeeVaultReceiver,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StrategyConfigOption {
        UpdateDepositCap,
        UpdateDepositCapIxn,
        UpdateWithdrawalCapACapacity,
        UpdateWithdrawalCapAInterval,
        UpdateWithdrawalCapACurrentTotal,
        UpdateWithdrawalCapBCapacity,
        UpdateWithdrawalCapBInterval,
        UpdateWithdrawalCapBCurrentTotal,
        UpdateMaxDeviationBps,
        UpdateSwapVaultMaxSlippage,
        UpdateStrategyType,
        UpdateDepositFee,
        UpdateWithdrawFee,
        UpdateCollectFeesFee,
        UpdateReward0Fee,
        UpdateReward1Fee,
        UpdateReward2Fee,
        UpdateAdminAuthority,
        KaminoRewardIndex0Ts,
        KaminoRewardIndex1Ts,
        KaminoRewardIndex2Ts,
        KaminoRewardIndex0RewardPerSecond,
        KaminoRewardIndex1RewardPerSecond,
        KaminoRewardIndex2RewardPerSecond,
        UpdateDepositBlocked,
        UpdateRaydiumProtocolPositionOrBaseVaultAuthority,
        UpdateRaydiumPoolConfigOrBaseVaultAuthority,
        UpdateInvestBlocked,
        UpdateWithdrawBlocked,
        UpdateLocalAdminBlocked,
        DeprecatedUpdateCollateralIdA,
        DeprecatedUpdateCollateralIdB,
        UpdateFlashVaultSwap,
        AllowDepositWithoutInvest,
        UpdateSwapVaultMaxSlippageFromRef,
        ResetReferencePrices,
        UpdateStrategyCreationState,
        UpdateIsCommunity,
        UpdateRebalanceType,
        UpdateRebalanceParams,
        UpdateDepositMintingMethod,
        UpdateLookupTable,
        UpdateReferencePriceType,
        UpdateReward0Amount,
        UpdateReward1Amount,
        UpdateReward2Amount,
        UpdateFarm,
        UpdateRebalancesCapCapacity,
        UpdateRebalancesCapInterval,
        UpdateRebalancesCapCurrentTotal,
        UpdateSwapUnevenAuthority,
        UpdatePendingStrategyAdmin,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StrategyStatus {
        Uninitialized,
        Active,
        Frozen,
        Rebalancing,
        NoPosition,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StrategyType {
        Stable,
        Pegged,
        Volatile,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CreationStatus {
        Ignored,
        Shadow,
        Live,
        Deprecated,
        Staging,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExecutiveWithdrawAction {
        Freeze,
        Unfreeze,
        Rebalance,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ReferencePriceType {
        Pool,
        Twap,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LiquidityCalculationMode {
        Deposit,
        Withdraw,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateCollateralInfoMode {
        CollateralId,
        LowerHeuristic,
        UpperHeuristic,
        ExpHeuristic,
        TwapDivergence,
        UpdateScopeTwap,
        UpdateScopeChain,
        UpdateName,
        UpdatePriceMaxAge,
        UpdateTwapMaxAge,
        UpdateDisabled,
        UpdateStakingRateChain,
        UpdateMaxIgnorableAmountAsReward,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum BalanceStatus {
        Balanced,
        Unbalanced,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceAutodriftStep {
        Uninitialized,
        Autodrifting,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StakingRateSource {
        Constant,
        Scope,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DriftDirection {
        Increasing,
        Decreasing,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceDriftStep {
        Uninitialized,
        Drifting,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExpanderStep {
        ExpandOrContract(u16),
        Recenter,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceTakeProfitToken {
        A,
        B,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceTakeProfitStep {
        Uninitialized,
        TakingProfit,
        Finished,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceAction {
        NewPriceRange(DexSpecificPrice, DexSpecificPrice),
        NewTickRange(i32, i32),
        WithdrawAndFreeze,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RebalanceType {
        Manual,
        PricePercentage,
        PricePercentageWithReset,
        Drift,
        TakeProfit,
        PeriodicRebalance,
        Expander,
        Autodrift,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CollateralTestToken {
        Usdc,
        Usdh,
        Sol,
        Eth,
        Btc,
        Msol,
        Stsol,
        Usdt,
        Orca,
        Mnde,
        Hbb,
        Jsol,
        Ush,
        Dai,
        Ldo,
        Scnsol,
        Uxd,
        Hdg,
        Dust,
        Usdr,
        Ratio,
        Uxp,
        Jitosol,
        Ray,
        Bonk,
        Samo,
        LaineSol,
        Bsol,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ScopePriceIdTest {
        Sol,
        Eth,
        Btc,
        Srm,
        Ray,
        Ftt,
        Msol,
        ScnSolSol,
        Bnb,
        Avax,
        DaoSolSol,
        SaberMsolSol,
        Usdh,
        StSol,
        CsolSol,
        CethEth,
        CbtcBtc,
        CmsolSol,
        WstEth,
        Ldo,
        Usdc,
        CusdcUsdc,
        Usdt,
        Orca,
        Mnde,
        Hbb,
        CorcaOrca,
        CslndSlnd,
        CsrmSrm,
        CrayRay,
        CfttFtt,
        CstsolStsol,
        Slnd,
        Dai,
        JsolSol,
        Ush,
        Uxd,
        UsdhTwap,
        UshTwap,
        UxdTwap,
        Hdg,
        Dust,
        Usdr,
        UsdrTwap,
        Ratio,
        Uxp,
        Kuxdusdcorca,
        JitosolSol,
        SolEma,
        EthEma,
        BtcEma,
        SrmEma,
        RayEma,
        FttEma,
        MsolEma,
        BnbEma,
        AvaxEma,
        StsolEma,
        UsdcEma,
        UsdtEma,
        SlndEma,
        DaiEma,
        WstEthTwap,
        DustTwap,
        Bonk,
        BonkTwap,
        Samo,
        SamoTwap,
        Bsol,
        LaineSol,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Dex {
        Orca,
        Raydium,
        Meteora,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeStrategyAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub pool: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub baseVaultAuthority: String,
        pub sharesMint: String,
        pub sharesMintAuthority: String,
        pub tokenInfos: String,
        pub systemProgram: String,
        pub rent: String,
        pub tokenProgram: String,
        pub tokenATokenProgram: String,
        pub tokenBTokenProgram: String,
        pub strategy: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeKaminoRewardAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub globalConfig: String,
        pub rewardMint: String,
        pub rewardVault: String,
        pub tokenInfos: String,
        pub baseVaultAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddKaminoRewardsAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub rewardMint: String,
        pub rewardVault: String,
        pub baseVaultAuthority: String,
        pub rewardAta: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeGlobalConfigAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeCollateralInfoAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub collInfo: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateCollateralInfoAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub tokenInfos: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InsertCollateralInfoAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub tokenInfos: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeSharesMetadataAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub globalConfig: String,
        pub sharesMint: String,
        pub sharesMetadata: String,
        pub sharesMintAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub metadataProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSharesMetadataAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub globalConfig: String,
        pub sharesMint: String,
        pub sharesMetadata: String,
        pub sharesMintAuthority: String,
        pub metadataProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateTreasuryFeeVaultAccounts {
        pub signer: String,
        pub globalConfig: String,
        pub feeMint: String,
        pub treasuryFeeVault: String,
        pub treasuryFeeVaultAuthority: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenInfos: Option<String>,
        pub systemProgram: String,
        pub rent: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStrategyConfigAccounts {
        pub adminAuthority: String,
        pub newAccount: String,
        pub strategy: String,
        pub globalConfig: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateRewardMappingAccounts {
        pub payer: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub rewardMint: String,
        pub rewardVault: String,
        pub baseVaultAuthority: String,
        pub tokenInfos: String,
        pub systemProgram: String,
        pub rent: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenLiquidityPositionAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub baseVaultAuthority: String,
        pub position: String,
        pub positionMint: String,
        pub positionMetadataAccount: String,
        pub positionTokenAccount: String,
        pub rent: String,
        pub system: String,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram2022: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub associatedTokenProgram: String,
        pub poolProgram: String,
        pub oldTickArrayLowerOrBaseVaultAuthority: String,
        pub oldTickArrayUpperOrBaseVaultAuthority: String,
        pub oldPositionOrBaseVaultAuthority: String,
        pub oldPositionMintOrBaseVaultAuthority: String,
        pub oldPositionTokenAccountOrBaseVaultAuthority: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenAMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBMint: Option<String>,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scopePrices: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenInfos: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub consensusAccount: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseStrategyAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub oldPositionOrBaseVaultAuthority: String,
        pub oldPositionMintOrBaseVaultAuthority: String,
        pub oldPositionTokenAccountOrBaseVaultAuthority: String,
        pub oldTickArrayLowerOrBaseVaultAuthority: String,
        pub oldTickArrayUpperOrBaseVaultAuthority: String,
        pub pool: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub userTokenAAta: String,
        pub userTokenBAta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenAMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBMint: Option<String>,
        pub reward0Vault: String,
        pub reward1Vault: String,
        pub reward2Vault: String,
        pub kaminoReward0Vault: String,
        pub kaminoReward1Vault: String,
        pub kaminoReward2Vault: String,
        pub userReward0Ata: String,
        pub userReward1Ata: String,
        pub userReward2Ata: String,
        pub userKaminoReward0Ata: String,
        pub userKaminoReward1Ata: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub userKaminoReward2Ata: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub baseVaultAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub poolProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub system: Option<String>,
        pub eventAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub position: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub baseVaultAuthority: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub userSharesAta: String,
        pub sharesMint: String,
        pub sharesMintAuthority: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instructionSysvarAccount: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InvestAccounts {
        pub payer: String,
        pub strategy: String,
        pub globalConfig: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenAMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBMint: Option<String>,
        pub baseVaultAuthority: String,
        pub pool: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram2022: Option<String>,
        pub position: String,
        pub raydiumProtocolPositionOrBaseVaultAuthority: String,
        pub positionTokenAccount: String,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        pub poolProgram: String,
        pub instructionSysvarAccount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAndInvestAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub position: String,
        pub raydiumProtocolPositionOrBaseVaultAuthority: String,
        pub positionTokenAccount: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub baseVaultAuthority: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub userSharesAta: String,
        pub sharesMint: String,
        pub sharesMintAuthority: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub poolProgram: String,
        pub instructionSysvarAccount: String,
        pub eventAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub position: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub baseVaultAuthority: String,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub userSharesAta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sharesMint: Option<String>,
        pub treasuryFeeTokenAVault: String,
        pub treasuryFeeTokenBVault: String,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram2022: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub positionTokenAccount: String,
        pub poolProgram: String,
        pub instructionSysvarAccount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ExecutiveWithdrawAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub position: String,
        pub raydiumProtocolPositionOrBaseVaultAuthority: String,
        pub positionTokenAccount: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub baseVaultAuthority: String,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram2022: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub poolProgram: Option<String>,
        pub eventAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectFeesAndRewardsAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub baseVaultAuthority: String,
        pub pool: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub position: String,
        pub raydiumProtocolPositionOrBaseVaultAuthority: String,
        pub positionTokenAccount: String,
        pub tokenAVault: String,
        pub poolTokenVaultA: String,
        pub tokenBVault: String,
        pub poolTokenVaultB: String,
        pub treasuryFeeTokenAVault: String,
        pub treasuryFeeTokenBVault: String,
        pub treasuryFeeVaultAuthority: String,
        pub reward0Vault: String,
        pub reward1Vault: String,
        pub reward2Vault: String,
        pub poolRewardVault0: String,
        pub poolRewardVault1: String,
        pub poolRewardVault2: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenProgram2022: Option<String>,
        pub poolProgram: String,
        pub instructionSysvarAccount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapRewardsAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub rewardVault: String,
        pub baseVaultAuthority: String,
        pub treasuryFeeTokenAVault: String,
        pub treasuryFeeTokenBVault: String,
        pub treasuryFeeVaultAuthority: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rewardMint: Option<String>,
        pub userTokenAAta: String,
        pub userTokenBAta: String,
        pub userRewardTokenAccount: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        pub systemProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rewardTokenProgram: Option<String>,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CheckExpectedVaultsBalancesAccounts {
        pub user: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SingleTokenDepositAndInvestWithMinAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub position: String,
        pub raydiumProtocolPositionOrBaseVaultAuthority: String,
        pub positionTokenAccount: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub baseVaultAuthority: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub userSharesAta: String,
        pub sharesMint: String,
        pub sharesMintAuthority: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        pub tokenProgram: String,
        pub tokenProgram2022: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub poolProgram: String,
        pub instructionSysvarAccount: String,
        pub eventAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SingleTokenDepositWithMinAccounts {
        pub user: String,
        pub strategy: String,
        pub globalConfig: String,
        pub pool: String,
        pub position: String,
        pub tickArrayLower: String,
        pub tickArrayUpper: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub baseVaultAuthority: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub userSharesAta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sharesMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sharesMintAuthority: Option<String>,
        pub scopePrices: String,
        pub tokenInfos: String,
        pub tokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashSwapUnevenVaultsStartAccounts {
        pub swapper: String,
        pub strategy: String,
        pub globalConfig: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub baseVaultAuthority: String,
        pub pool: String,
        pub position: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tickArrayLower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tickArrayUpper: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenAMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        pub instructionSysvarAccount: String,
        pub consensusAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashSwapUnevenVaultsEndAccounts {
        pub swapper: String,
        pub strategy: String,
        pub globalConfig: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub tokenAAta: String,
        pub tokenBAta: String,
        pub baseVaultAuthority: String,
        pub pool: String,
        pub position: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub TokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tickArrayLower: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tickArrayUpper: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenAMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBMint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        pub instructionSysvarAccount: String,
        pub consensusAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EmergencySwapAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub globalConfig: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub baseVaultAuthority: String,
        pub pool: String,
        pub position: String,
        pub poolTokenVaultA: String,
        pub poolTokenVaultB: String,
        pub tickArray0: String,
        pub tickArray1: String,
        pub tickArray2: String,
        pub oracle: String,
        pub poolProgram: String,
        pub scopePrices: String,
        pub tokenInfos: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawFromTreasuryAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub mint: String,
        pub treasuryFeeVault: String,
        pub treasuryFeeVaultAuthority: String,
        pub tokenAccount: String,
        pub systemProgram: String,
        pub rent: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PermisionlessWithdrawFromTreasuryAccounts {
        pub signer: String,
        pub globalConfig: String,
        pub mint: String,
        pub treasuryFeeVault: String,
        pub treasuryFeeVaultAuthority: String,
        pub tokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawFromTopupAccounts {
        pub adminAuthority: String,
        pub topupVault: String,
        pub system: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ChangePoolAccounts {
        pub adminAuthority: String,
        pub strategy: String,
        pub oldPosition: String,
        pub baseVaultAuthority: String,
        pub newPool: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub strategyRewardVault0OrBaseVaultAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub strategyRewardVault1OrBaseVaultAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub strategyRewardVault2OrBaseVaultAuthority: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseProgramAccountAccounts {
        pub adminAuthority: String,
        pub program: String,
        pub programData: String,
        pub closingAccount: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OrcaSwapAccounts {
        pub funder: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenATokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tokenBTokenProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memoProgram: Option<String>,
        pub tokenAuthority: String,
        pub whirlpool: String,
        pub tokenOwnerAccountA: String,
        pub tokenVaultA: String,
        pub tokenOwnerAccountB: String,
        pub tokenVaultB: String,
        pub tokenMintA: String,
        pub tokenMintB: String,
        pub tickArray0: String,
        pub tickArray1: String,
        pub tickArray2: String,
        pub oracle: String,
        pub whirlpoolProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SignTermsAccounts {
        pub owner: String,
        pub ownerSignatureState: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStrategyAdminAccounts {
        pub pendingAdmin: String,
        pub strategy: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeStrategyArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub strategy_type: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_a_collateral_id: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_b_collateral_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeKaminoRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub kamino_reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_token: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddKaminoRewardsArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub kamino_reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeGlobalConfigArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeCollateralInfoArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateCollateralInfoArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub mode: u64,
        pub value: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InsertCollateralInfoArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub index: u64,
        pub params: CollateralInfoParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeSharesMetadataArguments {
        pub name: String,
        pub symbol: String,
        pub uri: String,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateSharesMetadataArguments {
        pub name: String,
        pub symbol: String,
        pub uri: String,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigArguments {
        pub key: u16,
        pub index: u16,
        pub value: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateTreasuryFeeVaultArguments {
        pub collateral_id: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateStrategyConfigArguments {
        pub mode: u16,
        #[serde(with = "BigArray")]
        pub value: [u8; 128usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardMappingArguments {
        pub reward_index: u8,
        pub collateral_token: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OpenLiquidityPositionArguments {
        pub tick_lower_index: i64,
        pub tick_upper_index: i64,
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseStrategyArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_max_a: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_max_b: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InvestArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositAndInvestArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_max_a: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_max_b: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExecutiveWithdrawArguments {
        pub action: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectFeesAndRewardsArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapRewardsArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_a_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_b_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_collateral_id: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_collateral_token_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CheckExpectedVaultsBalancesArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_a_ata_balance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_b_ata_balance: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SingleTokenDepositAndInvestWithMinArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_a_min_post_deposit_balance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_b_min_post_deposit_balance: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SingleTokenDepositWithMinArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_a_min_post_deposit_balance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub token_b_min_post_deposit_balance: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashSwapUnevenVaultsStartArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub a_to_b: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashSwapUnevenVaultsEndArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_repay_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_to_leave_to_user: u64,
        pub a_to_b: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EmergencySwapArguments {
        pub a_to_b: bool,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub target_limit_bps: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawFromTreasuryArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PermisionlessWithdrawFromTreasuryArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawFromTopupArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ChangePoolArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseProgramAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OrcaSwapArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub other_amount_threshold: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub sqrt_price_limit: u128,
        pub amount_specified_is_input: bool,
        pub a_to_b: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SignTermsArguments {
        #[serde(with = "BigArray")]
        pub signature: [u8; 64usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateStrategyAdminArguments {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeStrategy {
        accounts: InitializeStrategyAccounts,
        args: InitializeStrategyArguments,
    },
    InitializeKaminoReward {
        accounts: InitializeKaminoRewardAccounts,
        args: InitializeKaminoRewardArguments,
    },
    AddKaminoRewards {
        accounts: AddKaminoRewardsAccounts,
        args: AddKaminoRewardsArguments,
    },
    InitializeGlobalConfig {
        accounts: InitializeGlobalConfigAccounts,
        args: InitializeGlobalConfigArguments,
    },
    InitializeCollateralInfo {
        accounts: InitializeCollateralInfoAccounts,
        args: InitializeCollateralInfoArguments,
    },
    UpdateCollateralInfo {
        accounts: UpdateCollateralInfoAccounts,
        args: UpdateCollateralInfoArguments,
    },
    InsertCollateralInfo {
        accounts: InsertCollateralInfoAccounts,
        args: InsertCollateralInfoArguments,
    },
    InitializeSharesMetadata {
        accounts: InitializeSharesMetadataAccounts,
        args: InitializeSharesMetadataArguments,
    },
    UpdateSharesMetadata {
        accounts: UpdateSharesMetadataAccounts,
        args: UpdateSharesMetadataArguments,
    },
    UpdateGlobalConfig {
        accounts: UpdateGlobalConfigAccounts,
        args: UpdateGlobalConfigArguments,
    },
    UpdateTreasuryFeeVault {
        accounts: UpdateTreasuryFeeVaultAccounts,
        args: UpdateTreasuryFeeVaultArguments,
    },
    UpdateStrategyConfig {
        accounts: UpdateStrategyConfigAccounts,
        args: UpdateStrategyConfigArguments,
    },
    UpdateRewardMapping {
        accounts: UpdateRewardMappingAccounts,
        args: UpdateRewardMappingArguments,
    },
    OpenLiquidityPosition {
        accounts: OpenLiquidityPositionAccounts,
        args: OpenLiquidityPositionArguments,
    },
    CloseStrategy {
        accounts: CloseStrategyAccounts,
        args: CloseStrategyArguments,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArguments,
    },
    Invest {
        accounts: InvestAccounts,
        args: InvestArguments,
    },
    DepositAndInvest {
        accounts: DepositAndInvestAccounts,
        args: DepositAndInvestArguments,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArguments,
    },
    ExecutiveWithdraw {
        accounts: ExecutiveWithdrawAccounts,
        args: ExecutiveWithdrawArguments,
    },
    CollectFeesAndRewards {
        accounts: CollectFeesAndRewardsAccounts,
        args: CollectFeesAndRewardsArguments,
    },
    SwapRewards {
        accounts: SwapRewardsAccounts,
        args: SwapRewardsArguments,
    },
    CheckExpectedVaultsBalances {
        accounts: CheckExpectedVaultsBalancesAccounts,
        args: CheckExpectedVaultsBalancesArguments,
    },
    SingleTokenDepositAndInvestWithMin {
        accounts: SingleTokenDepositAndInvestWithMinAccounts,
        args: SingleTokenDepositAndInvestWithMinArguments,
    },
    SingleTokenDepositWithMin {
        accounts: SingleTokenDepositWithMinAccounts,
        args: SingleTokenDepositWithMinArguments,
    },
    FlashSwapUnevenVaultsStart {
        accounts: FlashSwapUnevenVaultsStartAccounts,
        args: FlashSwapUnevenVaultsStartArguments,
    },
    FlashSwapUnevenVaultsEnd {
        accounts: FlashSwapUnevenVaultsEndAccounts,
        args: FlashSwapUnevenVaultsEndArguments,
    },
    EmergencySwap {
        accounts: EmergencySwapAccounts,
        args: EmergencySwapArguments,
    },
    WithdrawFromTreasury {
        accounts: WithdrawFromTreasuryAccounts,
        args: WithdrawFromTreasuryArguments,
    },
    PermisionlessWithdrawFromTreasury {
        accounts: PermisionlessWithdrawFromTreasuryAccounts,
        args: PermisionlessWithdrawFromTreasuryArguments,
    },
    WithdrawFromTopup {
        accounts: WithdrawFromTopupAccounts,
        args: WithdrawFromTopupArguments,
    },
    ChangePool {
        accounts: ChangePoolAccounts,
        args: ChangePoolArguments,
    },
    CloseProgramAccount {
        accounts: CloseProgramAccountAccounts,
        args: CloseProgramAccountArguments,
    },
    OrcaSwap {
        accounts: OrcaSwapAccounts,
        args: OrcaSwapArguments,
    },
    SignTerms {
        accounts: SignTermsAccounts,
        args: SignTermsArguments,
    },
    UpdateStrategyAdmin {
        accounts: UpdateStrategyAdminAccounts,
        args: UpdateStrategyAdminArguments,
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
            [208u8, 119u8, 144u8, 145u8, 178u8, 57u8, 105u8, 252u8] => {
                let mut rdr: &[u8] = rest;
                let strategy_type: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_a_collateral_id: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_b_collateral_id: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeStrategyArguments {
                    strategy_type,
                    token_a_collateral_id,
                    token_b_collateral_id,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(17usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let sharesMint = keys.next().unwrap().clone();
                let sharesMintAuthority = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenATokenProgram = keys.next().unwrap().clone();
                let tokenBTokenProgram = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeStrategyAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    pool,
                    tokenAMint,
                    tokenBMint,
                    tokenAVault,
                    tokenBVault,
                    baseVaultAuthority,
                    sharesMint,
                    sharesMintAuthority,
                    tokenInfos,
                    systemProgram,
                    rent,
                    tokenProgram,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    strategy,
                };
                return Ok(Instruction::InitializeStrategy { accounts, args });
            }
            [203u8, 212u8, 8u8, 90u8, 91u8, 118u8, 111u8, 50u8] => {
                let mut rdr: &[u8] = rest;
                let kamino_reward_index: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let collateral_token: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeKaminoRewardArguments {
                    kamino_reward_index,
                    collateral_token,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(10usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeKaminoRewardAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    globalConfig,
                    rewardMint,
                    rewardVault,
                    tokenInfos,
                    baseVaultAuthority,
                    systemProgram,
                    rent,
                    tokenProgram,
                };
                return Ok(Instruction::InitializeKaminoReward { accounts, args });
            }
            [174u8, 174u8, 142u8, 193u8, 47u8, 77u8, 235u8, 65u8] => {
                let mut rdr: &[u8] = rest;
                let kamino_reward_index: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AddKaminoRewardsArguments {
                    kamino_reward_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let rewardAta = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddKaminoRewardsAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    rewardMint,
                    rewardVault,
                    baseVaultAuthority,
                    rewardAta,
                    tokenProgram,
                };
                return Ok(Instruction::AddKaminoRewards { accounts, args });
            }
            [113u8, 216u8, 122u8, 131u8, 225u8, 209u8, 22u8, 55u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeGlobalConfigArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGlobalConfigAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    systemProgram,
                };
                return Ok(Instruction::InitializeGlobalConfig { accounts, args });
            }
            [74u8, 61u8, 216u8, 76u8, 244u8, 91u8, 18u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeCollateralInfoArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let collInfo = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeCollateralInfoAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    collInfo,
                    systemProgram,
                };
                return Ok(Instruction::InitializeCollateralInfo { accounts, args });
            }
            [76u8, 94u8, 131u8, 44u8, 137u8, 61u8, 161u8, 110u8] => {
                let mut rdr: &[u8] = rest;
                let index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let mode: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let value: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateCollateralInfoArguments { index, mode, value };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateCollateralInfoAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    tokenInfos,
                };
                return Ok(Instruction::UpdateCollateralInfo { accounts, args });
            }
            [22u8, 97u8, 4u8, 78u8, 166u8, 188u8, 51u8, 190u8] => {
                let mut rdr: &[u8] = rest;
                let index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let params: CollateralInfoParams =
                    <CollateralInfoParams as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InsertCollateralInfoArguments { index, params };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InsertCollateralInfoAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    tokenInfos,
                };
                return Ok(Instruction::InsertCollateralInfo { accounts, args });
            }
            [3u8, 15u8, 172u8, 114u8, 200u8, 0u8, 131u8, 32u8] => {
                let mut rdr: &[u8] = rest;
                let name: String = <String as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let symbol: String = <String as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let uri: String = <String as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeSharesMetadataArguments { name, symbol, uri };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(9usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let sharesMint = keys.next().unwrap().clone();
                let sharesMetadata = keys.next().unwrap().clone();
                let sharesMintAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeSharesMetadataAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    globalConfig,
                    sharesMint,
                    sharesMetadata,
                    sharesMintAuthority,
                    systemProgram,
                    rent,
                    metadataProgram,
                };
                return Ok(Instruction::InitializeSharesMetadata { accounts, args });
            }
            [155u8, 34u8, 122u8, 165u8, 245u8, 137u8, 147u8, 107u8] => {
                let mut rdr: &[u8] = rest;
                let name: String = <String as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let symbol: String = <String as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let uri: String = <String as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateSharesMetadataArguments { name, symbol, uri };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let sharesMint = keys.next().unwrap().clone();
                let sharesMetadata = keys.next().unwrap().clone();
                let sharesMintAuthority = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSharesMetadataAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    globalConfig,
                    sharesMint,
                    sharesMetadata,
                    sharesMintAuthority,
                    metadataProgram,
                };
                return Ok(Instruction::UpdateSharesMetadata { accounts, args });
            }
            [164u8, 84u8, 130u8, 189u8, 111u8, 58u8, 250u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let key: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let index: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let value: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateGlobalConfigArguments { key, index, value };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    systemProgram,
                };
                return Ok(Instruction::UpdateGlobalConfig { accounts, args });
            }
            [9u8, 241u8, 94u8, 91u8, 173u8, 74u8, 166u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let collateral_id: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateTreasuryFeeVaultArguments { collateral_id };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(8usize);
                let signer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let feeMint = keys.next().unwrap().clone();
                let treasuryFeeVault = keys.next().unwrap().clone();
                let treasuryFeeVaultAuthority = keys.next().unwrap().clone();
                let tokenInfos = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTreasuryFeeVaultAccounts {
                    remaining,
                    signer,
                    globalConfig,
                    feeMint,
                    treasuryFeeVault,
                    treasuryFeeVaultAuthority,
                    tokenInfos,
                    systemProgram,
                    rent,
                    tokenProgram,
                };
                return Ok(Instruction::UpdateTreasuryFeeVault { accounts, args });
            }
            [81u8, 217u8, 177u8, 65u8, 40u8, 227u8, 8u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let mode: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let value: [u8; 128usize] =
                    <[u8; 128usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateStrategyConfigArguments { mode, value };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(5usize);
                let adminAuthority = keys.next().unwrap().clone();
                let newAccount = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStrategyConfigAccounts {
                    remaining,
                    adminAuthority,
                    newAccount,
                    strategy,
                    globalConfig,
                    systemProgram,
                };
                return Ok(Instruction::UpdateStrategyConfig { accounts, args });
            }
            [203u8, 37u8, 37u8, 96u8, 23u8, 85u8, 233u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let reward_index: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let collateral_token: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateRewardMappingArguments {
                    reward_index,
                    collateral_token,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(11usize);
                let payer = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateRewardMappingAccounts {
                    remaining,
                    payer,
                    strategy,
                    globalConfig,
                    pool,
                    rewardMint,
                    rewardVault,
                    baseVaultAuthority,
                    tokenInfos,
                    systemProgram,
                    rent,
                    tokenProgram,
                };
                return Ok(Instruction::UpdateRewardMapping { accounts, args });
            }
            [204u8, 234u8, 204u8, 219u8, 6u8, 91u8, 96u8, 241u8] => {
                let mut rdr: &[u8] = rest;
                let tick_lower_index: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let tick_upper_index: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bump: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = OpenLiquidityPositionArguments {
                    tick_lower_index,
                    tick_upper_index,
                    bump,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(25usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionMint = keys.next().unwrap().clone();
                let positionMetadataAccount = keys.next().unwrap().clone();
                let positionTokenAccount = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let associatedTokenProgram = keys.next().unwrap().clone();
                let poolProgram = keys.next().unwrap().clone();
                let oldTickArrayLowerOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldTickArrayUpperOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldPositionMintOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldPositionTokenAccountOrBaseVaultAuthority = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let tokenAMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let scopePrices = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenInfos = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let consensusAccount = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = OpenLiquidityPositionAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    globalConfig,
                    pool,
                    tickArrayLower,
                    tickArrayUpper,
                    baseVaultAuthority,
                    position,
                    positionMint,
                    positionMetadataAccount,
                    positionTokenAccount,
                    rent,
                    system,
                    tokenProgram,
                    tokenProgram2022,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    associatedTokenProgram,
                    poolProgram,
                    oldTickArrayLowerOrBaseVaultAuthority,
                    oldTickArrayUpperOrBaseVaultAuthority,
                    oldPositionOrBaseVaultAuthority,
                    oldPositionMintOrBaseVaultAuthority,
                    oldPositionTokenAccountOrBaseVaultAuthority,
                    tokenAVault,
                    tokenBVault,
                    tokenAMint,
                    tokenBMint,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    scopePrices,
                    tokenInfos,
                    eventAuthority,
                    consensusAccount,
                };
                return Ok(Instruction::OpenLiquidityPosition { accounts, args });
            }
            [56u8, 247u8, 170u8, 246u8, 89u8, 221u8, 134u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseStrategyArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(24usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let oldPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldPositionMintOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldPositionTokenAccountOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldTickArrayLowerOrBaseVaultAuthority = keys.next().unwrap().clone();
                let oldTickArrayUpperOrBaseVaultAuthority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let userTokenAAta = keys.next().unwrap().clone();
                let userTokenBAta = keys.next().unwrap().clone();
                let tokenAMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let reward0Vault = keys.next().unwrap().clone();
                let reward1Vault = keys.next().unwrap().clone();
                let reward2Vault = keys.next().unwrap().clone();
                let kaminoReward0Vault = keys.next().unwrap().clone();
                let kaminoReward1Vault = keys.next().unwrap().clone();
                let kaminoReward2Vault = keys.next().unwrap().clone();
                let userReward0Ata = keys.next().unwrap().clone();
                let userReward1Ata = keys.next().unwrap().clone();
                let userReward2Ata = keys.next().unwrap().clone();
                let userKaminoReward0Ata = keys.next().unwrap().clone();
                let userKaminoReward1Ata = keys.next().unwrap().clone();
                let userKaminoReward2Ata = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let baseVaultAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let poolProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let system = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseStrategyAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    oldPositionOrBaseVaultAuthority,
                    oldPositionMintOrBaseVaultAuthority,
                    oldPositionTokenAccountOrBaseVaultAuthority,
                    oldTickArrayLowerOrBaseVaultAuthority,
                    oldTickArrayUpperOrBaseVaultAuthority,
                    pool,
                    tokenAVault,
                    tokenBVault,
                    userTokenAAta,
                    userTokenBAta,
                    tokenAMint,
                    tokenBMint,
                    reward0Vault,
                    reward1Vault,
                    reward2Vault,
                    kaminoReward0Vault,
                    kaminoReward1Vault,
                    kaminoReward2Vault,
                    userReward0Ata,
                    userReward1Ata,
                    userReward2Ata,
                    userKaminoReward0Ata,
                    userKaminoReward1Ata,
                    userKaminoReward2Ata,
                    baseVaultAuthority,
                    poolProgram,
                    tokenProgram,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    system,
                    eventAuthority,
                };
                return Ok(Instruction::CloseStrategy { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let token_max_a: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_max_b: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositArguments {
                    token_max_a,
                    token_max_b,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(19usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let userSharesAta = keys.next().unwrap().clone();
                let sharesMint = keys.next().unwrap().clone();
                let sharesMintAuthority = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let instructionSysvarAccount = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    pool,
                    position,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAVault,
                    tokenBVault,
                    baseVaultAuthority,
                    tokenAAta,
                    tokenBAta,
                    tokenAMint,
                    tokenBMint,
                    userSharesAta,
                    sharesMint,
                    sharesMintAuthority,
                    scopePrices,
                    tokenInfos,
                    tokenProgram,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    instructionSysvarAccount,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [13u8, 245u8, 180u8, 103u8, 254u8, 182u8, 121u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let args = InvestArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(19usize);
                let payer = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let tokenAMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let baseVaultAuthority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let position = keys.next().unwrap().clone();
                let raydiumProtocolPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let positionTokenAccount = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let poolProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = InvestAccounts {
                    remaining,
                    payer,
                    strategy,
                    globalConfig,
                    tokenAVault,
                    tokenBVault,
                    tokenAMint,
                    tokenBMint,
                    baseVaultAuthority,
                    pool,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    tokenProgram,
                    tokenProgram2022,
                    position,
                    raydiumProtocolPositionOrBaseVaultAuthority,
                    positionTokenAccount,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    tickArrayLower,
                    tickArrayUpper,
                    scopePrices,
                    tokenInfos,
                    poolProgram,
                    instructionSysvarAccount,
                    eventAuthority,
                };
                return Ok(Instruction::Invest { accounts, args });
            }
            [22u8, 157u8, 173u8, 6u8, 187u8, 25u8, 86u8, 109u8] => {
                let mut rdr: &[u8] = rest;
                let token_max_a: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_max_b: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositAndInvestArguments {
                    token_max_a,
                    token_max_b,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(28usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let raydiumProtocolPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let positionTokenAccount = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let userSharesAta = keys.next().unwrap().clone();
                let sharesMint = keys.next().unwrap().clone();
                let sharesMintAuthority = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let poolProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAndInvestAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    pool,
                    position,
                    raydiumProtocolPositionOrBaseVaultAuthority,
                    positionTokenAccount,
                    tokenAVault,
                    tokenBVault,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    tickArrayLower,
                    tickArrayUpper,
                    baseVaultAuthority,
                    tokenAAta,
                    tokenBAta,
                    tokenAMint,
                    tokenBMint,
                    userSharesAta,
                    sharesMint,
                    sharesMintAuthority,
                    scopePrices,
                    tokenInfos,
                    tokenProgram,
                    tokenProgram2022,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    poolProgram,
                    instructionSysvarAccount,
                    eventAuthority,
                };
                return Ok(Instruction::DepositAndInvest { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let shares_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawArguments { shares_amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(23usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let userSharesAta = keys.next().unwrap().clone();
                let sharesMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let treasuryFeeTokenAVault = keys.next().unwrap().clone();
                let treasuryFeeTokenBVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let positionTokenAccount = keys.next().unwrap().clone();
                let poolProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    pool,
                    position,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAVault,
                    tokenBVault,
                    baseVaultAuthority,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    tokenAAta,
                    tokenBAta,
                    tokenAMint,
                    tokenBMint,
                    userSharesAta,
                    sharesMint,
                    treasuryFeeTokenAVault,
                    treasuryFeeTokenBVault,
                    tokenProgram,
                    tokenProgram2022,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    positionTokenAccount,
                    poolProgram,
                    instructionSysvarAccount,
                    eventAuthority,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            [159u8, 39u8, 110u8, 137u8, 100u8, 234u8, 204u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let action: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ExecutiveWithdrawArguments { action };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(19usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let raydiumProtocolPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let positionTokenAccount = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram2022 = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let poolProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ExecutiveWithdrawAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    globalConfig,
                    pool,
                    position,
                    raydiumProtocolPositionOrBaseVaultAuthority,
                    positionTokenAccount,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAVault,
                    tokenBVault,
                    baseVaultAuthority,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    tokenAMint,
                    tokenBMint,
                    scopePrices,
                    tokenInfos,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    tokenProgram,
                    tokenProgram2022,
                    poolProgram,
                    eventAuthority,
                };
                return Ok(Instruction::ExecutiveWithdraw { accounts, args });
            }
            [113u8, 18u8, 75u8, 8u8, 182u8, 31u8, 105u8, 186u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectFeesAndRewardsArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(28usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let raydiumProtocolPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let positionTokenAccount = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let treasuryFeeTokenAVault = keys.next().unwrap().clone();
                let treasuryFeeTokenBVault = keys.next().unwrap().clone();
                let treasuryFeeVaultAuthority = keys.next().unwrap().clone();
                let reward0Vault = keys.next().unwrap().clone();
                let reward1Vault = keys.next().unwrap().clone();
                let reward2Vault = keys.next().unwrap().clone();
                let poolRewardVault0 = keys.next().unwrap().clone();
                let poolRewardVault1 = keys.next().unwrap().clone();
                let poolRewardVault2 = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let poolProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = CollectFeesAndRewardsAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    baseVaultAuthority,
                    pool,
                    tickArrayLower,
                    tickArrayUpper,
                    position,
                    raydiumProtocolPositionOrBaseVaultAuthority,
                    positionTokenAccount,
                    tokenAVault,
                    poolTokenVaultA,
                    tokenBVault,
                    poolTokenVaultB,
                    treasuryFeeTokenAVault,
                    treasuryFeeTokenBVault,
                    treasuryFeeVaultAuthority,
                    reward0Vault,
                    reward1Vault,
                    reward2Vault,
                    poolRewardVault0,
                    poolRewardVault1,
                    poolRewardVault2,
                    tokenAMint,
                    tokenBMint,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    tokenProgram,
                    tokenProgram2022,
                    poolProgram,
                    instructionSysvarAccount,
                    eventAuthority,
                };
                return Ok(Instruction::CollectFeesAndRewards { accounts, args });
            }
            [92u8, 41u8, 172u8, 30u8, 190u8, 65u8, 174u8, 90u8] => {
                let mut rdr: &[u8] = rest;
                let token_a_in: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_b_in: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reward_index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reward_collateral_id: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_collateral_token_out: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SwapRewardsArguments {
                    token_a_in,
                    token_b_in,
                    reward_index,
                    reward_collateral_id,
                    min_collateral_token_out,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(20usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let treasuryFeeTokenAVault = keys.next().unwrap().clone();
                let treasuryFeeTokenBVault = keys.next().unwrap().clone();
                let treasuryFeeVaultAuthority = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let rewardMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let userTokenAAta = keys.next().unwrap().clone();
                let userTokenBAta = keys.next().unwrap().clone();
                let userRewardTokenAccount = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let rewardTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapRewardsAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    pool,
                    tokenAVault,
                    tokenBVault,
                    rewardVault,
                    baseVaultAuthority,
                    treasuryFeeTokenAVault,
                    treasuryFeeTokenBVault,
                    treasuryFeeVaultAuthority,
                    tokenAMint,
                    tokenBMint,
                    rewardMint,
                    userTokenAAta,
                    userTokenBAta,
                    userRewardTokenAccount,
                    scopePrices,
                    tokenInfos,
                    systemProgram,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    rewardTokenProgram,
                    instructionSysvarAccount,
                };
                return Ok(Instruction::SwapRewards { accounts, args });
            }
            [75u8, 151u8, 187u8, 125u8, 50u8, 4u8, 11u8, 71u8] => {
                let mut rdr: &[u8] = rest;
                let token_a_ata_balance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_b_ata_balance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CheckExpectedVaultsBalancesArguments {
                    token_a_ata_balance,
                    token_b_ata_balance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let user = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CheckExpectedVaultsBalancesAccounts {
                    remaining,
                    user,
                    tokenAAta,
                    tokenBAta,
                };
                return Ok(Instruction::CheckExpectedVaultsBalances { accounts, args });
            }
            [118u8, 134u8, 143u8, 192u8, 188u8, 21u8, 131u8, 17u8] => {
                let mut rdr: &[u8] = rest;
                let token_a_min_post_deposit_balance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_b_min_post_deposit_balance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SingleTokenDepositAndInvestWithMinArguments {
                    token_a_min_post_deposit_balance,
                    token_b_min_post_deposit_balance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(28usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let raydiumProtocolPositionOrBaseVaultAuthority = keys.next().unwrap().clone();
                let positionTokenAccount = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let userSharesAta = keys.next().unwrap().clone();
                let sharesMint = keys.next().unwrap().clone();
                let sharesMintAuthority = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenProgram2022 = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let poolProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SingleTokenDepositAndInvestWithMinAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    pool,
                    position,
                    raydiumProtocolPositionOrBaseVaultAuthority,
                    positionTokenAccount,
                    tokenAVault,
                    tokenBVault,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    tickArrayLower,
                    tickArrayUpper,
                    baseVaultAuthority,
                    tokenAAta,
                    tokenBAta,
                    tokenAMint,
                    tokenBMint,
                    userSharesAta,
                    sharesMint,
                    sharesMintAuthority,
                    scopePrices,
                    tokenInfos,
                    tokenProgram,
                    tokenProgram2022,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    poolProgram,
                    instructionSysvarAccount,
                    eventAuthority,
                };
                return Ok(Instruction::SingleTokenDepositAndInvestWithMin { accounts, args });
            }
            [250u8, 142u8, 102u8, 160u8, 72u8, 12u8, 83u8, 139u8] => {
                let mut rdr: &[u8] = rest;
                let token_a_min_post_deposit_balance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let token_b_min_post_deposit_balance: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SingleTokenDepositWithMinArguments {
                    token_a_min_post_deposit_balance,
                    token_b_min_post_deposit_balance,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(19usize);
                let user = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let tickArrayLower = keys.next().unwrap().clone();
                let tickArrayUpper = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let userSharesAta = keys.next().unwrap().clone();
                let sharesMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let sharesMintAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SingleTokenDepositWithMinAccounts {
                    remaining,
                    user,
                    strategy,
                    globalConfig,
                    pool,
                    position,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAVault,
                    tokenBVault,
                    baseVaultAuthority,
                    tokenAAta,
                    tokenBAta,
                    tokenAMint,
                    tokenBMint,
                    userSharesAta,
                    sharesMint,
                    sharesMintAuthority,
                    scopePrices,
                    tokenInfos,
                    tokenProgram,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    instructionSysvarAccount,
                };
                return Ok(Instruction::SingleTokenDepositWithMin { accounts, args });
            }
            [129u8, 111u8, 174u8, 12u8, 10u8, 60u8, 149u8, 193u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let a_to_b: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = FlashSwapUnevenVaultsStartArguments { amount, a_to_b };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(14usize);
                let swapper = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tickArrayLower = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tickArrayUpper = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenAMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let consensusAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FlashSwapUnevenVaultsStartAccounts {
                    remaining,
                    swapper,
                    strategy,
                    globalConfig,
                    tokenAVault,
                    tokenBVault,
                    tokenAAta,
                    tokenBAta,
                    baseVaultAuthority,
                    pool,
                    position,
                    scopePrices,
                    tokenInfos,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAMint,
                    tokenBMint,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    instructionSysvarAccount,
                    consensusAccount,
                };
                return Ok(Instruction::FlashSwapUnevenVaultsStart { accounts, args });
            }
            [226u8, 2u8, 190u8, 101u8, 202u8, 132u8, 156u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let min_repay_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount_to_leave_to_user: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let a_to_b: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = FlashSwapUnevenVaultsEndArguments {
                    min_repay_amount,
                    amount_to_leave_to_user,
                    a_to_b,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(14usize);
                let swapper = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let tokenAAta = keys.next().unwrap().clone();
                let tokenBAta = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let TokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tickArrayLower = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tickArrayUpper = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenAMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let consensusAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FlashSwapUnevenVaultsEndAccounts {
                    remaining,
                    swapper,
                    strategy,
                    globalConfig,
                    tokenAVault,
                    tokenBVault,
                    tokenAAta,
                    tokenBAta,
                    baseVaultAuthority,
                    pool,
                    position,
                    scopePrices,
                    tokenInfos,
                    TokenProgram,
                    tickArrayLower,
                    tickArrayUpper,
                    tokenAMint,
                    tokenBMint,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    instructionSysvarAccount,
                    consensusAccount,
                };
                return Ok(Instruction::FlashSwapUnevenVaultsEnd { accounts, args });
            }
            [73u8, 226u8, 248u8, 215u8, 5u8, 197u8, 211u8, 229u8] => {
                let mut rdr: &[u8] = rest;
                let a_to_b: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let target_limit_bps: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = EmergencySwapArguments {
                    a_to_b,
                    target_limit_bps,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(19usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let poolTokenVaultA = keys.next().unwrap().clone();
                let poolTokenVaultB = keys.next().unwrap().clone();
                let tickArray0 = keys.next().unwrap().clone();
                let tickArray1 = keys.next().unwrap().clone();
                let tickArray2 = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let poolProgram = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenInfos = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = EmergencySwapAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    globalConfig,
                    tokenAMint,
                    tokenBMint,
                    tokenAVault,
                    tokenBVault,
                    baseVaultAuthority,
                    pool,
                    position,
                    poolTokenVaultA,
                    poolTokenVaultB,
                    tickArray0,
                    tickArray1,
                    tickArray2,
                    oracle,
                    poolProgram,
                    scopePrices,
                    tokenInfos,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                };
                return Ok(Instruction::EmergencySwap { accounts, args });
            }
            [0u8, 164u8, 86u8, 76u8, 56u8, 72u8, 12u8, 170u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawFromTreasuryArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(9usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let treasuryFeeVault = keys.next().unwrap().clone();
                let treasuryFeeVaultAuthority = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawFromTreasuryAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    mint,
                    treasuryFeeVault,
                    treasuryFeeVaultAuthority,
                    tokenAccount,
                    systemProgram,
                    rent,
                    tokenProgram,
                };
                return Ok(Instruction::WithdrawFromTreasury { accounts, args });
            }
            [167u8, 36u8, 32u8, 79u8, 97u8, 170u8, 183u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = PermisionlessWithdrawFromTreasuryArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let signer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let treasuryFeeVault = keys.next().unwrap().clone();
                let treasuryFeeVaultAuthority = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PermisionlessWithdrawFromTreasuryAccounts {
                    remaining,
                    signer,
                    globalConfig,
                    mint,
                    treasuryFeeVault,
                    treasuryFeeVaultAuthority,
                    tokenAccount,
                    tokenProgram,
                };
                return Ok(Instruction::PermisionlessWithdrawFromTreasury { accounts, args });
            }
            [95u8, 227u8, 138u8, 220u8, 240u8, 95u8, 150u8, 113u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawFromTopupArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let adminAuthority = keys.next().unwrap().clone();
                let topupVault = keys.next().unwrap().clone();
                let system = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawFromTopupAccounts {
                    remaining,
                    adminAuthority,
                    topupVault,
                    system,
                };
                return Ok(Instruction::WithdrawFromTopup { accounts, args });
            }
            [141u8, 221u8, 123u8, 235u8, 35u8, 9u8, 145u8, 201u8] => {
                let mut rdr: &[u8] = rest;
                let args = ChangePoolArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(5usize);
                let adminAuthority = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let oldPosition = keys.next().unwrap().clone();
                let baseVaultAuthority = keys.next().unwrap().clone();
                let newPool = keys.next().unwrap().clone();
                let strategyRewardVault0OrBaseVaultAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let strategyRewardVault1OrBaseVaultAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let strategyRewardVault2OrBaseVaultAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = ChangePoolAccounts {
                    remaining,
                    adminAuthority,
                    strategy,
                    oldPosition,
                    baseVaultAuthority,
                    newPool,
                    strategyRewardVault0OrBaseVaultAuthority,
                    strategyRewardVault1OrBaseVaultAuthority,
                    strategyRewardVault2OrBaseVaultAuthority,
                };
                return Ok(Instruction::ChangePool { accounts, args });
            }
            [245u8, 14u8, 192u8, 211u8, 99u8, 42u8, 170u8, 187u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseProgramAccountArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(5usize);
                let adminAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let programData = keys.next().unwrap().clone();
                let closingAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseProgramAccountAccounts {
                    remaining,
                    adminAuthority,
                    program,
                    programData,
                    closingAccount,
                    systemProgram,
                };
                return Ok(Instruction::CloseProgramAccount { accounts, args });
            }
            [33u8, 94u8, 249u8, 97u8, 250u8, 254u8, 198u8, 93u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let other_amount_threshold: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let sqrt_price_limit: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount_specified_is_input: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let a_to_b: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = OrcaSwapArguments {
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit,
                    amount_specified_is_input,
                    a_to_b,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(14usize);
                let funder = keys.next().unwrap().clone();
                let tokenATokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenBTokenProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let memoProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenAuthority = keys.next().unwrap().clone();
                let whirlpool = keys.next().unwrap().clone();
                let tokenOwnerAccountA = keys.next().unwrap().clone();
                let tokenVaultA = keys.next().unwrap().clone();
                let tokenOwnerAccountB = keys.next().unwrap().clone();
                let tokenVaultB = keys.next().unwrap().clone();
                let tokenMintA = keys.next().unwrap().clone();
                let tokenMintB = keys.next().unwrap().clone();
                let tickArray0 = keys.next().unwrap().clone();
                let tickArray1 = keys.next().unwrap().clone();
                let tickArray2 = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let whirlpoolProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OrcaSwapAccounts {
                    remaining,
                    funder,
                    tokenATokenProgram,
                    tokenBTokenProgram,
                    memoProgram,
                    tokenAuthority,
                    whirlpool,
                    tokenOwnerAccountA,
                    tokenVaultA,
                    tokenOwnerAccountB,
                    tokenVaultB,
                    tokenMintA,
                    tokenMintB,
                    tickArray0,
                    tickArray1,
                    tickArray2,
                    oracle,
                    whirlpoolProgram,
                };
                return Ok(Instruction::OrcaSwap { accounts, args });
            }
            [226u8, 42u8, 174u8, 143u8, 144u8, 159u8, 139u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let signature: [u8; 64usize] =
                    <[u8; 64usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SignTermsArguments { signature };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let owner = keys.next().unwrap().clone();
                let ownerSignatureState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SignTermsAccounts {
                    remaining,
                    owner,
                    ownerSignatureState,
                    systemProgram,
                    rent,
                };
                return Ok(Instruction::SignTerms { accounts, args });
            }
            [13u8, 227u8, 164u8, 236u8, 32u8, 39u8, 6u8, 255u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateStrategyAdminArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let pendingAdmin = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStrategyAdminAccounts {
                    remaining,
                    pendingAdmin,
                    strategy,
                };
                return Ok(Instruction::UpdateStrategyAdmin { accounts, args });
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
}
