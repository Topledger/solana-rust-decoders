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
    serde_big_array::big_array! { BigArray ; 64 }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AccountEventHeader {
        #[serde(with = "pubkey_serde_option")]
        pub signer: Option<Pubkey>,
        #[serde(with = "pubkey_serde")]
        pub marginfi_account: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub marginfi_account_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub marginfi_group: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Balance {
        pub active: u8,
        #[serde(with = "pubkey_serde")]
        pub bank_pk: Pubkey,
        pub bank_asset_tag: u8,
        pub pad0: [u8; 6],
        pub asset_shares: WrappedI80F48,
        pub liability_shares: WrappedI80F48,
        pub emissions_outstanding: WrappedI80F48,
        pub last_update: u64,
        pub padding: [u64; 1],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Bank {
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub mint_decimals: u8,
        #[serde(with = "pubkey_serde")]
        pub group: Pubkey,
        pub pad0: [u8; 7],
        pub asset_share_value: WrappedI80F48,
        pub liability_share_value: WrappedI80F48,
        #[serde(with = "pubkey_serde")]
        pub liquidity_vault: Pubkey,
        pub liquidity_vault_bump: u8,
        pub liquidity_vault_authority_bump: u8,
        #[serde(with = "pubkey_serde")]
        pub insurance_vault: Pubkey,
        pub insurance_vault_bump: u8,
        pub insurance_vault_authority_bump: u8,
        pub pad1: [u8; 4],
        pub collected_insurance_fees_outstanding: WrappedI80F48,
        #[serde(with = "pubkey_serde")]
        pub fee_vault: Pubkey,
        pub fee_vault_bump: u8,
        pub fee_vault_authority_bump: u8,
        pub pad2: [u8; 6],
        pub collected_group_fees_outstanding: WrappedI80F48,
        pub total_liability_shares: WrappedI80F48,
        pub total_asset_shares: WrappedI80F48,
        pub last_update: i64,
        pub config: BankConfig,
        pub flags: u64,
        pub emissions_rate: u64,
        pub emissions_remaining: WrappedI80F48,
        #[serde(with = "pubkey_serde")]
        pub emissions_mint: Pubkey,
        pub collected_program_fees_outstanding: WrappedI80F48,
        pub emode: EmodeSettings,
        #[serde(with = "pubkey_serde")]
        pub fees_destination_account: Pubkey,
        pub padding_0: [u8; 8],
        pub padding_1: [[u64; 2]; 30],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct BankConfig {
        pub asset_weight_init: WrappedI80F48,
        pub asset_weight_maint: WrappedI80F48,
        pub liability_weight_init: WrappedI80F48,
        pub liability_weight_maint: WrappedI80F48,
        pub deposit_limit: u64,
        pub interest_rate_config: InterestRateConfig,
        pub operational_state: BankOperationalState,
        pub oracle_setup: OracleSetup,
        pub oracle_keys: [Pubkey; 5],
        pub pad0: [u8; 6],
        pub borrow_limit: u64,
        pub risk_tier: RiskTier,
        pub asset_tag: u8,
        pub pad1: [u8; 6],
        pub total_asset_value_init_limit: u64,
        pub oracle_max_age: u16,
        pub padding0: [u8; 6],
        pub padding1: [u8; 32],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct BankConfigCompact {
        pub asset_weight_init: WrappedI80F48,
        pub asset_weight_maint: WrappedI80F48,
        pub liability_weight_init: WrappedI80F48,
        pub liability_weight_maint: WrappedI80F48,
        pub deposit_limit: u64,
        pub interest_rate_config: InterestRateConfigCompact,
        pub operational_state: BankOperationalState,
        pub borrow_limit: u64,
        pub risk_tier: RiskTier,
        pub asset_tag: u8,
        pub pad0: [u8; 6],
        pub total_asset_value_init_limit: u64,
        pub oracle_max_age: u16,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct BankConfigOpt {
        pub asset_weight_init: Option<WrappedI80F48>,
        pub asset_weight_maint: Option<WrappedI80F48>,
        pub liability_weight_init: Option<WrappedI80F48>,
        pub liability_weight_maint: Option<WrappedI80F48>,
        pub deposit_limit: Option<u64>,
        pub borrow_limit: Option<u64>,
        pub operational_state: Option<BankOperationalState>,
        pub interest_rate_config: Option<InterestRateConfigOpt>,
        pub risk_tier: Option<RiskTier>,
        pub asset_tag: Option<u8>,
        pub total_asset_value_init_limit: Option<u64>,
        pub oracle_max_age: Option<u16>,
        pub permissionless_bad_debt_settlement: Option<bool>,
        pub freeze_settings: Option<bool>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum BankOperationalState {
        Paused,
        Operational,
        ReduceOnly,
    }
    impl Default for BankOperationalState {
        fn default() -> Self {
            Self::Paused
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct EditStakedSettingsEvent {
        #[serde(with = "pubkey_serde")]
        pub group: Pubkey,
        pub settings: StakedSettingsEditConfig,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct EmodeConfig {
        pub entries: [EmodeEntry; 10],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct EmodeEntry {
        pub collateral_bank_emode_tag: u16,
        pub flags: u8,
        pub pad0: [u8; 5],
        pub asset_weight_init: WrappedI80F48,
        pub asset_weight_maint: WrappedI80F48,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct EmodeSettings {
        pub emode_tag: u16,
        pub pad0: [u8; 6],
        pub timestamp: i64,
        pub flags: u64,
        pub emode_config: EmodeConfig,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct FeeState {
        #[serde(with = "pubkey_serde")]
        pub key: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub global_fee_admin: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub global_fee_wallet: Pubkey,
        pub placeholder0: u64,
        pub bank_init_flat_sol_fee: u32,
        pub bump_seed: u8,
        pub padding0: [u8; 4],
        pub padding1: [u8; 15],
        pub program_fee_fixed: WrappedI80F48,
        pub program_fee_rate: WrappedI80F48,
        pub reserved0: [u8; 32],
        #[serde(with = "BigArray")]
        pub reserved1: [u8; 64],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct FeeStateCache {
        #[serde(with = "pubkey_serde")]
        pub global_fee_wallet: Pubkey,
        pub program_fee_fixed: WrappedI80F48,
        pub program_fee_rate: WrappedI80F48,
        pub last_update: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct GroupEventHeader {
        #[serde(with = "pubkey_serde_option")]
        pub signer: Option<Pubkey>,
        #[serde(with = "pubkey_serde")]
        pub marginfi_group: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct HealthCache {
        pub asset_value: WrappedI80F48,
        pub liability_value: WrappedI80F48,
        pub asset_value_maint: WrappedI80F48,
        pub liability_value_maint: WrappedI80F48,
        pub asset_value_equity: WrappedI80F48,
        pub liability_value_equity: WrappedI80F48,
        pub timestamp: i64,
        pub flags: u32,
        pub mrgn_err: u32,
        pub prices: [[u8; 8]; 16],
        pub internal_err: u32,
        pub err_index: u8,
        pub program_version: u8,
        pub pad0: [u8; 2],
        pub internal_liq_err: u32,
        pub internal_bankruptcy_err: u32,
        pub reserved0: [u8; 32],
        pub reserved1: [u8; 16],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct HealthPulseEvent {
        #[serde(with = "pubkey_serde")]
        pub account: Pubkey,
        pub health_cache: HealthCache,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InterestRateConfig {
        pub optimal_utilization_rate: WrappedI80F48,
        pub plateau_interest_rate: WrappedI80F48,
        pub max_interest_rate: WrappedI80F48,
        pub insurance_fee_fixed_apr: WrappedI80F48,
        pub insurance_ir_fee: WrappedI80F48,
        pub protocol_fixed_fee_apr: WrappedI80F48,
        pub protocol_ir_fee: WrappedI80F48,
        pub protocol_origination_fee: WrappedI80F48,
        pub padding0: [u8; 16],
        pub padding1: [[u8; 32]; 3],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InterestRateConfigCompact {
        pub optimal_utilization_rate: WrappedI80F48,
        pub plateau_interest_rate: WrappedI80F48,
        pub max_interest_rate: WrappedI80F48,
        pub insurance_fee_fixed_apr: WrappedI80F48,
        pub insurance_ir_fee: WrappedI80F48,
        pub protocol_fixed_fee_apr: WrappedI80F48,
        pub protocol_ir_fee: WrappedI80F48,
        pub protocol_origination_fee: WrappedI80F48,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InterestRateConfigOpt {
        pub optimal_utilization_rate: Option<WrappedI80F48>,
        pub plateau_interest_rate: Option<WrappedI80F48>,
        pub max_interest_rate: Option<WrappedI80F48>,
        pub insurance_fee_fixed_apr: Option<WrappedI80F48>,
        pub insurance_ir_fee: Option<WrappedI80F48>,
        pub protocol_fixed_fee_apr: Option<WrappedI80F48>,
        pub protocol_ir_fee: Option<WrappedI80F48>,
        pub protocol_origination_fee: Option<WrappedI80F48>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingAccount {
        pub balances: [Balance; 16],
        pub padding: [u64; 8],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingAccountBorrowEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingAccountDepositEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingAccountLiquidateEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub liquidatee_marginfi_account: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub liquidatee_marginfi_account_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub asset_bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub asset_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub liability_bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub liability_mint: Pubkey,
        pub liquidatee_pre_health: f64,
        pub liquidatee_post_health: f64,
        pub pre_balances: LiquidationBalances,
        pub post_balances: LiquidationBalances,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingAccountRepayEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub amount: u64,
        pub close_balance: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingAccountWithdrawEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub amount: u64,
        pub close_balance: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankAccrueInterestEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub delta: u64,
        pub fees_collected: f64,
        pub insurance_collected: f64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankCollectFeesEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub group_fees_collected: f64,
        pub group_fees_outstanding: f64,
        pub insurance_fees_collected: f64,
        pub insurance_fees_outstanding: f64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankConfigureEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub config: BankConfigOpt,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankConfigureFrozenEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub deposit_limit: u64,
        pub borrow_limit: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankConfigureOracleEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        pub oracle_setup: u8,
        #[serde(with = "pubkey_serde")]
        pub oracle: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankCreateEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LendingPoolBankHandleBankruptcyEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub bank: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub bad_debt: f64,
        pub covered_amount: f64,
        pub socialized_amount: f64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LiquidationBalances {
        pub liquidatee_asset_balance: f64,
        pub liquidatee_liability_balance: f64,
        pub liquidator_asset_balance: f64,
        pub liquidator_liability_balance: f64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MarginfiAccount {
        #[serde(with = "pubkey_serde")]
        pub group: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        pub lending_account: LendingAccount,
        pub account_flags: u64,
        #[serde(with = "pubkey_serde")]
        pub emissions_destination_account: Pubkey,
        pub health_cache: HealthCache,
        pub padding0: [u64; 21],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MarginfiAccountCreateEvent {
        pub header: AccountEventHeader,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MarginfiAccountTransferAccountAuthorityEvent {
        pub header: AccountEventHeader,
        #[serde(with = "pubkey_serde")]
        pub old_account_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub new_account_authority: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MarginfiGroup {
        #[serde(with = "pubkey_serde")]
        pub admin: Pubkey,
        pub group_flags: u64,
        pub fee_state_cache: FeeStateCache,
        pub banks: u16,
        pub pad0: [u8; 6],
        #[serde(with = "pubkey_serde")]
        pub emode_admin: Pubkey,
        pub padding_0: [[u64; 2]; 24],
        pub padding_1: [[u64; 2]; 32],
        pub padding_4: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MarginfiGroupConfigureEvent {
        pub header: GroupEventHeader,
        #[serde(with = "pubkey_serde")]
        pub admin: Pubkey,
        pub flags: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MarginfiGroupCreateEvent {
        pub header: GroupEventHeader,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum OracleSetup {
        None,
        PythLegacy,
        SwitchboardV2,
        PythPushOracle,
        SwitchboardPull,
        StakedWithPythPush,
    }
    impl Default for OracleSetup {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum RiskTier {
        Collateral,
        Isolated,
    }
    impl Default for RiskTier {
        fn default() -> Self {
            Self::Collateral
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct StakedSettings {
        #[serde(with = "pubkey_serde")]
        pub key: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub marginfi_group: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub oracle: Pubkey,
        pub asset_weight_init: WrappedI80F48,
        pub asset_weight_maint: WrappedI80F48,
        pub deposit_limit: u64,
        pub total_asset_value_init_limit: u64,
        pub oracle_max_age: u16,
        pub risk_tier: RiskTier,
        pub pad0: [u8; 5],
        pub reserved0: [u8; 8],
        pub reserved1: [u8; 32],
        #[serde(with = "BigArray")]
        pub reserved2: [u8; 64],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakedSettingsConfig {
        #[serde(with = "pubkey_serde")]
        pub oracle: Pubkey,
        pub asset_weight_init: WrappedI80F48,
        pub asset_weight_maint: WrappedI80F48,
        pub deposit_limit: u64,
        pub total_asset_value_init_limit: u64,
        pub oracle_max_age: u16,
        pub risk_tier: RiskTier,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakedSettingsEditConfig {
        #[serde(with = "pubkey_serde_option")]
        pub oracle: Option<Pubkey>,
        pub asset_weight_init: Option<WrappedI80F48>,
        pub asset_weight_maint: Option<WrappedI80F48>,
        pub deposit_limit: Option<u64>,
        pub total_asset_value_init_limit: Option<u64>,
        pub oracle_max_age: Option<u16>,
        pub risk_tier: Option<RiskTier>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct WrappedI80F48 {
        pub value: [u8; 16],
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct ConfigGroupFeeAccounts {
        pub marginfi_group: String,
        pub global_fee_admin: String,
        pub fee_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EditGlobalFeeStateAccounts {
        pub global_fee_admin: String,
        pub fee_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EditStakedSettingsAccounts {
        pub marginfi_group: String,
        pub admin: String,
        pub staked_settings: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitGlobalFeeStateAccounts {
        pub payer: String,
        pub fee_state: String,
        pub rent: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitStakedSettingsAccounts {
        pub marginfi_group: String,
        pub admin: String,
        pub fee_payer: String,
        pub staked_settings: String,
        pub rent: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountBorrowAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub bank: String,
        pub destination_token_account: String,
        pub bank_liquidity_vault_authority: String,
        pub liquidity_vault: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountCloseBalanceAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountDepositAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub bank: String,
        pub signer_token_account: String,
        pub liquidity_vault: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountEndFlashloanAccounts {
        pub marginfi_account: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountLiquidateAccounts {
        pub group: String,
        pub asset_bank: String,
        pub liab_bank: String,
        pub liquidator_marginfi_account: String,
        pub authority: String,
        pub liquidatee_marginfi_account: String,
        pub bank_liquidity_vault_authority: String,
        pub bank_liquidity_vault: String,
        pub bank_insurance_vault: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountPulseHealthAccounts {
        pub marginfi_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountRepayAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub bank: String,
        pub signer_token_account: String,
        pub liquidity_vault: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountSettleEmissionsAccounts {
        pub marginfi_account: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountSortBalancesAccounts {
        pub marginfi_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountStartFlashloanAccounts {
        pub marginfi_account: String,
        pub authority: String,
        pub ixs_sysvar: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountWithdrawAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub bank: String,
        pub destination_token_account: String,
        pub bank_liquidity_vault_authority: String,
        pub liquidity_vault: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountWithdrawEmissionsAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub bank: String,
        pub emissions_mint: String,
        pub emissions_auth: String,
        pub emissions_vault: String,
        pub destination_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingAccountWithdrawEmissionsPermissionlessAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub bank: String,
        pub emissions_mint: String,
        pub emissions_auth: String,
        pub emissions_vault: String,
        pub destination_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolAccrueBankInterestAccounts {
        pub group: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolAddBankAccounts {
        pub marginfi_group: String,
        pub admin: String,
        pub fee_payer: String,
        pub fee_state: String,
        pub global_fee_wallet: String,
        pub bank_mint: String,
        pub bank: String,
        pub liquidity_vault_authority: String,
        pub liquidity_vault: String,
        pub insurance_vault_authority: String,
        pub insurance_vault: String,
        pub fee_vault_authority: String,
        pub fee_vault: String,
        pub rent: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolAddBankPermissionlessAccounts {
        pub marginfi_group: String,
        pub staked_settings: String,
        pub fee_payer: String,
        pub bank_mint: String,
        pub sol_pool: String,
        pub stake_pool: String,
        pub bank: String,
        pub liquidity_vault_authority: String,
        pub liquidity_vault: String,
        pub insurance_vault_authority: String,
        pub insurance_vault: String,
        pub fee_vault_authority: String,
        pub fee_vault: String,
        pub rent: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolAddBankWithSeedAccounts {
        pub marginfi_group: String,
        pub admin: String,
        pub fee_payer: String,
        pub fee_state: String,
        pub global_fee_wallet: String,
        pub bank_mint: String,
        pub bank: String,
        pub liquidity_vault_authority: String,
        pub liquidity_vault: String,
        pub insurance_vault_authority: String,
        pub insurance_vault: String,
        pub fee_vault_authority: String,
        pub fee_vault: String,
        pub rent: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolCollectBankFeesAccounts {
        pub group: String,
        pub bank: String,
        pub liquidity_vault_authority: String,
        pub liquidity_vault: String,
        pub insurance_vault: String,
        pub fee_vault: String,
        pub fee_state: String,
        pub fee_ata: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolConfigureBankAccounts {
        pub group: String,
        pub admin: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolConfigureBankEmodeAccounts {
        pub group: String,
        pub emode_admin: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolConfigureBankOracleAccounts {
        pub group: String,
        pub admin: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolHandleBankruptcyAccounts {
        pub group: String,
        pub signer: String,
        pub bank: String,
        pub marginfi_account: String,
        pub liquidity_vault: String,
        pub insurance_vault: String,
        pub insurance_vault_authority: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolSetupEmissionsAccounts {
        pub group: String,
        pub admin: String,
        pub bank: String,
        pub emissions_mint: String,
        pub emissions_auth: String,
        pub emissions_token_account: String,
        pub emissions_funding_account: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolUpdateEmissionsParametersAccounts {
        pub group: String,
        pub admin: String,
        pub bank: String,
        pub emissions_mint: String,
        pub emissions_token_account: String,
        pub emissions_funding_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolUpdateFeesDestinationAccountAccounts {
        pub group: String,
        pub bank: String,
        pub admin: String,
        pub destination_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolWithdrawFeesAccounts {
        pub group: String,
        pub bank: String,
        pub admin: String,
        pub fee_vault: String,
        pub fee_vault_authority: String,
        pub dst_token_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolWithdrawFeesPermissionlessAccounts {
        pub group: String,
        pub bank: String,
        pub fee_vault: String,
        pub fee_vault_authority: String,
        pub fees_destination_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LendingPoolWithdrawInsuranceAccounts {
        pub group: String,
        pub bank: String,
        pub admin: String,
        pub insurance_vault: String,
        pub insurance_vault_authority: String,
        pub dst_token_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarginfiAccountCloseAccounts {
        pub marginfi_account: String,
        pub authority: String,
        pub fee_payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarginfiAccountInitializeAccounts {
        pub marginfi_group: String,
        pub marginfi_account: String,
        pub authority: String,
        pub fee_payer: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarginfiAccountUpdateEmissionsDestinationAccountAccounts {
        pub marginfi_account: String,
        pub authority: String,
        pub destination_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarginfiGroupConfigureAccounts {
        pub marginfi_group: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarginfiGroupInitializeAccounts {
        pub marginfi_group: String,
        pub admin: String,
        pub fee_state: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PropagateFeeStateAccounts {
        pub fee_state: String,
        pub marginfi_group: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PropagateStakedSettingsAccounts {
        pub marginfi_group: String,
        pub staked_settings: String,
        pub bank: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetAccountFlagAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetNewAccountAuthorityAccounts {
        pub marginfi_account: String,
        pub group: String,
        pub authority: String,
        pub new_authority: String,
        pub fee_payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnsetAccountFlagAccounts {
        pub group: String,
        pub marginfi_account: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigGroupFeeArgs {
        pub enable_program_fee: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EditGlobalFeeStateArgs {
        pub admin: [u8; 32usize],
        pub fee_wallet: [u8; 32usize],
        pub bank_init_flat_sol_fee: u32,
        pub program_fee_fixed: WrappedI80F48,
        pub program_fee_rate: WrappedI80F48,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EditStakedSettingsArgs {
        pub settings: StakedSettingsEditConfig,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitGlobalFeeStateArgs {
        pub admin: [u8; 32usize],
        pub fee_wallet: [u8; 32usize],
        pub bank_init_flat_sol_fee: u32,
        pub program_fee_fixed: WrappedI80F48,
        pub program_fee_rate: WrappedI80F48,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitStakedSettingsArgs {
        pub settings: StakedSettingsConfig,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountBorrowArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountCloseBalanceArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountDepositArgs {
        pub amount: u64,
        pub deposit_up_to_limit: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountEndFlashloanArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountLiquidateArgs {
        pub asset_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountPulseHealthArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountRepayArgs {
        pub amount: u64,
        pub repay_all: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountSettleEmissionsArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountSortBalancesArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountStartFlashloanArgs {
        pub end_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountWithdrawArgs {
        pub amount: u64,
        pub withdraw_all: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountWithdrawEmissionsArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingAccountWithdrawEmissionsPermissionlessArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolAccrueBankInterestArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolAddBankArgs {
        pub bank_config: BankConfigCompact,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolAddBankPermissionlessArgs {
        pub bank_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolAddBankWithSeedArgs {
        pub bank_config: BankConfigCompact,
        pub bank_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolCollectBankFeesArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolConfigureBankArgs {
        pub bank_config_opt: BankConfigOpt,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolConfigureBankEmodeArgs {
        pub emode_tag: u16,
        pub entries: [EmodeEntry; 10usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolConfigureBankOracleArgs {
        pub setup: u8,
        pub oracle: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolHandleBankruptcyArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolSetupEmissionsArgs {
        pub flags: u64,
        pub rate: u64,
        pub total_emissions: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolUpdateEmissionsParametersArgs {
        pub emissions_flags: Option<u64>,
        pub emissions_rate: Option<u64>,
        pub additional_emissions: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolUpdateFeesDestinationAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolWithdrawFeesArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolWithdrawFeesPermissionlessArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LendingPoolWithdrawInsuranceArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarginfiAccountCloseArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarginfiAccountInitializeArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarginfiAccountUpdateEmissionsDestinationAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarginfiGroupConfigureArgs {
        pub new_admin: [u8; 32usize],
        pub new_emode_admin: [u8; 32usize],
        pub is_arena_group: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarginfiGroupInitializeArgs {
        pub is_arena_group: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PropagateFeeStateArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PropagateStakedSettingsArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetAccountFlagArgs {
        pub flag: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetNewAccountAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnsetAccountFlagArgs {
        pub flag: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    ConfigGroupFee {
        accounts: ConfigGroupFeeAccounts,
        args: ConfigGroupFeeArgs,
    },
    EditGlobalFeeState {
        accounts: EditGlobalFeeStateAccounts,
        args: EditGlobalFeeStateArgs,
    },
    EditStakedSettings {
        accounts: EditStakedSettingsAccounts,
        args: EditStakedSettingsArgs,
    },
    InitGlobalFeeState {
        accounts: InitGlobalFeeStateAccounts,
        args: InitGlobalFeeStateArgs,
    },
    InitStakedSettings {
        accounts: InitStakedSettingsAccounts,
        args: InitStakedSettingsArgs,
    },
    LendingAccountBorrow {
        accounts: LendingAccountBorrowAccounts,
        args: LendingAccountBorrowArgs,
    },
    LendingAccountCloseBalance {
        accounts: LendingAccountCloseBalanceAccounts,
        args: LendingAccountCloseBalanceArgs,
    },
    LendingAccountDeposit {
        accounts: LendingAccountDepositAccounts,
        args: LendingAccountDepositArgs,
    },
    LendingAccountEndFlashloan {
        accounts: LendingAccountEndFlashloanAccounts,
        args: LendingAccountEndFlashloanArgs,
    },
    LendingAccountLiquidate {
        accounts: LendingAccountLiquidateAccounts,
        args: LendingAccountLiquidateArgs,
    },
    LendingAccountPulseHealth {
        accounts: LendingAccountPulseHealthAccounts,
        args: LendingAccountPulseHealthArgs,
    },
    LendingAccountRepay {
        accounts: LendingAccountRepayAccounts,
        args: LendingAccountRepayArgs,
    },
    LendingAccountSettleEmissions {
        accounts: LendingAccountSettleEmissionsAccounts,
        args: LendingAccountSettleEmissionsArgs,
    },
    LendingAccountSortBalances {
        accounts: LendingAccountSortBalancesAccounts,
        args: LendingAccountSortBalancesArgs,
    },
    LendingAccountStartFlashloan {
        accounts: LendingAccountStartFlashloanAccounts,
        args: LendingAccountStartFlashloanArgs,
    },
    LendingAccountWithdraw {
        accounts: LendingAccountWithdrawAccounts,
        args: LendingAccountWithdrawArgs,
    },
    LendingAccountWithdrawEmissions {
        accounts: LendingAccountWithdrawEmissionsAccounts,
        args: LendingAccountWithdrawEmissionsArgs,
    },
    LendingAccountWithdrawEmissionsPermissionless {
        accounts: LendingAccountWithdrawEmissionsPermissionlessAccounts,
        args: LendingAccountWithdrawEmissionsPermissionlessArgs,
    },
    LendingPoolAccrueBankInterest {
        accounts: LendingPoolAccrueBankInterestAccounts,
        args: LendingPoolAccrueBankInterestArgs,
    },
    LendingPoolAddBank {
        accounts: LendingPoolAddBankAccounts,
        args: LendingPoolAddBankArgs,
    },
    LendingPoolAddBankPermissionless {
        accounts: LendingPoolAddBankPermissionlessAccounts,
        args: LendingPoolAddBankPermissionlessArgs,
    },
    LendingPoolAddBankWithSeed {
        accounts: LendingPoolAddBankWithSeedAccounts,
        args: LendingPoolAddBankWithSeedArgs,
    },
    LendingPoolCollectBankFees {
        accounts: LendingPoolCollectBankFeesAccounts,
        args: LendingPoolCollectBankFeesArgs,
    },
    LendingPoolConfigureBank {
        accounts: LendingPoolConfigureBankAccounts,
        args: LendingPoolConfigureBankArgs,
    },
    LendingPoolConfigureBankEmode {
        accounts: LendingPoolConfigureBankEmodeAccounts,
        args: LendingPoolConfigureBankEmodeArgs,
    },
    LendingPoolConfigureBankOracle {
        accounts: LendingPoolConfigureBankOracleAccounts,
        args: LendingPoolConfigureBankOracleArgs,
    },
    LendingPoolHandleBankruptcy {
        accounts: LendingPoolHandleBankruptcyAccounts,
        args: LendingPoolHandleBankruptcyArgs,
    },
    LendingPoolSetupEmissions {
        accounts: LendingPoolSetupEmissionsAccounts,
        args: LendingPoolSetupEmissionsArgs,
    },
    LendingPoolUpdateEmissionsParameters {
        accounts: LendingPoolUpdateEmissionsParametersAccounts,
        args: LendingPoolUpdateEmissionsParametersArgs,
    },
    LendingPoolUpdateFeesDestinationAccount {
        accounts: LendingPoolUpdateFeesDestinationAccountAccounts,
        args: LendingPoolUpdateFeesDestinationAccountArgs,
    },
    LendingPoolWithdrawFees {
        accounts: LendingPoolWithdrawFeesAccounts,
        args: LendingPoolWithdrawFeesArgs,
    },
    LendingPoolWithdrawFeesPermissionless {
        accounts: LendingPoolWithdrawFeesPermissionlessAccounts,
        args: LendingPoolWithdrawFeesPermissionlessArgs,
    },
    LendingPoolWithdrawInsurance {
        accounts: LendingPoolWithdrawInsuranceAccounts,
        args: LendingPoolWithdrawInsuranceArgs,
    },
    MarginfiAccountClose {
        accounts: MarginfiAccountCloseAccounts,
        args: MarginfiAccountCloseArgs,
    },
    MarginfiAccountInitialize {
        accounts: MarginfiAccountInitializeAccounts,
        args: MarginfiAccountInitializeArgs,
    },
    MarginfiAccountUpdateEmissionsDestinationAccount {
        accounts: MarginfiAccountUpdateEmissionsDestinationAccountAccounts,
        args: MarginfiAccountUpdateEmissionsDestinationAccountArgs,
    },
    MarginfiGroupConfigure {
        accounts: MarginfiGroupConfigureAccounts,
        args: MarginfiGroupConfigureArgs,
    },
    MarginfiGroupInitialize {
        accounts: MarginfiGroupInitializeAccounts,
        args: MarginfiGroupInitializeArgs,
    },
    PropagateFeeState {
        accounts: PropagateFeeStateAccounts,
        args: PropagateFeeStateArgs,
    },
    PropagateStakedSettings {
        accounts: PropagateStakedSettingsAccounts,
        args: PropagateStakedSettingsArgs,
    },
    SetAccountFlag {
        accounts: SetAccountFlagAccounts,
        args: SetAccountFlagArgs,
    },
    SetNewAccountAuthority {
        accounts: SetNewAccountAuthorityAccounts,
        args: SetNewAccountAuthorityArgs,
    },
    UnsetAccountFlag {
        accounts: UnsetAccountFlagAccounts,
        args: UnsetAccountFlagArgs,
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
            [231u8, 205u8, 66u8, 242u8, 220u8, 87u8, 145u8, 38u8] => {
                let mut rdr: &[u8] = rest;
                let args = ConfigGroupFeeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let global_fee_admin = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfigGroupFeeAccounts {
                    marginfi_group,
                    global_fee_admin,
                    fee_state,
                    remaining,
                };
                return Ok(Instruction::ConfigGroupFee { accounts, args });
            }
            [52u8, 62u8, 35u8, 129u8, 93u8, 69u8, 165u8, 202u8] => {
                let mut rdr: &[u8] = rest;
                let args = EditGlobalFeeStateArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let global_fee_admin = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EditGlobalFeeStateAccounts {
                    global_fee_admin,
                    fee_state,
                    remaining,
                };
                return Ok(Instruction::EditGlobalFeeState { accounts, args });
            }
            [11u8, 108u8, 215u8, 87u8, 240u8, 9u8, 66u8, 241u8] => {
                let mut rdr: &[u8] = rest;
                let args = EditStakedSettingsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let staked_settings = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EditStakedSettingsAccounts {
                    marginfi_group,
                    admin,
                    staked_settings,
                    remaining,
                };
                return Ok(Instruction::EditStakedSettings { accounts, args });
            }
            [82u8, 48u8, 247u8, 59u8, 220u8, 109u8, 231u8, 44u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitGlobalFeeStateArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitGlobalFeeStateAccounts {
                    payer,
                    fee_state,
                    rent,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::InitGlobalFeeState { accounts, args });
            }
            [52u8, 35u8, 149u8, 44u8, 69u8, 86u8, 69u8, 80u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitStakedSettingsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let staked_settings = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitStakedSettingsAccounts {
                    marginfi_group,
                    admin,
                    fee_payer,
                    staked_settings,
                    rent,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::InitStakedSettings { accounts, args });
            }
            [4u8, 126u8, 116u8, 53u8, 48u8, 5u8, 212u8, 31u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountBorrowArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let bank_liquidity_vault_authority = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountBorrowAccounts {
                    group,
                    marginfi_account,
                    authority,
                    bank,
                    destination_token_account,
                    bank_liquidity_vault_authority,
                    liquidity_vault,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountBorrow { accounts, args });
            }
            [245u8, 54u8, 41u8, 4u8, 243u8, 202u8, 31u8, 17u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountCloseBalanceArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountCloseBalanceAccounts {
                    group,
                    marginfi_account,
                    authority,
                    bank,
                    remaining,
                };
                return Ok(Instruction::LendingAccountCloseBalance { accounts, args });
            }
            [171u8, 94u8, 235u8, 103u8, 82u8, 64u8, 212u8, 140u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountDepositArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let signer_token_account = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountDepositAccounts {
                    group,
                    marginfi_account,
                    authority,
                    bank,
                    signer_token_account,
                    liquidity_vault,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountDeposit { accounts, args });
            }
            [105u8, 124u8, 201u8, 106u8, 153u8, 2u8, 8u8, 156u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountEndFlashloanArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountEndFlashloanAccounts {
                    marginfi_account,
                    authority,
                    remaining,
                };
                return Ok(Instruction::LendingAccountEndFlashloan { accounts, args });
            }
            [214u8, 169u8, 151u8, 213u8, 251u8, 167u8, 86u8, 219u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountLiquidateArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let asset_bank = keys.next().unwrap().clone();
                let liab_bank = keys.next().unwrap().clone();
                let liquidator_marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let liquidatee_marginfi_account = keys.next().unwrap().clone();
                let bank_liquidity_vault_authority = keys.next().unwrap().clone();
                let bank_liquidity_vault = keys.next().unwrap().clone();
                let bank_insurance_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountLiquidateAccounts {
                    group,
                    asset_bank,
                    liab_bank,
                    liquidator_marginfi_account,
                    authority,
                    liquidatee_marginfi_account,
                    bank_liquidity_vault_authority,
                    bank_liquidity_vault,
                    bank_insurance_vault,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountLiquidate { accounts, args });
            }
            [186u8, 52u8, 117u8, 97u8, 34u8, 74u8, 39u8, 253u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountPulseHealthArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountPulseHealthAccounts {
                    marginfi_account,
                    remaining,
                };
                return Ok(Instruction::LendingAccountPulseHealth { accounts, args });
            }
            [79u8, 209u8, 172u8, 177u8, 222u8, 51u8, 173u8, 151u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountRepayArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let signer_token_account = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountRepayAccounts {
                    group,
                    marginfi_account,
                    authority,
                    bank,
                    signer_token_account,
                    liquidity_vault,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountRepay { accounts, args });
            }
            [161u8, 58u8, 136u8, 174u8, 242u8, 223u8, 156u8, 176u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountSettleEmissionsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountSettleEmissionsAccounts {
                    marginfi_account,
                    bank,
                    remaining,
                };
                return Ok(Instruction::LendingAccountSettleEmissions { accounts, args });
            }
            [187u8, 194u8, 110u8, 84u8, 82u8, 170u8, 204u8, 9u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountSortBalancesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountSortBalancesAccounts {
                    marginfi_account,
                    remaining,
                };
                return Ok(Instruction::LendingAccountSortBalances { accounts, args });
            }
            [14u8, 131u8, 33u8, 220u8, 81u8, 186u8, 180u8, 107u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountStartFlashloanArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let ixs_sysvar = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountStartFlashloanAccounts {
                    marginfi_account,
                    authority,
                    ixs_sysvar,
                    remaining,
                };
                return Ok(Instruction::LendingAccountStartFlashloan { accounts, args });
            }
            [36u8, 72u8, 74u8, 19u8, 210u8, 210u8, 192u8, 192u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountWithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let bank_liquidity_vault_authority = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountWithdrawAccounts {
                    group,
                    marginfi_account,
                    authority,
                    bank,
                    destination_token_account,
                    bank_liquidity_vault_authority,
                    liquidity_vault,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountWithdraw { accounts, args });
            }
            [234u8, 22u8, 84u8, 214u8, 118u8, 176u8, 140u8, 170u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingAccountWithdrawEmissionsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let emissions_mint = keys.next().unwrap().clone();
                let emissions_auth = keys.next().unwrap().clone();
                let emissions_vault = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountWithdrawEmissionsAccounts {
                    group,
                    marginfi_account,
                    authority,
                    bank,
                    emissions_mint,
                    emissions_auth,
                    emissions_vault,
                    destination_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountWithdrawEmissions { accounts, args });
            }
            [4u8, 174u8, 124u8, 203u8, 44u8, 49u8, 145u8, 150u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    LendingAccountWithdrawEmissionsPermissionlessArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let emissions_mint = keys.next().unwrap().clone();
                let emissions_auth = keys.next().unwrap().clone();
                let emissions_vault = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingAccountWithdrawEmissionsPermissionlessAccounts {
                    group,
                    marginfi_account,
                    bank,
                    emissions_mint,
                    emissions_auth,
                    emissions_vault,
                    destination_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingAccountWithdrawEmissionsPermissionless {
                    accounts,
                    args,
                });
            }
            [108u8, 201u8, 30u8, 87u8, 47u8, 65u8, 97u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolAccrueBankInterestArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolAccrueBankInterestAccounts {
                    group,
                    bank,
                    remaining,
                };
                return Ok(Instruction::LendingPoolAccrueBankInterest { accounts, args });
            }
            [215u8, 68u8, 72u8, 78u8, 208u8, 218u8, 103u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolAddBankArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let global_fee_wallet = keys.next().unwrap().clone();
                let bank_mint = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let liquidity_vault_authority = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let insurance_vault_authority = keys.next().unwrap().clone();
                let insurance_vault = keys.next().unwrap().clone();
                let fee_vault_authority = keys.next().unwrap().clone();
                let fee_vault = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolAddBankAccounts {
                    marginfi_group,
                    admin,
                    fee_payer,
                    fee_state,
                    global_fee_wallet,
                    bank_mint,
                    bank,
                    liquidity_vault_authority,
                    liquidity_vault,
                    insurance_vault_authority,
                    insurance_vault,
                    fee_vault_authority,
                    fee_vault,
                    rent,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolAddBank { accounts, args });
            }
            [127u8, 187u8, 121u8, 34u8, 187u8, 167u8, 238u8, 102u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolAddBankPermissionlessArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let staked_settings = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let bank_mint = keys.next().unwrap().clone();
                let sol_pool = keys.next().unwrap().clone();
                let stake_pool = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let liquidity_vault_authority = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let insurance_vault_authority = keys.next().unwrap().clone();
                let insurance_vault = keys.next().unwrap().clone();
                let fee_vault_authority = keys.next().unwrap().clone();
                let fee_vault = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolAddBankPermissionlessAccounts {
                    marginfi_group,
                    staked_settings,
                    fee_payer,
                    bank_mint,
                    sol_pool,
                    stake_pool,
                    bank,
                    liquidity_vault_authority,
                    liquidity_vault,
                    insurance_vault_authority,
                    insurance_vault,
                    fee_vault_authority,
                    fee_vault,
                    rent,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolAddBankPermissionless { accounts, args });
            }
            [76u8, 211u8, 213u8, 171u8, 117u8, 78u8, 158u8, 76u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolAddBankWithSeedArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let global_fee_wallet = keys.next().unwrap().clone();
                let bank_mint = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let liquidity_vault_authority = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let insurance_vault_authority = keys.next().unwrap().clone();
                let insurance_vault = keys.next().unwrap().clone();
                let fee_vault_authority = keys.next().unwrap().clone();
                let fee_vault = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolAddBankWithSeedAccounts {
                    marginfi_group,
                    admin,
                    fee_payer,
                    fee_state,
                    global_fee_wallet,
                    bank_mint,
                    bank,
                    liquidity_vault_authority,
                    liquidity_vault,
                    insurance_vault_authority,
                    insurance_vault,
                    fee_vault_authority,
                    fee_vault,
                    rent,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolAddBankWithSeed { accounts, args });
            }
            [201u8, 5u8, 215u8, 116u8, 230u8, 92u8, 75u8, 150u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolCollectBankFeesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let liquidity_vault_authority = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let insurance_vault = keys.next().unwrap().clone();
                let fee_vault = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let fee_ata = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolCollectBankFeesAccounts {
                    group,
                    bank,
                    liquidity_vault_authority,
                    liquidity_vault,
                    insurance_vault,
                    fee_vault,
                    fee_state,
                    fee_ata,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolCollectBankFees { accounts, args });
            }
            [121u8, 173u8, 156u8, 40u8, 93u8, 148u8, 56u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolConfigureBankArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolConfigureBankAccounts {
                    group,
                    admin,
                    bank,
                    remaining,
                };
                return Ok(Instruction::LendingPoolConfigureBank { accounts, args });
            }
            [17u8, 175u8, 91u8, 57u8, 239u8, 86u8, 49u8, 71u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolConfigureBankEmodeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let emode_admin = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolConfigureBankEmodeAccounts {
                    group,
                    emode_admin,
                    bank,
                    remaining,
                };
                return Ok(Instruction::LendingPoolConfigureBankEmode { accounts, args });
            }
            [209u8, 82u8, 255u8, 171u8, 124u8, 21u8, 71u8, 81u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolConfigureBankOracleArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolConfigureBankOracleAccounts {
                    group,
                    admin,
                    bank,
                    remaining,
                };
                return Ok(Instruction::LendingPoolConfigureBankOracle { accounts, args });
            }
            [162u8, 11u8, 56u8, 139u8, 90u8, 128u8, 70u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolHandleBankruptcyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let liquidity_vault = keys.next().unwrap().clone();
                let insurance_vault = keys.next().unwrap().clone();
                let insurance_vault_authority = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolHandleBankruptcyAccounts {
                    group,
                    signer,
                    bank,
                    marginfi_account,
                    liquidity_vault,
                    insurance_vault,
                    insurance_vault_authority,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolHandleBankruptcy { accounts, args });
            }
            [206u8, 97u8, 120u8, 172u8, 113u8, 204u8, 169u8, 70u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolSetupEmissionsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let emissions_mint = keys.next().unwrap().clone();
                let emissions_auth = keys.next().unwrap().clone();
                let emissions_token_account = keys.next().unwrap().clone();
                let emissions_funding_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolSetupEmissionsAccounts {
                    group,
                    admin,
                    bank,
                    emissions_mint,
                    emissions_auth,
                    emissions_token_account,
                    emissions_funding_account,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolSetupEmissions { accounts, args });
            }
            [55u8, 213u8, 224u8, 168u8, 153u8, 53u8, 197u8, 40u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolUpdateEmissionsParametersArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let emissions_mint = keys.next().unwrap().clone();
                let emissions_token_account = keys.next().unwrap().clone();
                let emissions_funding_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolUpdateEmissionsParametersAccounts {
                    group,
                    admin,
                    bank,
                    emissions_mint,
                    emissions_token_account,
                    emissions_funding_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolUpdateEmissionsParameters { accounts, args });
            }
            [102u8, 4u8, 121u8, 243u8, 237u8, 110u8, 95u8, 13u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolUpdateFeesDestinationAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolUpdateFeesDestinationAccountAccounts {
                    group,
                    bank,
                    admin,
                    destination_account,
                    remaining,
                };
                return Ok(Instruction::LendingPoolUpdateFeesDestinationAccount { accounts, args });
            }
            [92u8, 140u8, 215u8, 254u8, 170u8, 0u8, 83u8, 174u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolWithdrawFeesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let fee_vault = keys.next().unwrap().clone();
                let fee_vault_authority = keys.next().unwrap().clone();
                let dst_token_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolWithdrawFeesAccounts {
                    group,
                    bank,
                    admin,
                    fee_vault,
                    fee_vault_authority,
                    dst_token_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolWithdrawFees { accounts, args });
            }
            [57u8, 245u8, 1u8, 208u8, 130u8, 18u8, 145u8, 113u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolWithdrawFeesPermissionlessArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let fee_vault = keys.next().unwrap().clone();
                let fee_vault_authority = keys.next().unwrap().clone();
                let fees_destination_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolWithdrawFeesPermissionlessAccounts {
                    group,
                    bank,
                    fee_vault,
                    fee_vault_authority,
                    fees_destination_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolWithdrawFeesPermissionless { accounts, args });
            }
            [108u8, 60u8, 60u8, 246u8, 104u8, 79u8, 159u8, 243u8] => {
                let mut rdr: &[u8] = rest;
                let args = LendingPoolWithdrawInsuranceArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let insurance_vault = keys.next().unwrap().clone();
                let insurance_vault_authority = keys.next().unwrap().clone();
                let dst_token_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LendingPoolWithdrawInsuranceAccounts {
                    group,
                    bank,
                    admin,
                    insurance_vault,
                    insurance_vault_authority,
                    dst_token_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::LendingPoolWithdrawInsurance { accounts, args });
            }
            [186u8, 221u8, 93u8, 34u8, 50u8, 97u8, 194u8, 241u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarginfiAccountCloseArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarginfiAccountCloseAccounts {
                    marginfi_account,
                    authority,
                    fee_payer,
                    remaining,
                };
                return Ok(Instruction::MarginfiAccountClose { accounts, args });
            }
            [43u8, 78u8, 61u8, 255u8, 148u8, 52u8, 249u8, 154u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarginfiAccountInitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarginfiAccountInitializeAccounts {
                    marginfi_group,
                    marginfi_account,
                    authority,
                    fee_payer,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::MarginfiAccountInitialize { accounts, args });
            }
            [73u8, 185u8, 162u8, 201u8, 111u8, 24u8, 116u8, 185u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    MarginfiAccountUpdateEmissionsDestinationAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarginfiAccountUpdateEmissionsDestinationAccountAccounts {
                    marginfi_account,
                    authority,
                    destination_account,
                    remaining,
                };
                return Ok(
                    Instruction::MarginfiAccountUpdateEmissionsDestinationAccount {
                        accounts,
                        args,
                    },
                );
            }
            [62u8, 199u8, 81u8, 78u8, 33u8, 13u8, 236u8, 61u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarginfiGroupConfigureArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarginfiGroupConfigureAccounts {
                    marginfi_group,
                    admin,
                    remaining,
                };
                return Ok(Instruction::MarginfiGroupConfigure { accounts, args });
            }
            [255u8, 67u8, 67u8, 26u8, 94u8, 31u8, 34u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarginfiGroupInitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let fee_state = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarginfiGroupInitializeAccounts {
                    marginfi_group,
                    admin,
                    fee_state,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::MarginfiGroupInitialize { accounts, args });
            }
            [64u8, 3u8, 166u8, 194u8, 129u8, 21u8, 101u8, 155u8] => {
                let mut rdr: &[u8] = rest;
                let args = PropagateFeeStateArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let fee_state = keys.next().unwrap().clone();
                let marginfi_group = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PropagateFeeStateAccounts {
                    fee_state,
                    marginfi_group,
                    remaining,
                };
                return Ok(Instruction::PropagateFeeState { accounts, args });
            }
            [210u8, 30u8, 152u8, 69u8, 130u8, 99u8, 222u8, 170u8] => {
                let mut rdr: &[u8] = rest;
                let args = PropagateStakedSettingsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_group = keys.next().unwrap().clone();
                let staked_settings = keys.next().unwrap().clone();
                let bank = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PropagateStakedSettingsAccounts {
                    marginfi_group,
                    staked_settings,
                    bank,
                    remaining,
                };
                return Ok(Instruction::PropagateStakedSettings { accounts, args });
            }
            [56u8, 238u8, 18u8, 207u8, 193u8, 82u8, 138u8, 174u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetAccountFlagArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetAccountFlagAccounts {
                    group,
                    marginfi_account,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetAccountFlag { accounts, args });
            }
            [153u8, 162u8, 50u8, 84u8, 182u8, 201u8, 74u8, 179u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetNewAccountAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marginfi_account = keys.next().unwrap().clone();
                let group = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let new_authority = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetNewAccountAuthorityAccounts {
                    marginfi_account,
                    group,
                    authority,
                    new_authority,
                    fee_payer,
                    remaining,
                };
                return Ok(Instruction::SetNewAccountAuthority { accounts, args });
            }
            [56u8, 81u8, 56u8, 85u8, 92u8, 49u8, 255u8, 70u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnsetAccountFlagArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let group = keys.next().unwrap().clone();
                let marginfi_account = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnsetAccountFlagAccounts {
                    group,
                    marginfi_account,
                    admin,
                    remaining,
                };
                return Ok(Instruction::UnsetAccountFlag { accounts, args });
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
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        EditStakedSettingsEvent {
            args: EditStakedSettingsEvent,
        },
        HealthPulseEvent {
            args: HealthPulseEvent,
        },
        LendingAccountBorrowEvent {
            args: LendingAccountBorrowEvent,
        },
        LendingAccountDepositEvent {
            args: LendingAccountDepositEvent,
        },
        LendingAccountLiquidateEvent {
            args: LendingAccountLiquidateEvent,
        },
        LendingAccountRepayEvent {
            args: LendingAccountRepayEvent,
        },
        LendingAccountWithdrawEvent {
            args: LendingAccountWithdrawEvent,
        },
        LendingPoolBankAccrueInterestEvent {
            args: LendingPoolBankAccrueInterestEvent,
        },
        LendingPoolBankCollectFeesEvent {
            args: LendingPoolBankCollectFeesEvent,
        },
        LendingPoolBankConfigureEvent {
            args: LendingPoolBankConfigureEvent,
        },
        LendingPoolBankConfigureFrozenEvent {
            args: LendingPoolBankConfigureFrozenEvent,
        },
        LendingPoolBankConfigureOracleEvent {
            args: LendingPoolBankConfigureOracleEvent,
        },
        LendingPoolBankCreateEvent {
            args: LendingPoolBankCreateEvent,
        },
        LendingPoolBankHandleBankruptcyEvent {
            args: LendingPoolBankHandleBankruptcyEvent,
        },
        MarginfiAccountCreateEvent {
            args: MarginfiAccountCreateEvent,
        },
        MarginfiAccountTransferAccountAuthorityEvent {
            args: MarginfiAccountTransferAccountAuthorityEvent,
        },
        MarginfiGroupConfigureEvent {
            args: MarginfiGroupConfigureEvent,
        },
        MarginfiGroupCreateEvent {
            args: MarginfiGroupCreateEvent,
        },
    }
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
            if data.len() < 16 {
                anyhow::bail!("Event log too short");
            }
            let wrapper: [u8; 8] = data[0..8].try_into().unwrap();
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!("Invalid event wrapper");
            }
            let disc: [u8; 8] = data[8..16].try_into().unwrap();
            let payload = &data[16..];
            match disc {
                [29u8, 58u8, 155u8, 191u8, 75u8, 220u8, 145u8, 206u8] => {
                    let mut rdr = &data[16..];
                    let args = EditStakedSettingsEvent::deserialize(&mut rdr)?;
                    return Ok(Event::EditStakedSettingsEvent { args });
                }
                [183u8, 159u8, 218u8, 110u8, 61u8, 220u8, 65u8, 1u8] => {
                    let mut rdr = &data[16..];
                    let args = HealthPulseEvent::deserialize(&mut rdr)?;
                    return Ok(Event::HealthPulseEvent { args });
                }
                [223u8, 96u8, 81u8, 10u8, 156u8, 99u8, 26u8, 59u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingAccountBorrowEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingAccountBorrowEvent { args });
                }
                [161u8, 54u8, 237u8, 217u8, 105u8, 248u8, 122u8, 151u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingAccountDepositEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingAccountDepositEvent { args });
                }
                [166u8, 160u8, 249u8, 154u8, 183u8, 39u8, 23u8, 242u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingAccountLiquidateEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingAccountLiquidateEvent { args });
                }
                [16u8, 220u8, 55u8, 111u8, 7u8, 80u8, 16u8, 25u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingAccountRepayEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingAccountRepayEvent { args });
                }
                [3u8, 220u8, 148u8, 243u8, 33u8, 249u8, 54u8, 88u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingAccountWithdrawEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingAccountWithdrawEvent { args });
                }
                [104u8, 117u8, 187u8, 156u8, 111u8, 154u8, 106u8, 186u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankAccrueInterestEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankAccrueInterestEvent { args });
                }
                [101u8, 119u8, 97u8, 250u8, 169u8, 175u8, 156u8, 253u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankCollectFeesEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankCollectFeesEvent { args });
                }
                [246u8, 35u8, 233u8, 110u8, 93u8, 152u8, 235u8, 40u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankConfigureEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankConfigureEvent { args });
                }
                [24u8, 10u8, 55u8, 18u8, 49u8, 150u8, 157u8, 179u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankConfigureFrozenEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankConfigureFrozenEvent { args });
                }
                [119u8, 140u8, 110u8, 253u8, 150u8, 64u8, 210u8, 62u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankConfigureOracleEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankConfigureOracleEvent { args });
                }
                [236u8, 220u8, 201u8, 63u8, 239u8, 126u8, 136u8, 249u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankCreateEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankCreateEvent { args });
                }
                [166u8, 77u8, 41u8, 140u8, 36u8, 94u8, 10u8, 57u8] => {
                    let mut rdr = &data[16..];
                    let args = LendingPoolBankHandleBankruptcyEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LendingPoolBankHandleBankruptcyEvent { args });
                }
                [183u8, 5u8, 117u8, 104u8, 122u8, 199u8, 68u8, 51u8] => {
                    let mut rdr = &data[16..];
                    let args = MarginfiAccountCreateEvent::deserialize(&mut rdr)?;
                    return Ok(Event::MarginfiAccountCreateEvent { args });
                }
                [112u8, 61u8, 140u8, 132u8, 251u8, 92u8, 90u8, 202u8] => {
                    let mut rdr = &data[16..];
                    let args = MarginfiAccountTransferAccountAuthorityEvent::deserialize(&mut rdr)?;
                    return Ok(Event::MarginfiAccountTransferAccountAuthorityEvent { args });
                }
                [241u8, 104u8, 172u8, 167u8, 41u8, 195u8, 199u8, 170u8] => {
                    let mut rdr = &data[16..];
                    let args = MarginfiGroupConfigureEvent::deserialize(&mut rdr)?;
                    return Ok(Event::MarginfiGroupConfigureEvent { args });
                }
                [233u8, 125u8, 61u8, 14u8, 98u8, 240u8, 136u8, 253u8] => {
                    let mut rdr = &data[16..];
                    let args = MarginfiGroupCreateEvent::deserialize(&mut rdr)?;
                    return Ok(Event::MarginfiGroupCreateEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
