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
    pub struct GlobalConfig {
        pub bump: u8,
        #[serde(with = "pubkey_serde")]
        pub governance_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub pyth_token_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub pyth_governance_realm: Pubkey,
        pub removed_unlocking_duration: u8,
        pub epoch_duration: u64,
        pub freeze: bool,
        #[serde(with = "pubkey_serde")]
        pub pda_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub governance_program: Pubkey,
        pub pyth_token_list_time: Option<i64>,
        pub agreement_hash: [u8; 32],
        pub mock_clock_time: i64,
        #[serde(with = "pubkey_serde")]
        pub pool_authority: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MaxVoterWeightRecord {
        #[serde(with = "pubkey_serde")]
        pub realm: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub governing_token_mint: Pubkey,
        pub max_voter_weight: u64,
        pub max_voter_weight_expiry: Option<u64>,
        pub reserved: [u8; 8],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Position {
        pub amount: u64,
        pub activation_epoch: u64,
        pub unlocking_start: Option<u64>,
        pub target_with_parameters: TargetWithParameters,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PositionData {
        #[serde(with = "pubkey_serde")]
        pub owner: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SplitRequest {
        pub amount: u64,
        #[serde(with = "pubkey_serde")]
        pub recipient: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakeAccountMetadataV2 {
        pub metadata_bump: u8,
        pub custody_bump: u8,
        pub authority_bump: u8,
        pub voter_bump: u8,
        #[serde(with = "pubkey_serde")]
        pub owner: Pubkey,
        pub lock: VestingSchedule,
        pub next_index: u8,
        pub deprecated: Option<u64>,
        pub signed_agreement_hash: Option<[u8; 32]>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TargetMetadata {
        pub bump: u8,
        pub last_update_at: u64,
        pub prev_epoch_locked: u64,
        pub locked: u64,
        pub delta_locked: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum TargetWithParameters {
        Voting,
        IntegrityPool { publisher: Pubkey },
    }
    impl Default for TargetWithParameters {
        fn default() -> Self {
            Self::Voting
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum VestingSchedule {
        FullyVested,
        PeriodicVesting {
            initial_balance: u64,
            start_date: i64,
            period_duration: u64,
            num_periods: u64,
        },
        PeriodicVestingAfterListing {
            initial_balance: u64,
            period_duration: u64,
            num_periods: u64,
        },
    }
    impl Default for VestingSchedule {
        fn default() -> Self {
            Self::FullyVested
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum VoterWeightAction {
        CastVote,
        CommentProposal,
        CreateGovernance,
        CreateProposal,
        SignOffProposal,
    }
    impl Default for VoterWeightAction {
        fn default() -> Self {
            Self::CastVote
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct VoterWeightRecord {
        #[serde(with = "pubkey_serde")]
        pub realm: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub governing_token_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub governing_token_owner: Pubkey,
        pub voter_weight: u64,
        pub voter_weight_expiry: Option<u64>,
        pub weight_action: Option<VoterWeightAction>,
        #[serde(with = "pubkey_serde_option")]
        pub weight_action_target: Option<Pubkey>,
        pub reserved: [u8; 8],
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct AcceptSplitAccounts {
        pub pda_authority: String,
        pub source_stake_account_positions: String,
        pub source_stake_account_metadata: String,
        pub source_stake_account_split_request: String,
        pub source_stake_account_custody: String,
        pub source_custody_authority: String,
        pub new_stake_account_positions: String,
        pub new_stake_account_metadata: String,
        pub new_stake_account_custody: String,
        pub new_custody_authority: String,
        pub config: String,
        pub pyth_token_mint: String,
        pub rent: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AdvanceClockAccounts {
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePositionAccounts {
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_custody: String,
        pub config: String,
        pub target_account: String,
        pub pool_authority: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreatePositionAccounts {
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_custody: String,
        pub config: String,
        pub target_account: String,
        pub pool_authority: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateStakeAccountAccounts {
        pub payer: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_custody: String,
        pub custody_authority: String,
        pub config: String,
        pub pyth_token_mint: String,
        pub rent: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTargetAccounts {
        pub payer: String,
        pub governance_authority: String,
        pub config: String,
        pub target_account: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateVoterRecordAccounts {
        pub payer: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub voter_record: String,
        pub config: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ExportPositionTypeAccounts {
        pub payer: String,
        pub config_account: String,
        pub rent: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitConfigAccounts {
        pub payer: String,
        pub config_account: String,
        pub rent: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct JoinDaoLlcAccounts {
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MergeTargetPositionsAccounts {
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub config: String,
        pub pool_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RecoverAccountAccounts {
        pub governance_authority: String,
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub voter_record: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RequestSplitAccounts {
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_split_request: String,
        pub config: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SlashAccountAccounts {
        pub pool_authority: String,
        pub publisher: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_custody: String,
        pub config: String,
        pub governance_target_account: String,
        pub destination: String,
        pub custody_authority: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAgreementHashAccounts {
        pub governance_authority: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGovernanceAuthorityAccounts {
        pub governance_authority: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateMaxVoterWeightAccounts {
        pub payer: String,
        pub max_voter_record: String,
        pub config: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePdaAuthorityAccounts {
        pub pda_authority: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePoolAuthorityAccounts {
        pub governance_authority: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateTokenListTimeAccounts {
        pub governance_authority: String,
        pub config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateVoterWeightAccounts {
        pub owner: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_custody: String,
        pub voter_record: String,
        pub config: String,
        pub governance_target: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawStakeAccounts {
        pub owner: String,
        pub destination: String,
        pub stake_account_positions: String,
        pub stake_account_metadata: String,
        pub stake_account_custody: String,
        pub custody_authority: String,
        pub config: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde_option;
    use crate::pubkey_serializer::pubkey_serde_u32;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AcceptSplitArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(with = "pubkey_serde_u32")]
        pub recipient: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AdvanceClockArgs {
        pub seconds: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionArgs {
        pub index: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub target_with_parameters: TargetWithParameters,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePositionArgs {
        pub target_with_parameters: TargetWithParameters,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateStakeAccountArgs {
        #[serde(with = "pubkey_serde_u32")]
        pub owner: [u8; 32usize],
        pub lock: VestingSchedule,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTargetArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateVoterRecordArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExportPositionTypeArgs {
        pub position: Position,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitConfigArgs {
        pub global_config: GlobalConfig,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct JoinDaoLlcArgs {
        pub agreement_hash: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MergeTargetPositionsArgs {
        pub target_with_parameters: TargetWithParameters,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RecoverAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RequestSplitArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(with = "pubkey_serde_u32")]
        pub recipient: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SlashAccountArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slash_ratio: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateAgreementHashArgs {
        pub agreement_hash: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGovernanceAuthorityArgs {
        #[serde(with = "pubkey_serde_u32")]
        pub new_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateMaxVoterWeightArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePdaAuthorityArgs {
        #[serde(with = "pubkey_serde_u32")]
        pub new_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePoolAuthorityArgs {
        #[serde(with = "pubkey_serde_u32")]
        pub pool_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateTokenListTimeArgs {
        pub token_list_time: Option<i64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateVoterWeightArgs {
        pub action: VoterWeightAction,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawStakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    AcceptSplit {
        accounts: AcceptSplitAccounts,
        args: AcceptSplitArgs,
    },
    AdvanceClock {
        accounts: AdvanceClockAccounts,
        args: AdvanceClockArgs,
    },
    ClosePosition {
        accounts: ClosePositionAccounts,
        args: ClosePositionArgs,
    },
    CreatePosition {
        accounts: CreatePositionAccounts,
        args: CreatePositionArgs,
    },
    CreateStakeAccount {
        accounts: CreateStakeAccountAccounts,
        args: CreateStakeAccountArgs,
    },
    CreateTarget {
        accounts: CreateTargetAccounts,
        args: CreateTargetArgs,
    },
    CreateVoterRecord {
        accounts: CreateVoterRecordAccounts,
        args: CreateVoterRecordArgs,
    },
    ExportPositionType {
        accounts: ExportPositionTypeAccounts,
        args: ExportPositionTypeArgs,
    },
    InitConfig {
        accounts: InitConfigAccounts,
        args: InitConfigArgs,
    },
    JoinDaoLlc {
        accounts: JoinDaoLlcAccounts,
        args: JoinDaoLlcArgs,
    },
    MergeTargetPositions {
        accounts: MergeTargetPositionsAccounts,
        args: MergeTargetPositionsArgs,
    },
    RecoverAccount {
        accounts: RecoverAccountAccounts,
        args: RecoverAccountArgs,
    },
    RequestSplit {
        accounts: RequestSplitAccounts,
        args: RequestSplitArgs,
    },
    SlashAccount {
        accounts: SlashAccountAccounts,
        args: SlashAccountArgs,
    },
    UpdateAgreementHash {
        accounts: UpdateAgreementHashAccounts,
        args: UpdateAgreementHashArgs,
    },
    UpdateGovernanceAuthority {
        accounts: UpdateGovernanceAuthorityAccounts,
        args: UpdateGovernanceAuthorityArgs,
    },
    UpdateMaxVoterWeight {
        accounts: UpdateMaxVoterWeightAccounts,
        args: UpdateMaxVoterWeightArgs,
    },
    UpdatePdaAuthority {
        accounts: UpdatePdaAuthorityAccounts,
        args: UpdatePdaAuthorityArgs,
    },
    UpdatePoolAuthority {
        accounts: UpdatePoolAuthorityAccounts,
        args: UpdatePoolAuthorityArgs,
    },
    UpdateTokenListTime {
        accounts: UpdateTokenListTimeAccounts,
        args: UpdateTokenListTimeArgs,
    },
    UpdateVoterWeight {
        accounts: UpdateVoterWeightAccounts,
        args: UpdateVoterWeightArgs,
    },
    WithdrawStake {
        accounts: WithdrawStakeAccounts,
        args: WithdrawStakeArgs,
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
            [177u8, 172u8, 17u8, 93u8, 193u8, 86u8, 54u8, 222u8] => {
                let mut rdr: &[u8] = rest;
                let args = AcceptSplitArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pda_authority = keys.next().unwrap().clone();
                let source_stake_account_positions = keys.next().unwrap().clone();
                let source_stake_account_metadata = keys.next().unwrap().clone();
                let source_stake_account_split_request = keys.next().unwrap().clone();
                let source_stake_account_custody = keys.next().unwrap().clone();
                let source_custody_authority = keys.next().unwrap().clone();
                let new_stake_account_positions = keys.next().unwrap().clone();
                let new_stake_account_metadata = keys.next().unwrap().clone();
                let new_stake_account_custody = keys.next().unwrap().clone();
                let new_custody_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let pyth_token_mint = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AcceptSplitAccounts {
                    pda_authority,
                    source_stake_account_positions,
                    source_stake_account_metadata,
                    source_stake_account_split_request,
                    source_stake_account_custody,
                    source_custody_authority,
                    new_stake_account_positions,
                    new_stake_account_metadata,
                    new_stake_account_custody,
                    new_custody_authority,
                    config,
                    pyth_token_mint,
                    rent,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::AcceptSplit { accounts, args });
            }
            [52u8, 57u8, 147u8, 111u8, 56u8, 227u8, 33u8, 127u8] => {
                let mut rdr: &[u8] = rest;
                let args = AdvanceClockArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AdvanceClockAccounts { config, remaining };
                return Ok(Instruction::AdvanceClock { accounts, args });
            }
            [123u8, 134u8, 81u8, 0u8, 49u8, 68u8, 98u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_custody = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let target_account = keys.next().unwrap().clone();
                let pool_authority = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePositionAccounts {
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_custody,
                    config,
                    target_account,
                    pool_authority,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::ClosePosition { accounts, args });
            }
            [48u8, 215u8, 197u8, 153u8, 96u8, 203u8, 180u8, 133u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreatePositionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_custody = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let target_account = keys.next().unwrap().clone();
                let pool_authority = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreatePositionAccounts {
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_custody,
                    config,
                    target_account,
                    pool_authority,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreatePosition { accounts, args });
            }
            [105u8, 24u8, 131u8, 19u8, 201u8, 250u8, 157u8, 73u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateStakeAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_custody = keys.next().unwrap().clone();
                let custody_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let pyth_token_mint = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateStakeAccountAccounts {
                    payer,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_custody,
                    custody_authority,
                    config,
                    pyth_token_mint,
                    rent,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateStakeAccount { accounts, args });
            }
            [76u8, 144u8, 128u8, 239u8, 121u8, 210u8, 123u8, 39u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateTargetArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let governance_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let target_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTargetAccounts {
                    payer,
                    governance_authority,
                    config,
                    target_account,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTarget { accounts, args });
            }
            [3u8, 12u8, 113u8, 222u8, 177u8, 4u8, 152u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateVoterRecordArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let voter_record = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateVoterRecordAccounts {
                    payer,
                    stake_account_positions,
                    stake_account_metadata,
                    voter_record,
                    config,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateVoterRecord { accounts, args });
            }
            [219u8, 172u8, 149u8, 212u8, 103u8, 230u8, 164u8, 179u8] => {
                let mut rdr: &[u8] = rest;
                let args = ExportPositionTypeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let config_account = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ExportPositionTypeAccounts {
                    payer,
                    config_account,
                    rent,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::ExportPositionType { accounts, args });
            }
            [23u8, 235u8, 115u8, 232u8, 168u8, 96u8, 1u8, 231u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let config_account = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitConfigAccounts {
                    payer,
                    config_account,
                    rent,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::InitConfig { accounts, args });
            }
            [79u8, 241u8, 203u8, 177u8, 232u8, 143u8, 124u8, 14u8] => {
                let mut rdr: &[u8] = rest;
                let args = JoinDaoLlcArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = JoinDaoLlcAccounts {
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    config,
                    remaining,
                };
                return Ok(Instruction::JoinDaoLlc { accounts, args });
            }
            [21u8, 136u8, 57u8, 2u8, 204u8, 219u8, 242u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let args = MergeTargetPositionsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let pool_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MergeTargetPositionsAccounts {
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    config,
                    pool_authority,
                    remaining,
                };
                return Ok(Instruction::MergeTargetPositions { accounts, args });
            }
            [240u8, 223u8, 246u8, 118u8, 26u8, 121u8, 34u8, 128u8] => {
                let mut rdr: &[u8] = rest;
                let args = RecoverAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let governance_authority = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let voter_record = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RecoverAccountAccounts {
                    governance_authority,
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    voter_record,
                    config,
                    remaining,
                };
                return Ok(Instruction::RecoverAccount { accounts, args });
            }
            [133u8, 146u8, 228u8, 165u8, 251u8, 207u8, 146u8, 23u8] => {
                let mut rdr: &[u8] = rest;
                let args = RequestSplitArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_split_request = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RequestSplitAccounts {
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_split_request,
                    config,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::RequestSplit { accounts, args });
            }
            [185u8, 97u8, 8u8, 208u8, 118u8, 205u8, 166u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let args = SlashAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool_authority = keys.next().unwrap().clone();
                let publisher = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_custody = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let governance_target_account = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let custody_authority = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SlashAccountAccounts {
                    pool_authority,
                    publisher,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_custody,
                    config,
                    governance_target_account,
                    destination,
                    custody_authority,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::SlashAccount { accounts, args });
            }
            [86u8, 232u8, 181u8, 137u8, 158u8, 110u8, 129u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateAgreementHashArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let governance_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAgreementHashAccounts {
                    governance_authority,
                    config,
                    remaining,
                };
                return Ok(Instruction::UpdateAgreementHash { accounts, args });
            }
            [11u8, 185u8, 227u8, 55u8, 39u8, 32u8, 168u8, 14u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGovernanceAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let governance_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGovernanceAuthorityAccounts {
                    governance_authority,
                    config,
                    remaining,
                };
                return Ok(Instruction::UpdateGovernanceAuthority { accounts, args });
            }
            [248u8, 36u8, 229u8, 234u8, 11u8, 132u8, 145u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateMaxVoterWeightArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let max_voter_record = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateMaxVoterWeightAccounts {
                    payer,
                    max_voter_record,
                    config,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::UpdateMaxVoterWeight { accounts, args });
            }
            [178u8, 112u8, 199u8, 196u8, 59u8, 40u8, 140u8, 61u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePdaAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pda_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePdaAuthorityAccounts {
                    pda_authority,
                    config,
                    remaining,
                };
                return Ok(Instruction::UpdatePdaAuthority { accounts, args });
            }
            [160u8, 162u8, 113u8, 9u8, 99u8, 187u8, 23u8, 239u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePoolAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let governance_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePoolAuthorityAccounts {
                    governance_authority,
                    config,
                    remaining,
                };
                return Ok(Instruction::UpdatePoolAuthority { accounts, args });
            }
            [38u8, 217u8, 99u8, 222u8, 253u8, 253u8, 31u8, 83u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateTokenListTimeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let governance_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTokenListTimeAccounts {
                    governance_authority,
                    config,
                    remaining,
                };
                return Ok(Instruction::UpdateTokenListTime { accounts, args });
            }
            [92u8, 35u8, 133u8, 94u8, 230u8, 70u8, 14u8, 157u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateVoterWeightArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_custody = keys.next().unwrap().clone();
                let voter_record = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let governance_target = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateVoterWeightAccounts {
                    owner,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_custody,
                    voter_record,
                    config,
                    governance_target,
                    remaining,
                };
                return Ok(Instruction::UpdateVoterWeight { accounts, args });
            }
            [153u8, 8u8, 22u8, 138u8, 105u8, 176u8, 87u8, 66u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawStakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let stake_account_positions = keys.next().unwrap().clone();
                let stake_account_metadata = keys.next().unwrap().clone();
                let stake_account_custody = keys.next().unwrap().clone();
                let custody_authority = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStakeAccounts {
                    owner,
                    destination,
                    stake_account_positions,
                    stake_account_metadata,
                    stake_account_custody,
                    custody_authority,
                    config,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::WithdrawStake { accounts, args });
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
