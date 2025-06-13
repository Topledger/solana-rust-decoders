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
    pub struct SplitStakeAccountInfo {
        #[serde(with = "pubkey_serde")]
        pub account: [u8; 32usize],
        pub index: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct U64ValueChange {
        pub old: u64,
        pub new: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct U32ValueChange {
        pub old: u32,
        pub new: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeValueChange {
        pub old: Fee,
        pub new: Fee,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeCentsValueChange {
        pub old: FeeCents,
        pub new: FeeCents,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PubkeyValueChange {
        #[serde(with = "pubkey_serde")]
        pub old: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub new: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BoolValueChange {
        pub old: bool,
        pub new: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ChangeAuthorityData {
        #[serde(with = "pubkey_serde_option")]
        pub admin: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub validator_manager: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub operational_sol_account: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub treasury_msol_account: Option<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ConfigLpParams {
        pub min_fee: Option<Fee>,
        pub max_fee: Option<Fee>,
        pub liquidity_target: Option<u64>,
        pub treasury_cut: Option<Fee>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ConfigMarinadeParams {
        pub rewards_fee: Option<Fee>,
        pub slots_for_stake_delta: Option<u64>,
        pub min_stake: Option<u64>,
        pub min_deposit: Option<u64>,
        pub min_withdraw: Option<u64>,
        pub staking_sol_cap: Option<u64>,
        pub liquidity_sol_cap: Option<u64>,
        pub withdraw_stake_account_enabled: Option<bool>,
        pub delayed_unstake_fee: Option<FeeCents>,
        pub withdraw_stake_account_fee: Option<FeeCents>,
        pub max_stake_moved_per_epoch: Option<Fee>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitializeData {
        #[serde(with = "pubkey_serde")]
        pub admin_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub validator_manager_authority: [u8; 32usize],
        pub min_stake: u64,
        pub rewards_fee: Fee,
        pub liq_pool: LiqPoolInitializeData,
        pub additional_stake_record_space: u32,
        pub additional_validator_record_space: u32,
        pub slots_for_stake_delta: u64,
        #[serde(with = "pubkey_serde")]
        pub pause_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiqPoolInitializeData {
        pub lp_liquidity_target: u64,
        pub lp_max_fee: Fee,
        pub lp_min_fee: Fee,
        pub lp_treasury_cut: Fee,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Fee {
        pub basis_points: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeCents {
        pub bp_cents: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiqPool {
        #[serde(with = "pubkey_serde")]
        pub lp_mint: [u8; 32usize],
        pub lp_mint_authority_bump_seed: u8,
        pub sol_leg_bump_seed: u8,
        pub msol_leg_authority_bump_seed: u8,
        #[serde(with = "pubkey_serde")]
        pub msol_leg: [u8; 32usize],
        pub lp_liquidity_target: u64,
        pub lp_max_fee: Fee,
        pub lp_min_fee: Fee,
        pub treasury_cut: Fee,
        pub lp_supply: u64,
        pub lent_from_sol_leg: u64,
        pub liquidity_sol_cap: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct List {
        #[serde(with = "pubkey_serde")]
        pub account: [u8; 32usize],
        pub item_size: u32,
        pub count: u32,
        #[serde(with = "pubkey_serde")]
        pub reserved1: [u8; 32usize],
        pub reserved2: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StakeRecord {
        #[serde(with = "pubkey_serde")]
        pub stake_account: [u8; 32usize],
        pub last_update_delegated_lamports: u64,
        pub last_update_epoch: u64,
        pub is_emergency_unstaking: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StakeList {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StakeSystem {
        pub stake_list: List,
        pub delayed_unstake_cooling_down: u64,
        pub stake_deposit_bump_seed: u8,
        pub stake_withdraw_bump_seed: u8,
        pub slots_for_stake_delta: u64,
        pub last_stake_delta_epoch: u64,
        pub min_stake: u64,
        pub extra_stake_delta_runs: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ValidatorRecord {
        #[serde(with = "pubkey_serde")]
        pub validator_account: [u8; 32usize],
        pub active_balance: u64,
        pub score: u32,
        pub last_stake_delta_epoch: u64,
        pub duplication_flag_bump_seed: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ValidatorList {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ValidatorSystem {
        pub validator_list: List,
        #[serde(with = "pubkey_serde")]
        pub manager_authority: [u8; 32usize],
        pub total_validator_score: u32,
        pub total_active_balance: u64,
        pub auto_add_validator_enabled: u8,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub state: String,
        pub reservePda: String,
        pub stakeList: String,
        pub validatorList: String,
        pub msolMint: String,
        pub operationalSolAccount: String,
        pub lpMint: String,
        pub solLegPda: String,
        pub msolLeg: String,
        pub treasuryMsolAccount: String,
        pub clock: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ChangeAuthorityAccounts {
        pub state: String,
        pub adminAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddValidatorAccounts {
        pub state: String,
        pub managerAuthority: String,
        pub validatorList: String,
        pub validatorVote: String,
        pub duplicationFlag: String,
        pub rentPayer: String,
        pub clock: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveValidatorAccounts {
        pub state: String,
        pub managerAuthority: String,
        pub validatorList: String,
        pub duplicationFlag: String,
        pub operationalSolAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetValidatorScoreAccounts {
        pub state: String,
        pub managerAuthority: String,
        pub validatorList: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConfigValidatorSystemAccounts {
        pub state: String,
        pub managerAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub state: String,
        pub msolMint: String,
        pub liqPoolSolLegPda: String,
        pub liqPoolMsolLeg: String,
        pub liqPoolMsolLegAuthority: String,
        pub reservePda: String,
        pub transferFrom: String,
        pub mintTo: String,
        pub msolMintAuthority: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositStakeAccountAccounts {
        pub state: String,
        pub validatorList: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeAuthority: String,
        pub duplicationFlag: String,
        pub rentPayer: String,
        pub msolMint: String,
        pub mintTo: String,
        pub msolMintAuthority: String,
        pub clock: String,
        pub rent: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidUnstakeAccounts {
        pub state: String,
        pub msolMint: String,
        pub liqPoolSolLegPda: String,
        pub liqPoolMsolLeg: String,
        pub treasuryMsolAccount: String,
        pub getMsolFrom: String,
        pub getMsolFromAuthority: String,
        pub transferSolTo: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityAccounts {
        pub state: String,
        pub lpMint: String,
        pub lpMintAuthority: String,
        pub liqPoolMsolLeg: String,
        pub liqPoolSolLegPda: String,
        pub transferFrom: String,
        pub mintTo: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidityAccounts {
        pub state: String,
        pub lpMint: String,
        pub burnFrom: String,
        pub burnFromAuthority: String,
        pub transferSolTo: String,
        pub transferMsolTo: String,
        pub liqPoolSolLegPda: String,
        pub liqPoolMsolLeg: String,
        pub liqPoolMsolLegAuthority: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConfigLpAccounts {
        pub state: String,
        pub adminAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConfigMarinadeAccounts {
        pub state: String,
        pub adminAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OrderUnstakeAccounts {
        pub state: String,
        pub msolMint: String,
        pub burnMsolFrom: String,
        pub burnMsolAuthority: String,
        pub newTicketAccount: String,
        pub clock: String,
        pub rent: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimAccounts {
        pub state: String,
        pub reservePda: String,
        pub ticketAccount: String,
        pub transferSolTo: String,
        pub clock: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct StakeReserveAccounts {
        pub state: String,
        pub validatorList: String,
        pub stakeList: String,
        pub validatorVote: String,
        pub reservePda: String,
        pub stakeAccount: String,
        pub stakeDepositAuthority: String,
        pub rentPayer: String,
        pub clock: String,
        pub epochSchedule: String,
        pub rent: String,
        pub stakeHistory: String,
        pub stakeConfig: String,
        pub systemProgram: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateActiveAccounts {
        pub state: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeWithdrawAuthority: String,
        pub reservePda: String,
        pub msolMint: String,
        pub msolMintAuthority: String,
        pub treasuryMsolAccount: String,
        pub clock: String,
        pub stakeHistory: String,
        pub stakeProgram: String,
        pub tokenProgram: String,
        pub validatorList: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateDeactivatedAccounts {
        pub state: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeWithdrawAuthority: String,
        pub reservePda: String,
        pub msolMint: String,
        pub msolMintAuthority: String,
        pub treasuryMsolAccount: String,
        pub clock: String,
        pub stakeHistory: String,
        pub stakeProgram: String,
        pub tokenProgram: String,
        pub operationalSolAccount: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeactivateStakeAccounts {
        pub state: String,
        pub reservePda: String,
        pub validatorList: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeDepositAuthority: String,
        pub splitStakeAccount: String,
        pub splitStakeRentPayer: String,
        pub clock: String,
        pub rent: String,
        pub epochSchedule: String,
        pub stakeHistory: String,
        pub systemProgram: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EmergencyUnstakeAccounts {
        pub state: String,
        pub validatorManagerAuthority: String,
        pub validatorList: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeDepositAuthority: String,
        pub clock: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PartialUnstakeAccounts {
        pub state: String,
        pub validatorManagerAuthority: String,
        pub validatorList: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeDepositAuthority: String,
        pub reservePda: String,
        pub splitStakeAccount: String,
        pub splitStakeRentPayer: String,
        pub clock: String,
        pub rent: String,
        pub stakeHistory: String,
        pub systemProgram: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MergeStakesAccounts {
        pub state: String,
        pub stakeList: String,
        pub validatorList: String,
        pub destinationStake: String,
        pub sourceStake: String,
        pub stakeDepositAuthority: String,
        pub stakeWithdrawAuthority: String,
        pub operationalSolAccount: String,
        pub clock: String,
        pub stakeHistory: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RedelegateAccounts {
        pub state: String,
        pub validatorList: String,
        pub stakeList: String,
        pub stakeAccount: String,
        pub stakeDepositAuthority: String,
        pub reservePda: String,
        pub splitStakeAccount: String,
        pub splitStakeRentPayer: String,
        pub destValidatorAccount: String,
        pub redelegateStakeAccount: String,
        pub clock: String,
        pub stakeHistory: String,
        pub stakeConfig: String,
        pub systemProgram: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PauseAccounts {
        pub state: String,
        pub pauseAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResumeAccounts {
        pub state: String,
        pub pauseAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawStakeAccountAccounts {
        pub state: String,
        pub msolMint: String,
        pub burnMsolFrom: String,
        pub burnMsolAuthority: String,
        pub treasuryMsolAccount: String,
        pub validatorList: String,
        pub stakeList: String,
        pub stakeWithdrawAuthority: String,
        pub stakeDepositAuthority: String,
        pub stakeAccount: String,
        pub splitStakeAccount: String,
        pub splitStakeRentPayer: String,
        pub clock: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub stakeProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocValidatorListAccounts {
        pub state: String,
        pub adminAuthority: String,
        pub validatorList: String,
        pub rentFunds: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocStakeListAccounts {
        pub state: String,
        pub adminAuthority: String,
        pub stakeList: String,
        pub rentFunds: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeArgs {
        pub data: InitializeData,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ChangeAuthorityArgs {
        pub data: ChangeAuthorityData,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddValidatorArgs {
        pub score: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveValidatorArgs {
        pub index: u32,
        pub validator_vote: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetValidatorScoreArgs {
        pub index: u32,
        pub validator_vote: [u8; 32usize],
        pub score: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigValidatorSystemArgs {
        pub extra_runs: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositStakeAccountArgs {
        pub validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidUnstakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub msol_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidityArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tokens: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigLpArgs {
        pub params: ConfigLpParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigMarinadeArgs {
        pub params: ConfigMarinadeParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OrderUnstakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub msol_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct StakeReserveArgs {
        pub validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateActiveArgs {
        pub stake_index: u32,
        pub validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateDeactivatedArgs {
        pub stake_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeactivateStakeArgs {
        pub stake_index: u32,
        pub validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EmergencyUnstakeArgs {
        pub stake_index: u32,
        pub validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PartialUnstakeArgs {
        pub stake_index: u32,
        pub validator_index: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub desired_unstake_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MergeStakesArgs {
        pub destination_stake_index: u32,
        pub source_stake_index: u32,
        pub validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RedelegateArgs {
        pub stake_index: u32,
        pub source_validator_index: u32,
        pub dest_validator_index: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PauseArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ResumeArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawStakeAccountArgs {
        pub stake_index: u32,
        pub validator_index: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub msol_amount: u64,
        pub beneficiary: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocValidatorListArgs {
        pub capacity: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocStakeListArgs {
        pub capacity: u32,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    ChangeAuthority {
        accounts: ChangeAuthorityAccounts,
        args: ChangeAuthorityArgs,
    },
    AddValidator {
        accounts: AddValidatorAccounts,
        args: AddValidatorArgs,
    },
    RemoveValidator {
        accounts: RemoveValidatorAccounts,
        args: RemoveValidatorArgs,
    },
    SetValidatorScore {
        accounts: SetValidatorScoreAccounts,
        args: SetValidatorScoreArgs,
    },
    ConfigValidatorSystem {
        accounts: ConfigValidatorSystemAccounts,
        args: ConfigValidatorSystemArgs,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArgs,
    },
    DepositStakeAccount {
        accounts: DepositStakeAccountAccounts,
        args: DepositStakeAccountArgs,
    },
    LiquidUnstake {
        accounts: LiquidUnstakeAccounts,
        args: LiquidUnstakeArgs,
    },
    AddLiquidity {
        accounts: AddLiquidityAccounts,
        args: AddLiquidityArgs,
    },
    RemoveLiquidity {
        accounts: RemoveLiquidityAccounts,
        args: RemoveLiquidityArgs,
    },
    ConfigLp {
        accounts: ConfigLpAccounts,
        args: ConfigLpArgs,
    },
    ConfigMarinade {
        accounts: ConfigMarinadeAccounts,
        args: ConfigMarinadeArgs,
    },
    OrderUnstake {
        accounts: OrderUnstakeAccounts,
        args: OrderUnstakeArgs,
    },
    Claim {
        accounts: ClaimAccounts,
        args: ClaimArgs,
    },
    StakeReserve {
        accounts: StakeReserveAccounts,
        args: StakeReserveArgs,
    },
    UpdateActive {
        accounts: UpdateActiveAccounts,
        args: UpdateActiveArgs,
    },
    UpdateDeactivated {
        accounts: UpdateDeactivatedAccounts,
        args: UpdateDeactivatedArgs,
    },
    DeactivateStake {
        accounts: DeactivateStakeAccounts,
        args: DeactivateStakeArgs,
    },
    EmergencyUnstake {
        accounts: EmergencyUnstakeAccounts,
        args: EmergencyUnstakeArgs,
    },
    PartialUnstake {
        accounts: PartialUnstakeAccounts,
        args: PartialUnstakeArgs,
    },
    MergeStakes {
        accounts: MergeStakesAccounts,
        args: MergeStakesArgs,
    },
    Redelegate {
        accounts: RedelegateAccounts,
        args: RedelegateArgs,
    },
    Pause {
        accounts: PauseAccounts,
        args: PauseArgs,
    },
    Resume {
        accounts: ResumeAccounts,
        args: ResumeArgs,
    },
    WithdrawStakeAccount {
        accounts: WithdrawStakeAccountAccounts,
        args: WithdrawStakeAccountArgs,
    },
    ReallocValidatorList {
        accounts: ReallocValidatorListAccounts,
        args: ReallocValidatorListArgs,
    },
    ReallocStakeList {
        accounts: ReallocStakeListAccounts,
        args: ReallocStakeListArgs,
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
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let operationalSolAccount = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let solLegPda = keys.next().unwrap().clone();
                let msolLeg = keys.next().unwrap().clone();
                let treasuryMsolAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    state,
                    reservePda,
                    stakeList,
                    validatorList,
                    msolMint,
                    operationalSolAccount,
                    lpMint,
                    solLegPda,
                    msolLeg,
                    treasuryMsolAccount,
                    clock,
                    rent,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [50u8, 106u8, 66u8, 104u8, 99u8, 118u8, 145u8, 88u8] => {
                let mut rdr: &[u8] = rest;
                let args = ChangeAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let adminAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ChangeAuthorityAccounts {
                    state,
                    adminAuthority,
                    remaining,
                };
                return Ok(Instruction::ChangeAuthority { accounts, args });
            }
            [250u8, 113u8, 53u8, 54u8, 141u8, 117u8, 215u8, 185u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddValidatorArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let managerAuthority = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let validatorVote = keys.next().unwrap().clone();
                let duplicationFlag = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddValidatorAccounts {
                    state,
                    managerAuthority,
                    validatorList,
                    validatorVote,
                    duplicationFlag,
                    rentPayer,
                    clock,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::AddValidator { accounts, args });
            }
            [25u8, 96u8, 211u8, 155u8, 161u8, 14u8, 168u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveValidatorArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let managerAuthority = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let duplicationFlag = keys.next().unwrap().clone();
                let operationalSolAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveValidatorAccounts {
                    state,
                    managerAuthority,
                    validatorList,
                    duplicationFlag,
                    operationalSolAccount,
                    remaining,
                };
                return Ok(Instruction::RemoveValidator { accounts, args });
            }
            [101u8, 41u8, 206u8, 33u8, 216u8, 111u8, 25u8, 78u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetValidatorScoreArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let managerAuthority = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetValidatorScoreAccounts {
                    state,
                    managerAuthority,
                    validatorList,
                    remaining,
                };
                return Ok(Instruction::SetValidatorScore { accounts, args });
            }
            [27u8, 90u8, 97u8, 209u8, 17u8, 115u8, 7u8, 40u8] => {
                let mut rdr: &[u8] = rest;
                let args = ConfigValidatorSystemArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let managerAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfigValidatorSystemAccounts {
                    state,
                    managerAuthority,
                    remaining,
                };
                return Ok(Instruction::ConfigValidatorSystem { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let liqPoolSolLegPda = keys.next().unwrap().clone();
                let liqPoolMsolLeg = keys.next().unwrap().clone();
                let liqPoolMsolLegAuthority = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let transferFrom = keys.next().unwrap().clone();
                let mintTo = keys.next().unwrap().clone();
                let msolMintAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    state,
                    msolMint,
                    liqPoolSolLegPda,
                    liqPoolMsolLeg,
                    liqPoolMsolLegAuthority,
                    reservePda,
                    transferFrom,
                    mintTo,
                    msolMintAuthority,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [110u8, 130u8, 115u8, 41u8, 164u8, 102u8, 2u8, 59u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositStakeAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeAuthority = keys.next().unwrap().clone();
                let duplicationFlag = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let mintTo = keys.next().unwrap().clone();
                let msolMintAuthority = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositStakeAccountAccounts {
                    state,
                    validatorList,
                    stakeList,
                    stakeAccount,
                    stakeAuthority,
                    duplicationFlag,
                    rentPayer,
                    msolMint,
                    mintTo,
                    msolMintAuthority,
                    clock,
                    rent,
                    systemProgram,
                    tokenProgram,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::DepositStakeAccount { accounts, args });
            }
            [30u8, 30u8, 119u8, 240u8, 191u8, 227u8, 12u8, 16u8] => {
                let mut rdr: &[u8] = rest;
                let args = LiquidUnstakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let liqPoolSolLegPda = keys.next().unwrap().clone();
                let liqPoolMsolLeg = keys.next().unwrap().clone();
                let treasuryMsolAccount = keys.next().unwrap().clone();
                let getMsolFrom = keys.next().unwrap().clone();
                let getMsolFromAuthority = keys.next().unwrap().clone();
                let transferSolTo = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidUnstakeAccounts {
                    state,
                    msolMint,
                    liqPoolSolLegPda,
                    liqPoolMsolLeg,
                    treasuryMsolAccount,
                    getMsolFrom,
                    getMsolFromAuthority,
                    transferSolTo,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::LiquidUnstake { accounts, args });
            }
            [181u8, 157u8, 89u8, 67u8, 143u8, 182u8, 52u8, 72u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let lpMintAuthority = keys.next().unwrap().clone();
                let liqPoolMsolLeg = keys.next().unwrap().clone();
                let liqPoolSolLegPda = keys.next().unwrap().clone();
                let transferFrom = keys.next().unwrap().clone();
                let mintTo = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityAccounts {
                    state,
                    lpMint,
                    lpMintAuthority,
                    liqPoolMsolLeg,
                    liqPoolSolLegPda,
                    transferFrom,
                    mintTo,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AddLiquidity { accounts, args });
            }
            [80u8, 85u8, 209u8, 72u8, 24u8, 206u8, 177u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquidityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let burnFrom = keys.next().unwrap().clone();
                let burnFromAuthority = keys.next().unwrap().clone();
                let transferSolTo = keys.next().unwrap().clone();
                let transferMsolTo = keys.next().unwrap().clone();
                let liqPoolSolLegPda = keys.next().unwrap().clone();
                let liqPoolMsolLeg = keys.next().unwrap().clone();
                let liqPoolMsolLegAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidityAccounts {
                    state,
                    lpMint,
                    burnFrom,
                    burnFromAuthority,
                    transferSolTo,
                    transferMsolTo,
                    liqPoolSolLegPda,
                    liqPoolMsolLeg,
                    liqPoolMsolLegAuthority,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidity { accounts, args });
            }
            [10u8, 24u8, 168u8, 119u8, 86u8, 48u8, 225u8, 17u8] => {
                let mut rdr: &[u8] = rest;
                let args = ConfigLpArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let adminAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfigLpAccounts {
                    state,
                    adminAuthority,
                    remaining,
                };
                return Ok(Instruction::ConfigLp { accounts, args });
            }
            [67u8, 3u8, 34u8, 114u8, 190u8, 185u8, 17u8, 62u8] => {
                let mut rdr: &[u8] = rest;
                let args = ConfigMarinadeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let adminAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfigMarinadeAccounts {
                    state,
                    adminAuthority,
                    remaining,
                };
                return Ok(Instruction::ConfigMarinade { accounts, args });
            }
            [97u8, 167u8, 144u8, 107u8, 117u8, 190u8, 128u8, 36u8] => {
                let mut rdr: &[u8] = rest;
                let args = OrderUnstakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let burnMsolFrom = keys.next().unwrap().clone();
                let burnMsolAuthority = keys.next().unwrap().clone();
                let newTicketAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OrderUnstakeAccounts {
                    state,
                    msolMint,
                    burnMsolFrom,
                    burnMsolAuthority,
                    newTicketAccount,
                    clock,
                    rent,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::OrderUnstake { accounts, args });
            }
            [62u8, 198u8, 214u8, 193u8, 213u8, 159u8, 108u8, 210u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let ticketAccount = keys.next().unwrap().clone();
                let transferSolTo = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimAccounts {
                    state,
                    reservePda,
                    ticketAccount,
                    transferSolTo,
                    clock,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::Claim { accounts, args });
            }
            [87u8, 217u8, 23u8, 179u8, 205u8, 25u8, 113u8, 129u8] => {
                let mut rdr: &[u8] = rest;
                let args = StakeReserveArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let validatorVote = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let epochSchedule = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeConfig = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = StakeReserveAccounts {
                    state,
                    validatorList,
                    stakeList,
                    validatorVote,
                    reservePda,
                    stakeAccount,
                    stakeDepositAuthority,
                    rentPayer,
                    clock,
                    epochSchedule,
                    rent,
                    stakeHistory,
                    stakeConfig,
                    systemProgram,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::StakeReserve { accounts, args });
            }
            [4u8, 67u8, 81u8, 64u8, 136u8, 245u8, 93u8, 152u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateActiveArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeWithdrawAuthority = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let msolMintAuthority = keys.next().unwrap().clone();
                let treasuryMsolAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateActiveAccounts {
                    state,
                    stakeList,
                    stakeAccount,
                    stakeWithdrawAuthority,
                    reservePda,
                    msolMint,
                    msolMintAuthority,
                    treasuryMsolAccount,
                    clock,
                    stakeHistory,
                    stakeProgram,
                    tokenProgram,
                    validatorList,
                    remaining,
                };
                return Ok(Instruction::UpdateActive { accounts, args });
            }
            [16u8, 232u8, 131u8, 115u8, 156u8, 100u8, 239u8, 50u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateDeactivatedArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeWithdrawAuthority = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let msolMintAuthority = keys.next().unwrap().clone();
                let treasuryMsolAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let operationalSolAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateDeactivatedAccounts {
                    state,
                    stakeList,
                    stakeAccount,
                    stakeWithdrawAuthority,
                    reservePda,
                    msolMint,
                    msolMintAuthority,
                    treasuryMsolAccount,
                    clock,
                    stakeHistory,
                    stakeProgram,
                    tokenProgram,
                    operationalSolAccount,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::UpdateDeactivated { accounts, args });
            }
            [165u8, 158u8, 229u8, 97u8, 168u8, 220u8, 187u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeactivateStakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let splitStakeAccount = keys.next().unwrap().clone();
                let splitStakeRentPayer = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let epochSchedule = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeactivateStakeAccounts {
                    state,
                    reservePda,
                    validatorList,
                    stakeList,
                    stakeAccount,
                    stakeDepositAuthority,
                    splitStakeAccount,
                    splitStakeRentPayer,
                    clock,
                    rent,
                    epochSchedule,
                    stakeHistory,
                    systemProgram,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::DeactivateStake { accounts, args });
            }
            [123u8, 69u8, 168u8, 195u8, 183u8, 213u8, 199u8, 214u8] => {
                let mut rdr: &[u8] = rest;
                let args = EmergencyUnstakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let validatorManagerAuthority = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EmergencyUnstakeAccounts {
                    state,
                    validatorManagerAuthority,
                    validatorList,
                    stakeList,
                    stakeAccount,
                    stakeDepositAuthority,
                    clock,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::EmergencyUnstake { accounts, args });
            }
            [55u8, 241u8, 205u8, 221u8, 45u8, 114u8, 205u8, 163u8] => {
                let mut rdr: &[u8] = rest;
                let args = PartialUnstakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let validatorManagerAuthority = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let splitStakeAccount = keys.next().unwrap().clone();
                let splitStakeRentPayer = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PartialUnstakeAccounts {
                    state,
                    validatorManagerAuthority,
                    validatorList,
                    stakeList,
                    stakeAccount,
                    stakeDepositAuthority,
                    reservePda,
                    splitStakeAccount,
                    splitStakeRentPayer,
                    clock,
                    rent,
                    stakeHistory,
                    systemProgram,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::PartialUnstake { accounts, args });
            }
            [216u8, 36u8, 141u8, 225u8, 243u8, 78u8, 125u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = MergeStakesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let destinationStake = keys.next().unwrap().clone();
                let sourceStake = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let stakeWithdrawAuthority = keys.next().unwrap().clone();
                let operationalSolAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MergeStakesAccounts {
                    state,
                    stakeList,
                    validatorList,
                    destinationStake,
                    sourceStake,
                    stakeDepositAuthority,
                    stakeWithdrawAuthority,
                    operationalSolAccount,
                    clock,
                    stakeHistory,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::MergeStakes { accounts, args });
            }
            [212u8, 82u8, 51u8, 160u8, 228u8, 80u8, 116u8, 35u8] => {
                let mut rdr: &[u8] = rest;
                let args = RedelegateArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let splitStakeAccount = keys.next().unwrap().clone();
                let splitStakeRentPayer = keys.next().unwrap().clone();
                let destValidatorAccount = keys.next().unwrap().clone();
                let redelegateStakeAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeConfig = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RedelegateAccounts {
                    state,
                    validatorList,
                    stakeList,
                    stakeAccount,
                    stakeDepositAuthority,
                    reservePda,
                    splitStakeAccount,
                    splitStakeRentPayer,
                    destValidatorAccount,
                    redelegateStakeAccount,
                    clock,
                    stakeHistory,
                    stakeConfig,
                    systemProgram,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::Redelegate { accounts, args });
            }
            [211u8, 22u8, 221u8, 251u8, 74u8, 121u8, 193u8, 47u8] => {
                let mut rdr: &[u8] = rest;
                let args = PauseArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let pauseAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PauseAccounts {
                    state,
                    pauseAuthority,
                    remaining,
                };
                return Ok(Instruction::Pause { accounts, args });
            }
            [1u8, 166u8, 51u8, 170u8, 127u8, 32u8, 141u8, 206u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResumeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let pauseAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResumeAccounts {
                    state,
                    pauseAuthority,
                    remaining,
                };
                return Ok(Instruction::Resume { accounts, args });
            }
            [211u8, 85u8, 184u8, 65u8, 183u8, 177u8, 233u8, 217u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawStakeAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let burnMsolFrom = keys.next().unwrap().clone();
                let burnMsolAuthority = keys.next().unwrap().clone();
                let treasuryMsolAccount = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let stakeWithdrawAuthority = keys.next().unwrap().clone();
                let stakeDepositAuthority = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let splitStakeAccount = keys.next().unwrap().clone();
                let splitStakeRentPayer = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStakeAccountAccounts {
                    state,
                    msolMint,
                    burnMsolFrom,
                    burnMsolAuthority,
                    treasuryMsolAccount,
                    validatorList,
                    stakeList,
                    stakeWithdrawAuthority,
                    stakeDepositAuthority,
                    stakeAccount,
                    splitStakeAccount,
                    splitStakeRentPayer,
                    clock,
                    systemProgram,
                    tokenProgram,
                    stakeProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawStakeAccount { accounts, args });
            }
            [215u8, 59u8, 218u8, 133u8, 93u8, 138u8, 60u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocValidatorListArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let adminAuthority = keys.next().unwrap().clone();
                let validatorList = keys.next().unwrap().clone();
                let rentFunds = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocValidatorListAccounts {
                    state,
                    adminAuthority,
                    validatorList,
                    rentFunds,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::ReallocValidatorList { accounts, args });
            }
            [12u8, 36u8, 124u8, 27u8, 128u8, 96u8, 85u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocStakeListArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let state = keys.next().unwrap().clone();
                let adminAuthority = keys.next().unwrap().clone();
                let stakeList = keys.next().unwrap().clone();
                let rentFunds = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocStakeListAccounts {
                    state,
                    adminAuthority,
                    stakeList,
                    rentFunds,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::ReallocStakeList { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
