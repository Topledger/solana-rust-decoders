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
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum GlobalConfigOption {
        SetPendingGlobalAdmin,
        SetTreasuryFeeBps,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LockingMode {
        None,
        Continuous,
        WithExpiry,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardInfo {
        pub token: TokenInfo,
        #[serde(with = "pubkey_serde")]
        pub rewards_vault: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub rewards_available: u64,
        pub reward_schedule_curve: RewardScheduleCurve,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_claim_duration_seconds: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_issuance_ts: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub rewards_issued_unclaimed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub rewards_issued_cumulative: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_per_share_scaled: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub placeholder0: u64,
        pub reward_type: u8,
        pub rewards_per_second_decimals: u8,
        pub padding0: [u8; 6usize],
        pub padding1: [u64; 20usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardPerTimeUnitPoint {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub ts_start: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_per_time_unit: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardScheduleCurve {
        pub points: [RewardPerTimeUnitPoint; 20usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RewardType {
        Proportional,
        Constant,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TimeUnit {
        Seconds,
        Slots,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TokenInfo {
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub decimals: u64,
        #[serde(with = "pubkey_serde")]
        pub token_program: [u8; 32usize],
        pub padding: [u64; 6usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DatedPrice {
        pub price: Price,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_updated_slot: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unix_timestamp: u64,
        pub reserved: [u64; 2usize],
        pub reserved2: [u16; 3usize],
        pub index: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Price {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub value: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scopePrices: Option<String>,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFarmConfigAccounts {
        pub signer: String,
        pub farmState: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scopePrices: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeUserAccounts {
        pub authority: String,
        pub payer: String,
        pub owner: String,
        pub delegatee: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub userState: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub farmState: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub farmVaultsAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scopePrices: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scopePrices: Option<String>,
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
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeGlobalConfigArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigArguments {
        pub mode: u8,
        pub value: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeFarmArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeFarmDelegatedArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeRewardArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddRewardsArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFarmConfigArguments {
        pub mode: u16,
        pub data: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeUserArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferOwnershipArguments {
        #[serde(with = "pubkey_serde")]
        pub new_owner: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RewardUserOnceArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshFarmArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct StakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetStakeDelegatedArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub new_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct HarvestRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnstakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub stake_shares_scaled: u128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshUserStateArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawUnstakedDepositsArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawTreasuryArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositToFarmVaultArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawFromFarmVaultArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawSlashedAmountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFarmAdminArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateSecondDelegatedAuthorityArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IdlMissingTypesArguments {
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
        args: InitializeGlobalConfigArguments,
    },
    UpdateGlobalConfig {
        accounts: UpdateGlobalConfigAccounts,
        args: UpdateGlobalConfigArguments,
    },
    InitializeFarm {
        accounts: InitializeFarmAccounts,
        args: InitializeFarmArguments,
    },
    InitializeFarmDelegated {
        accounts: InitializeFarmDelegatedAccounts,
        args: InitializeFarmDelegatedArguments,
    },
    InitializeReward {
        accounts: InitializeRewardAccounts,
        args: InitializeRewardArguments,
    },
    AddRewards {
        accounts: AddRewardsAccounts,
        args: AddRewardsArguments,
    },
    UpdateFarmConfig {
        accounts: UpdateFarmConfigAccounts,
        args: UpdateFarmConfigArguments,
    },
    InitializeUser {
        accounts: InitializeUserAccounts,
        args: InitializeUserArguments,
    },
    TransferOwnership {
        accounts: TransferOwnershipAccounts,
        args: TransferOwnershipArguments,
    },
    RewardUserOnce {
        accounts: RewardUserOnceAccounts,
        args: RewardUserOnceArguments,
    },
    RefreshFarm {
        accounts: RefreshFarmAccounts,
        args: RefreshFarmArguments,
    },
    Stake {
        accounts: StakeAccounts,
        args: StakeArguments,
    },
    SetStakeDelegated {
        accounts: SetStakeDelegatedAccounts,
        args: SetStakeDelegatedArguments,
    },
    HarvestReward {
        accounts: HarvestRewardAccounts,
        args: HarvestRewardArguments,
    },
    Unstake {
        accounts: UnstakeAccounts,
        args: UnstakeArguments,
    },
    RefreshUserState {
        accounts: RefreshUserStateAccounts,
        args: RefreshUserStateArguments,
    },
    WithdrawUnstakedDeposits {
        accounts: WithdrawUnstakedDepositsAccounts,
        args: WithdrawUnstakedDepositsArguments,
    },
    WithdrawTreasury {
        accounts: WithdrawTreasuryAccounts,
        args: WithdrawTreasuryArguments,
    },
    DepositToFarmVault {
        accounts: DepositToFarmVaultAccounts,
        args: DepositToFarmVaultArguments,
    },
    WithdrawFromFarmVault {
        accounts: WithdrawFromFarmVaultAccounts,
        args: WithdrawFromFarmVaultArguments,
    },
    WithdrawSlashedAmount {
        accounts: WithdrawSlashedAmountAccounts,
        args: WithdrawSlashedAmountArguments,
    },
    UpdateFarmAdmin {
        accounts: UpdateFarmAdminAccounts,
        args: UpdateFarmAdminArguments,
    },
    UpdateGlobalConfigAdmin {
        accounts: UpdateGlobalConfigAdminAccounts,
        args: UpdateGlobalConfigAdminArguments,
    },
    WithdrawReward {
        accounts: WithdrawRewardAccounts,
        args: WithdrawRewardArguments,
    },
    UpdateSecondDelegatedAuthority {
        accounts: UpdateSecondDelegatedAuthorityAccounts,
        args: UpdateSecondDelegatedAuthorityArguments,
    },
    IdlMissingTypes {
        accounts: IdlMissingTypesAccounts,
        args: IdlMissingTypesArguments,
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
                let args = InitializeGlobalConfigArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let treasuryVaultsAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGlobalConfigAccounts {
                    remaining,
                    globalAdmin,
                    globalConfig,
                    treasuryVaultsAuthority,
                    systemProgram,
                };
                return Ok(Instruction::InitializeGlobalConfig { accounts, args });
            }
            [164u8, 84u8, 130u8, 189u8, 111u8, 58u8, 250u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let mode: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let value: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateGlobalConfigArguments { mode, value };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAccounts {
                    remaining,
                    globalAdmin,
                    globalConfig,
                };
                return Ok(Instruction::UpdateGlobalConfig { accounts, args });
            }
            [252u8, 28u8, 185u8, 172u8, 244u8, 74u8, 117u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeFarmArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(9usize);
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
                    remaining,
                    farmAdmin,
                    farmState,
                    globalConfig,
                    farmVault,
                    farmVaultsAuthority,
                    tokenMint,
                    tokenProgram,
                    systemProgram,
                    rent,
                };
                return Ok(Instruction::InitializeFarm { accounts, args });
            }
            [250u8, 84u8, 101u8, 25u8, 51u8, 77u8, 204u8, 91u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeFarmDelegatedArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let farmAdmin = keys.next().unwrap().clone();
                let farmDelegate = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeFarmDelegatedAccounts {
                    remaining,
                    farmAdmin,
                    farmDelegate,
                    farmState,
                    globalConfig,
                    farmVaultsAuthority,
                    systemProgram,
                    rent,
                };
                return Ok(Instruction::InitializeFarmDelegated { accounts, args });
            }
            [95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeRewardArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(11usize);
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
                    remaining,
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
                };
                return Ok(Instruction::InitializeReward { accounts, args });
            }
            [88u8, 186u8, 25u8, 227u8, 38u8, 137u8, 81u8, 23u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reward_index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AddRewardsArguments {
                    amount,
                    reward_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let payer = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let payerRewardTokenAta = keys.next().unwrap().clone();
                let scopePrices = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddRewardsAccounts {
                    remaining,
                    payer,
                    farmState,
                    rewardMint,
                    rewardVault,
                    farmVaultsAuthority,
                    payerRewardTokenAta,
                    scopePrices,
                    tokenProgram,
                };
                return Ok(Instruction::AddRewards { accounts, args });
            }
            [214u8, 176u8, 188u8, 244u8, 203u8, 59u8, 230u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let mode: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let data: Vec<u8> = rdr.to_vec();
                let args = UpdateFarmConfigArguments { mode, data };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let signer = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = UpdateFarmConfigAccounts {
                    remaining,
                    signer,
                    farmState,
                    scopePrices,
                };
                return Ok(Instruction::UpdateFarmConfig { accounts, args });
            }
            [111u8, 17u8, 185u8, 250u8, 60u8, 122u8, 38u8, 254u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeUserArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(6usize);
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let delegatee = keys.next().unwrap().clone();
                let userState = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let farmState = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeUserAccounts {
                    remaining,
                    authority,
                    payer,
                    owner,
                    delegatee,
                    userState,
                    farmState,
                    systemProgram,
                    rent,
                };
                return Ok(Instruction::InitializeUser { accounts, args });
            }
            [65u8, 177u8, 215u8, 73u8, 53u8, 45u8, 99u8, 47u8] => {
                let mut rdr: &[u8] = rest;
                let new_owner: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = TransferOwnershipArguments { new_owner };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferOwnershipAccounts {
                    remaining,
                    owner,
                    userState,
                };
                return Ok(Instruction::TransferOwnership { accounts, args });
            }
            [219u8, 137u8, 57u8, 22u8, 94u8, 186u8, 96u8, 114u8] => {
                let mut rdr: &[u8] = rest;
                let reward_index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = RewardUserOnceArguments {
                    reward_index,
                    amount,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let farmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RewardUserOnceAccounts {
                    remaining,
                    farmAdmin,
                    farmState,
                    userState,
                };
                return Ok(Instruction::RewardUserOnce { accounts, args });
            }
            [214u8, 131u8, 138u8, 183u8, 144u8, 194u8, 172u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshFarmArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshFarmAccounts {
                    remaining,
                    farmState,
                    scopePrices,
                };
                return Ok(Instruction::RefreshFarm { accounts, args });
            }
            [206u8, 176u8, 202u8, 18u8, 200u8, 209u8, 179u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = StakeArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(8usize);
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
                    remaining,
                    owner,
                    userState,
                    farmState,
                    farmVault,
                    userAta,
                    tokenMint,
                    scopePrices,
                    tokenProgram,
                };
                return Ok(Instruction::Stake { accounts, args });
            }
            [73u8, 171u8, 184u8, 75u8, 30u8, 56u8, 198u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let new_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = SetStakeDelegatedArguments { new_amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let delegateAuthority = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetStakeDelegatedAccounts {
                    remaining,
                    delegateAuthority,
                    userState,
                    farmState,
                };
                return Ok(Instruction::SetStakeDelegated { accounts, args });
            }
            [68u8, 200u8, 228u8, 233u8, 184u8, 32u8, 226u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let reward_index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = HarvestRewardArguments { reward_index };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(9usize);
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let userRewardAta = keys.next().unwrap().clone();
                let rewardsVault = keys.next().unwrap().clone();
                let rewardsTreasuryVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let scopePrices = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = HarvestRewardAccounts {
                    remaining,
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
                };
                return Ok(Instruction::HarvestReward { accounts, args });
            }
            [90u8, 95u8, 107u8, 42u8, 205u8, 124u8, 50u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let stake_shares_scaled: u128 =
                    <u128 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UnstakeArguments {
                    stake_shares_scaled,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnstakeAccounts {
                    remaining,
                    owner,
                    userState,
                    farmState,
                    scopePrices,
                };
                return Ok(Instruction::Unstake { accounts, args });
            }
            [1u8, 135u8, 12u8, 62u8, 243u8, 140u8, 77u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshUserStateArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshUserStateAccounts {
                    remaining,
                    userState,
                    farmState,
                    scopePrices,
                };
                return Ok(Instruction::RefreshUserState { accounts, args });
            }
            [36u8, 102u8, 187u8, 49u8, 220u8, 36u8, 132u8, 67u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawUnstakedDepositsArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let owner = keys.next().unwrap().clone();
                let userState = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let userAta = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawUnstakedDepositsAccounts {
                    remaining,
                    owner,
                    userState,
                    farmState,
                    userAta,
                    farmVault,
                    farmVaultsAuthority,
                    tokenProgram,
                };
                return Ok(Instruction::WithdrawUnstakedDeposits { accounts, args });
            }
            [40u8, 63u8, 122u8, 158u8, 144u8, 216u8, 83u8, 96u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawTreasuryArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardTreasuryVault = keys.next().unwrap().clone();
                let treasuryVaultAuthority = keys.next().unwrap().clone();
                let withdrawDestinationTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawTreasuryAccounts {
                    remaining,
                    globalAdmin,
                    globalConfig,
                    rewardMint,
                    rewardTreasuryVault,
                    treasuryVaultAuthority,
                    withdrawDestinationTokenAccount,
                    tokenProgram,
                };
                return Ok(Instruction::WithdrawTreasury { accounts, args });
            }
            [131u8, 166u8, 64u8, 94u8, 108u8, 213u8, 114u8, 183u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = DepositToFarmVaultArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(5usize);
                let depositor = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let depositorAta = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositToFarmVaultAccounts {
                    remaining,
                    depositor,
                    farmState,
                    farmVault,
                    depositorAta,
                    tokenProgram,
                };
                return Ok(Instruction::DepositToFarmVault { accounts, args });
            }
            [22u8, 82u8, 128u8, 250u8, 86u8, 79u8, 124u8, 78u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawFromFarmVaultArguments { amount };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(6usize);
                let withdrawAuthority = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let withdrawerTokenAccount = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawFromFarmVaultAccounts {
                    remaining,
                    withdrawAuthority,
                    farmState,
                    withdrawerTokenAccount,
                    farmVault,
                    farmVaultsAuthority,
                    tokenProgram,
                };
                return Ok(Instruction::WithdrawFromFarmVault { accounts, args });
            }
            [202u8, 217u8, 67u8, 74u8, 172u8, 22u8, 140u8, 216u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawSlashedAmountArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(6usize);
                let crank = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let slashedAmountSpillAddress = keys.next().unwrap().clone();
                let farmVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawSlashedAmountAccounts {
                    remaining,
                    crank,
                    farmState,
                    slashedAmountSpillAddress,
                    farmVault,
                    farmVaultsAuthority,
                    tokenProgram,
                };
                return Ok(Instruction::WithdrawSlashedAmount { accounts, args });
            }
            [20u8, 37u8, 136u8, 19u8, 122u8, 239u8, 36u8, 130u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateFarmAdminArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let pendingFarmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFarmAdminAccounts {
                    remaining,
                    pendingFarmAdmin,
                    farmState,
                };
                return Ok(Instruction::UpdateFarmAdmin { accounts, args });
            }
            [184u8, 87u8, 23u8, 193u8, 156u8, 238u8, 175u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigAdminArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let pendingGlobalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAdminAccounts {
                    remaining,
                    pendingGlobalAdmin,
                    globalConfig,
                };
                return Ok(Instruction::UpdateGlobalConfigAdmin { accounts, args });
            }
            [191u8, 187u8, 176u8, 137u8, 9u8, 25u8, 187u8, 244u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reward_index: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = WithdrawRewardArguments {
                    amount,
                    reward_index,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let farmAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let farmVaultsAuthority = keys.next().unwrap().clone();
                let adminRewardTokenAta = keys.next().unwrap().clone();
                let scopePrices = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawRewardAccounts {
                    remaining,
                    farmAdmin,
                    farmState,
                    rewardMint,
                    rewardVault,
                    farmVaultsAuthority,
                    adminRewardTokenAta,
                    scopePrices,
                    tokenProgram,
                };
                return Ok(Instruction::WithdrawReward { accounts, args });
            }
            [127u8, 26u8, 6u8, 181u8, 203u8, 248u8, 117u8, 64u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateSecondDelegatedAuthorityArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let globalAdmin = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let newSecondDelegatedAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSecondDelegatedAuthorityAccounts {
                    remaining,
                    globalAdmin,
                    farmState,
                    globalConfig,
                    newSecondDelegatedAuthority,
                };
                return Ok(Instruction::UpdateSecondDelegatedAuthority { accounts, args });
            }
            [130u8, 80u8, 38u8, 153u8, 80u8, 212u8, 182u8, 253u8] => {
                let mut rdr: &[u8] = rest;
                let global_config_option_kind: GlobalConfigOption =
                    <GlobalConfigOption as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let farm_config_option_kind: FarmConfigOption =
                    <FarmConfigOption as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let time_unit: TimeUnit =
                    <TimeUnit as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let locking_mode: LockingMode =
                    <LockingMode as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let reward_type: RewardType =
                    <RewardType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = IdlMissingTypesArguments {
                    global_config_option_kind,
                    farm_config_option_kind,
                    time_unit,
                    locking_mode,
                    reward_type,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IdlMissingTypesAccounts {
                    remaining,
                    globalAdmin,
                    globalConfig,
                };
                return Ok(Instruction::IdlMissingTypes { accounts, args });
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
    pub use typedefs::*;
}
