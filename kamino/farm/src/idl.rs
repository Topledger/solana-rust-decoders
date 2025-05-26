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
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum FarmConfigOption {
        UpdateRewardRps,
        UpdateRewardMinClaimDuration,
        WithdrawAuthority,
        DepositWarmupPeriod,
        WithdrawCooldownPeriod,
        RewardType,
        RpsDecimals,
        LockingMode,
        LockingStartTimestamp,
        LockingDuration,
        LockingEarlyWithdrawalPenaltyBps,
        DepositCapAmount,
        SlashedAmountSpillAddress,
        ScopePricesAccount,
        ScopeOraclePriceId,
        ScopeOracleMaxAge,
        UpdateRewardScheduleCurvePoints,
        UpdatePendingFarmAdmin,
        UpdateStrategyId,
        UpdateDelegatedRpsAdmin,
        UpdateVaultId,
        UpdateExtraDelegatedAuthority,
    }
    impl Default for FarmConfigOption {
        fn default() -> Self {
            Self::UpdateRewardRps
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum GlobalConfigOption {
        SetPendingGlobalAdmin,
        SetTreasuryFeeBps,
    }
    impl Default for GlobalConfigOption {
        fn default() -> Self {
            Self::SetPendingGlobalAdmin
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LockingMode {
        None,
        Continuous,
        WithExpiry,
    }
    impl Default for LockingMode {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardInfo {
        pub token: TokenInfo,
        pub rewards_vault: String,
        pub rewards_available: u64,
        pub reward_schedule_curve: RewardScheduleCurve,
        pub min_claim_duration_seconds: u64,
        pub last_issuance_ts: u64,
        pub rewards_issued_unclaimed: u64,
        pub rewards_issued_cumulative: u64,
        pub reward_per_share_scaled: u128,
        pub placeholder0: u64,
        pub reward_type: u8,
        pub rewards_per_second_decimals: u8,
        pub padding0: Vec<u8>,
        pub padding1: Vec<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardPerTimeUnitPoint {
        pub ts_start: u64,
        pub reward_per_time_unit: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardScheduleCurve {
        pub points: Vec<RewardPerTimeUnitPoint>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RewardType {
        Proportional,
        Constant,
    }
    impl Default for RewardType {
        fn default() -> Self {
            Self::Proportional
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TimeUnit {
        Seconds,
        Slots,
    }
    impl Default for TimeUnit {
        fn default() -> Self {
            Self::Seconds
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TokenInfo {
        pub mint: String,
        pub decimals: u64,
        pub token_program: String,
        pub padding: Vec<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DatedPrice {
        pub price: Price,
        pub last_updated_slot: u64,
        pub unix_timestamp: u64,
        pub reserved: Vec<u64>,
        pub reserved2: Vec<u16>,
        pub index: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Price {
        pub value: u64,
        pub exp: u64,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeGlobalConfigAccounts {
        pub globalAdmin: String,
        pub globalConfig: String,
        pub treasuryVaultsAuthority: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAccounts {
        pub globalAdmin: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeFarmAccounts {
        pub farmAdmin: String,
        pub farmState: String,
        pub globalConfig: String,
        pub farmVault: String,
        pub farmVaultsAuthority: String,
        pub tokenMint: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeFarmDelegatedAccounts {
        pub farmAdmin: String,
        pub farmDelegate: String,
        pub farmState: String,
        pub globalConfig: String,
        pub farmVaultsAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeRewardAccounts {
        pub farmAdmin: String,
        pub farmState: String,
        pub globalConfig: String,
        pub rewardMint: String,
        pub rewardVault: String,
        pub rewardTreasuryVault: String,
        pub farmVaultsAuthority: String,
        pub treasuryVaultsAuthority: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddRewardsAccounts {
        pub payer: String,
        pub farmState: String,
        pub rewardMint: String,
        pub rewardVault: String,
        pub farmVaultsAuthority: String,
        pub payerRewardTokenAta: String,
        pub scopePrices: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFarmConfigAccounts {
        pub signer: String,
        pub farmState: String,
        pub scopePrices: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeUserAccounts {
        pub authority: String,
        pub payer: String,
        pub owner: String,
        pub delegatee: String,
        pub userState: String,
        pub farmState: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferOwnershipAccounts {
        pub owner: String,
        pub userState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RewardUserOnceAccounts {
        pub farmAdmin: String,
        pub farmState: String,
        pub userState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshFarmAccounts {
        pub farmState: String,
        pub scopePrices: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct StakeAccounts {
        pub owner: String,
        pub userState: String,
        pub farmState: String,
        pub farmVault: String,
        pub userAta: String,
        pub tokenMint: String,
        pub scopePrices: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetStakeDelegatedAccounts {
        pub delegateAuthority: String,
        pub userState: String,
        pub farmState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct HarvestRewardAccounts {
        pub owner: String,
        pub userState: String,
        pub farmState: String,
        pub globalConfig: String,
        pub rewardMint: String,
        pub userRewardAta: String,
        pub rewardsVault: String,
        pub rewardsTreasuryVault: String,
        pub farmVaultsAuthority: String,
        pub scopePrices: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnstakeAccounts {
        pub owner: String,
        pub userState: String,
        pub farmState: String,
        pub scopePrices: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshUserStateAccounts {
        pub userState: String,
        pub farmState: String,
        pub scopePrices: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawUnstakedDepositsAccounts {
        pub owner: String,
        pub userState: String,
        pub farmState: String,
        pub userAta: String,
        pub farmVault: String,
        pub farmVaultsAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawTreasuryAccounts {
        pub globalAdmin: String,
        pub globalConfig: String,
        pub rewardMint: String,
        pub rewardTreasuryVault: String,
        pub treasuryVaultAuthority: String,
        pub withdrawDestinationTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositToFarmVaultAccounts {
        pub depositor: String,
        pub farmState: String,
        pub farmVault: String,
        pub depositorAta: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawFromFarmVaultAccounts {
        pub withdrawAuthority: String,
        pub farmState: String,
        pub withdrawerTokenAccount: String,
        pub farmVault: String,
        pub farmVaultsAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawSlashedAmountAccounts {
        pub crank: String,
        pub farmState: String,
        pub slashedAmountSpillAddress: String,
        pub farmVault: String,
        pub farmVaultsAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFarmAdminAccounts {
        pub pendingFarmAdmin: String,
        pub farmState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminAccounts {
        pub pendingGlobalAdmin: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawRewardAccounts {
        pub farmAdmin: String,
        pub farmState: String,
        pub rewardMint: String,
        pub rewardVault: String,
        pub farmVaultsAuthority: String,
        pub adminRewardTokenAta: String,
        pub scopePrices: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSecondDelegatedAuthorityAccounts {
        pub globalAdmin: String,
        pub farmState: String,
        pub globalConfig: String,
        pub newSecondDelegatedAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IdlMissingTypesAccounts {
        pub globalAdmin: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeGlobalConfigArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigArgs {
        pub mode: u8,
        pub value: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeFarmArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeFarmDelegatedArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeRewardArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddRewardsArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFarmConfigArgs {
        pub mode: u16,
        pub data: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeUserArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferOwnershipArgs {
        pub new_owner: String,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RewardUserOnceArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshFarmArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct StakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetStakeDelegatedArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub new_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct HarvestRewardArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnstakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub stake_shares_scaled: u128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshUserStateArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawUnstakedDepositsArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawTreasuryArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositToFarmVaultArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawFromFarmVaultArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawSlashedAmountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFarmAdminArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawRewardArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateSecondDelegatedAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IdlMissingTypesArgs {
        pub global_config_option_kind: GlobalConfigOption,
        pub farm_config_option_kind: FarmConfigOption,
        pub time_unit: TimeUnit,
        pub locking_mode: LockingMode,
        pub reward_type: RewardType,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeGlobalConfig {
        accounts: InitializeGlobalConfigAccounts,
        args: InitializeGlobalConfigArgs,
    },
    UpdateGlobalConfig {
        accounts: UpdateGlobalConfigAccounts,
        args: UpdateGlobalConfigArgs,
    },
    InitializeFarm {
        accounts: InitializeFarmAccounts,
        args: InitializeFarmArgs,
    },
    InitializeFarmDelegated {
        accounts: InitializeFarmDelegatedAccounts,
        args: InitializeFarmDelegatedArgs,
    },
    InitializeReward {
        accounts: InitializeRewardAccounts,
        args: InitializeRewardArgs,
    },
    AddRewards {
        accounts: AddRewardsAccounts,
        args: AddRewardsArgs,
    },
    UpdateFarmConfig {
        accounts: UpdateFarmConfigAccounts,
        args: UpdateFarmConfigArgs,
    },
    InitializeUser {
        accounts: InitializeUserAccounts,
        args: InitializeUserArgs,
    },
    TransferOwnership {
        accounts: TransferOwnershipAccounts,
        args: TransferOwnershipArgs,
    },
    RewardUserOnce {
        accounts: RewardUserOnceAccounts,
        args: RewardUserOnceArgs,
    },
    RefreshFarm {
        accounts: RefreshFarmAccounts,
        args: RefreshFarmArgs,
    },
    Stake {
        accounts: StakeAccounts,
        args: StakeArgs,
    },
    SetStakeDelegated {
        accounts: SetStakeDelegatedAccounts,
        args: SetStakeDelegatedArgs,
    },
    HarvestReward {
        accounts: HarvestRewardAccounts,
        args: HarvestRewardArgs,
    },
    Unstake {
        accounts: UnstakeAccounts,
        args: UnstakeArgs,
    },
    RefreshUserState {
        accounts: RefreshUserStateAccounts,
        args: RefreshUserStateArgs,
    },
    WithdrawUnstakedDeposits {
        accounts: WithdrawUnstakedDepositsAccounts,
        args: WithdrawUnstakedDepositsArgs,
    },
    WithdrawTreasury {
        accounts: WithdrawTreasuryAccounts,
        args: WithdrawTreasuryArgs,
    },
    DepositToFarmVault {
        accounts: DepositToFarmVaultAccounts,
        args: DepositToFarmVaultArgs,
    },
    WithdrawFromFarmVault {
        accounts: WithdrawFromFarmVaultAccounts,
        args: WithdrawFromFarmVaultArgs,
    },
    WithdrawSlashedAmount {
        accounts: WithdrawSlashedAmountAccounts,
        args: WithdrawSlashedAmountArgs,
    },
    UpdateFarmAdmin {
        accounts: UpdateFarmAdminAccounts,
        args: UpdateFarmAdminArgs,
    },
    UpdateGlobalConfigAdmin {
        accounts: UpdateGlobalConfigAdminAccounts,
        args: UpdateGlobalConfigAdminArgs,
    },
    WithdrawReward {
        accounts: WithdrawRewardAccounts,
        args: WithdrawRewardArgs,
    },
    UpdateSecondDelegatedAuthority {
        accounts: UpdateSecondDelegatedAuthorityAccounts,
        args: UpdateSecondDelegatedAuthorityArgs,
    },
    IdlMissingTypes {
        accounts: IdlMissingTypesAccounts,
        args: IdlMissingTypesArgs,
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
            [113u8, 216u8, 122u8, 131u8, 225u8, 209u8, 22u8, 55u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeGlobalConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let treasuryVaultsAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGlobalConfigAccounts {
                    globalAdmin,
                    globalConfig,
                    treasuryVaultsAuthority,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeGlobalConfig { accounts, args });
            }
            [164u8, 84u8, 130u8, 189u8, 111u8, 58u8, 250u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAccounts {
                    globalAdmin,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateGlobalConfig { accounts, args });
            }
            [252u8, 28u8, 185u8, 172u8, 244u8, 74u8, 117u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeFarmArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let farmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeFarmAccounts {
                    farmAdmin,
                    farmState,
                    globalConfig,
                    farmVault,
                    farmVaultsAuthority,
                    tokenMint,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeFarm { accounts, args });
            }
            [250u8, 84u8, 101u8, 25u8, 51u8, 77u8, 204u8, 91u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeFarmDelegatedArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let farmAdmin = keys.next().unwrap().clone();
                let farmDelegate = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeFarmDelegatedAccounts {
                    farmAdmin,
                    farmDelegate,
                    farmState,
                    globalConfig,
                    farmVaultsAuthority,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeFarmDelegated { accounts, args });
            }
            [95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeRewardArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let farmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let rewardTreasuryVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let treasuryVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeRewardAccounts {
                    farmAdmin,
                    farmState,
                    globalConfig,
                    rewardMint,
                    rewardVault,
                    rewardTreasuryVault,
                    farmVaultsAuthority,
                    treasuryVaultsAuthority,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeReward { accounts, args });
            }
            [88u8, 186u8, 25u8, 227u8, 38u8, 137u8, 81u8, 23u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddRewardsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let payerRewardTokenAta = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddRewardsAccounts {
                    payer,
                    farmState,
                    rewardMint,
                    rewardVault,
                    farmVaultsAuthority,
                    payerRewardTokenAta,
                    scopePrices,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AddRewards { accounts, args });
            }
            [214u8, 176u8, 188u8, 244u8, 203u8, 59u8, 230u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateFarmConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFarmConfigAccounts {
                    signer,
                    farmState,
                    scopePrices,
                    remaining,
                };
                return Ok(Instruction::UpdateFarmConfig { accounts, args });
            }
            [111u8, 17u8, 185u8, 250u8, 60u8, 122u8, 38u8, 254u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeUserArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let delegatee = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeUserAccounts {
                    authority,
                    payer,
                    owner,
                    delegatee,
                    userState,
                    farmState,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeUser { accounts, args });
            }
            [65u8, 177u8, 215u8, 73u8, 53u8, 45u8, 99u8, 47u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferOwnershipArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferOwnershipAccounts {
                    owner,
                    userState,
                    remaining,
                };
                return Ok(Instruction::TransferOwnership { accounts, args });
            }
            [219u8, 137u8, 57u8, 22u8, 94u8, 186u8, 96u8, 114u8] => {
                let mut rdr: &[u8] = rest;
                let args = RewardUserOnceArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let farmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RewardUserOnceAccounts {
                    farmAdmin,
                    farmState,
                    userState,
                    remaining,
                };
                return Ok(Instruction::RewardUserOnce { accounts, args });
            }
            [214u8, 131u8, 138u8, 183u8, 144u8, 194u8, 172u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshFarmArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshFarmAccounts {
                    farmState,
                    scopePrices,
                    remaining,
                };
                return Ok(Instruction::RefreshFarm { accounts, args });
            }
            [206u8, 176u8, 202u8, 18u8, 200u8, 209u8, 179u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = StakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let userAta = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = StakeAccounts {
                    owner,
                    userState,
                    farmState,
                    farmVault,
                    userAta,
                    tokenMint,
                    scopePrices,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Stake { accounts, args });
            }
            [73u8, 171u8, 184u8, 75u8, 30u8, 56u8, 198u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetStakeDelegatedArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let delegateAuthority = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetStakeDelegatedAccounts {
                    delegateAuthority,
                    userState,
                    farmState,
                    remaining,
                };
                return Ok(Instruction::SetStakeDelegated { accounts, args });
            }
            [68u8, 200u8, 228u8, 233u8, 184u8, 32u8, 226u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = HarvestRewardArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let userRewardAta = keys.next().unwrap().clone();
                let rewardsVault = keys.next().unwrap().clone();
                let rewardsTreasuryVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = HarvestRewardAccounts {
                    owner,
                    userState,
                    farmState,
                    globalConfig,
                    rewardMint,
                    userRewardAta,
                    rewardsVault,
                    rewardsTreasuryVault,
                    farmVaultsAuthority,
                    scopePrices,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::HarvestReward { accounts, args });
            }
            [90u8, 95u8, 107u8, 42u8, 205u8, 124u8, 50u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnstakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnstakeAccounts {
                    owner,
                    userState,
                    farmState,
                    scopePrices,
                    remaining,
                };
                return Ok(Instruction::Unstake { accounts, args });
            }
            [1u8, 135u8, 12u8, 62u8, 243u8, 140u8, 77u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshUserStateArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshUserStateAccounts {
                    userState,
                    farmState,
                    scopePrices,
                    remaining,
                };
                return Ok(Instruction::RefreshUserState { accounts, args });
            }
            [36u8, 102u8, 187u8, 49u8, 220u8, 36u8, 132u8, 67u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawUnstakedDepositsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let userAta = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawUnstakedDepositsAccounts {
                    owner,
                    userState,
                    farmState,
                    userAta,
                    farmVault,
                    farmVaultsAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawUnstakedDeposits { accounts, args });
            }
            [40u8, 63u8, 122u8, 158u8, 144u8, 216u8, 83u8, 96u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawTreasuryArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardTreasuryVault = keys.next().unwrap().clone();
                let treasuryVaultAuthority = keys.next().unwrap().clone();
                let withdrawDestinationTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawTreasuryAccounts {
                    globalAdmin,
                    globalConfig,
                    rewardMint,
                    rewardTreasuryVault,
                    treasuryVaultAuthority,
                    withdrawDestinationTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawTreasury { accounts, args });
            }
            [131u8, 166u8, 64u8, 94u8, 108u8, 213u8, 114u8, 183u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositToFarmVaultArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let depositor = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let depositorAta = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositToFarmVaultAccounts {
                    depositor,
                    farmState,
                    farmVault,
                    depositorAta,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DepositToFarmVault { accounts, args });
            }
            [22u8, 82u8, 128u8, 250u8, 86u8, 79u8, 124u8, 78u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawFromFarmVaultArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let withdrawAuthority = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let withdrawerTokenAccount = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawFromFarmVaultAccounts {
                    withdrawAuthority,
                    farmState,
                    withdrawerTokenAccount,
                    farmVault,
                    farmVaultsAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawFromFarmVault { accounts, args });
            }
            [202u8, 217u8, 67u8, 74u8, 172u8, 22u8, 140u8, 216u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawSlashedAmountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let crank = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let slashedAmountSpillAddress = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawSlashedAmountAccounts {
                    crank,
                    farmState,
                    slashedAmountSpillAddress,
                    farmVault,
                    farmVaultsAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawSlashedAmount { accounts, args });
            }
            [20u8, 37u8, 136u8, 19u8, 122u8, 239u8, 36u8, 130u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateFarmAdminArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pendingFarmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFarmAdminAccounts {
                    pendingFarmAdmin,
                    farmState,
                    remaining,
                };
                return Ok(Instruction::UpdateFarmAdmin { accounts, args });
            }
            [184u8, 87u8, 23u8, 193u8, 156u8, 238u8, 175u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigAdminArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pendingGlobalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAdminAccounts {
                    pendingGlobalAdmin,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateGlobalConfigAdmin { accounts, args });
            }
            [191u8, 187u8, 176u8, 137u8, 9u8, 25u8, 187u8, 244u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawRewardArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let farmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let adminRewardTokenAta = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawRewardAccounts {
                    farmAdmin,
                    farmState,
                    rewardMint,
                    rewardVault,
                    farmVaultsAuthority,
                    adminRewardTokenAta,
                    scopePrices,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawReward { accounts, args });
            }
            [127u8, 26u8, 6u8, 181u8, 203u8, 248u8, 117u8, 64u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateSecondDelegatedAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let newSecondDelegatedAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSecondDelegatedAuthorityAccounts {
                    globalAdmin,
                    farmState,
                    globalConfig,
                    newSecondDelegatedAuthority,
                    remaining,
                };
                return Ok(Instruction::UpdateSecondDelegatedAuthority { accounts, args });
            }
            [130u8, 80u8, 38u8, 153u8, 80u8, 212u8, 182u8, 253u8] => {
                let mut rdr: &[u8] = rest;
                let args = IdlMissingTypesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IdlMissingTypesAccounts {
                    globalAdmin,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::IdlMissingTypes { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
