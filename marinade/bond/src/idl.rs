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
use crate::pubkey_serializer::pubkey_serde;
use crate::pubkey_serializer::pubkey_serde_option;
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
    pub struct PubkeyValueChange {
        #[serde(with = "pubkey_serde")]
        pub old: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub new: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct U64ValueChange {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub old: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub new: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DelegationInfo {
        #[serde(with = "pubkey_serde")]
        pub voter_pubkey: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub stake: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub activation_epoch: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deactivation_epoch: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SplitStakeData {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ConfigureBondWithMintArgs {
        #[serde(with = "pubkey_serde")]
        pub validator_identity: [u8; 32usize],
        #[serde(with = "pubkey_serde_option")]
        pub bond_authority: Option<[u8; 32usize]>,
        pub cpmpe: Option<u64>,
        pub max_stake_wanted: Option<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ConfigureBondArgs {
        #[serde(with = "pubkey_serde_option")]
        pub bond_authority: Option<[u8; 32usize]>,
        pub cpmpe: Option<u64>,
        pub max_stake_wanted: Option<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitBondArgs {
        #[serde(with = "pubkey_serde")]
        pub bond_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cpmpe: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_stake_wanted: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ConfigureConfigArgs {
        #[serde(with = "pubkey_serde_option")]
        pub admin: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub operator: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub pause_authority: Option<[u8; 32usize]>,
        pub epochs_to_claim_settlement: Option<u64>,
        pub withdraw_lockup_epochs: Option<u64>,
        pub minimum_stake_lamports: Option<u64>,
        pub slots_to_start_settlement_claiming: Option<u64>,
        pub min_bond_max_stake_wanted: Option<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitConfigArgs {
        #[serde(with = "pubkey_serde")]
        pub admin_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub operator_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epochs_to_claim_settlement: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_lockup_epochs: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slots_to_start_settlement_claiming: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ClaimSettlementV2Args {
        pub proof: Vec<[u8; 32usize]>,
        pub tree_node_hash: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_staker: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_withdrawer: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub index: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ClaimSettlementArgs {
        pub proof: Vec<[u8; 32usize]>,
        pub tree_node_hash: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_staker: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_withdrawer: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub claim: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitSettlementArgs {
        pub merkle_root: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_merkle_nodes: u64,
        #[serde(with = "pubkey_serde")]
        pub rent_collector: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MergeStakeArgs {
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitWithdrawRequestArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Bumps {
        pub pda: u8,
        pub staker_authority: u8,
        pub settlement_claims: u8,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitConfigAccounts {
        pub config: String,
        pub rentPayer: String,
        pub systemProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConfigureConfigAccounts {
        pub config: String,
        pub adminAuthority: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseSettlementClaimAccounts {
        pub settlement: String,
        pub settlementClaim: String,
        pub rentCollector: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitBondAccounts {
        pub config: String,
        pub voteAccount: String,
        pub validatorIdentity: String,
        pub bond: String,
        pub rentPayer: String,
        pub systemProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConfigureBondAccounts {
        pub config: String,
        pub bond: String,
        pub authority: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub voteAccount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConfigureBondWithMintAccounts {
        pub config: String,
        pub bond: String,
        pub mint: String,
        pub voteAccount: String,
        pub tokenAccount: String,
        pub tokenAuthority: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MintBondAccounts {
        pub config: String,
        pub bond: String,
        pub mint: String,
        pub validatorIdentity: String,
        pub validatorIdentityTokenAccount: String,
        pub voteAccount: String,
        pub metadata: String,
        pub rentPayer: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub metadataProgram: String,
        pub rent: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FundBondAccounts {
        pub config: String,
        pub bond: String,
        pub bondsWithdrawerAuthority: String,
        pub stakeAccount: String,
        pub stakeAuthority: String,
        pub clock: String,
        pub stakeHistory: String,
        pub stakeProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitWithdrawRequestAccounts {
        pub config: String,
        pub bond: String,
        pub voteAccount: String,
        pub authority: String,
        pub withdrawRequest: String,
        pub rentPayer: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelWithdrawRequestAccounts {
        pub config: String,
        pub bond: String,
        pub voteAccount: String,
        pub authority: String,
        pub withdrawRequest: String,
        pub rentCollector: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimWithdrawRequestAccounts {
        pub config: String,
        pub bond: String,
        pub voteAccount: String,
        pub authority: String,
        pub withdrawRequest: String,
        pub bondsWithdrawerAuthority: String,
        pub stakeAccount: String,
        pub withdrawer: String,
        pub splitStakeAccount: String,
        pub splitStakeRentPayer: String,
        pub stakeProgram: String,
        pub systemProgram: String,
        pub stakeHistory: String,
        pub clock: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitSettlementAccounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub settlementClaims: Option<String>,
        pub operatorAuthority: String,
        pub rentPayer: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpsizeSettlementClaimsAccounts {
        pub settlementClaims: String,
        pub rentPayer: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelSettlementAccounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        pub settlementClaims: String,
        pub authority: String,
        pub bondsWithdrawerAuthority: String,
        pub rentCollector: String,
        pub splitRentCollector: String,
        pub splitRentRefundAccount: String,
        pub clock: String,
        pub stakeProgram: String,
        pub stakeHistory: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FundSettlementAccounts {
        pub config: String,
        pub bond: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub voteAccount: Option<String>,
        pub settlement: String,
        pub operatorAuthority: String,
        pub stakeAccount: String,
        pub settlementStakerAuthority: String,
        pub bondsWithdrawerAuthority: String,
        pub splitStakeAccount: String,
        pub splitStakeRentPayer: String,
        pub systemProgram: String,
        pub stakeHistory: String,
        pub clock: String,
        pub rent: String,
        pub stakeProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stakeConfig: Option<String>,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MergeStakeAccounts {
        pub config: String,
        pub sourceStake: String,
        pub destinationStake: String,
        pub stakerAuthority: String,
        pub stakeHistory: String,
        pub clock: String,
        pub stakeProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResetStakeAccounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        pub stakeAccount: String,
        pub bondsWithdrawerAuthority: String,
        pub voteAccount: String,
        pub stakeHistory: String,
        pub stakeConfig: String,
        pub clock: String,
        pub stakeProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawStakeAccounts {
        pub config: String,
        pub operatorAuthority: String,
        pub settlement: String,
        pub stakeAccount: String,
        pub bondsWithdrawerAuthority: String,
        pub withdrawTo: String,
        pub stakeHistory: String,
        pub clock: String,
        pub stakeProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EmergencyPauseAccounts {
        pub config: String,
        pub pauseAuthority: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EmergencyResumeAccounts {
        pub config: String,
        pub pauseAuthority: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseSettlementAccounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        pub bondsWithdrawerAuthority: String,
        pub rentCollector: String,
        pub splitRentCollector: String,
        pub splitRentRefundAccount: String,
        pub clock: String,
        pub stakeProgram: String,
        pub stakeHistory: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseSettlementV2Accounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        pub settlementClaims: String,
        pub bondsWithdrawerAuthority: String,
        pub rentCollector: String,
        pub splitRentCollector: String,
        pub splitRentRefundAccount: String,
        pub clock: String,
        pub stakeProgram: String,
        pub stakeHistory: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimSettlementV2Accounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        pub settlementClaims: String,
        pub stakeAccountFrom: String,
        pub stakeAccountTo: String,
        pub bondsWithdrawerAuthority: String,
        pub stakeHistory: String,
        pub clock: String,
        pub stakeProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimSettlementAccounts {
        pub config: String,
        pub bond: String,
        pub settlement: String,
        pub settlementClaims: String,
        pub stakeAccountFrom: String,
        pub stakeAccountTo: String,
        pub bondsWithdrawerAuthority: String,
        pub stakeHistory: String,
        pub clock: String,
        pub stakeProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitConfigArguments {
        pub init_config_args: InitConfigArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigureConfigArguments {
        pub configure_config_args: ConfigureConfigArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseSettlementClaimArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitBondArguments {
        pub init_bond_args: InitBondArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigureBondArguments {
        pub configure_bond_args: ConfigureBondArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigureBondWithMintArguments {
        pub args: ConfigureBondWithMintArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MintBondArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundBondArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitWithdrawRequestArguments {
        pub create_withdraw_request_args: InitWithdrawRequestArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CancelWithdrawRequestArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimWithdrawRequestArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitSettlementArguments {
        pub init_settlement_args: InitSettlementArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpsizeSettlementClaimsArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CancelSettlementArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundSettlementArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MergeStakeArguments {
        pub merge_args: MergeStakeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ResetStakeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawStakeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EmergencyPauseArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EmergencyResumeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseSettlementArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseSettlementV2Arguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimSettlementV2Arguments {
        pub claim_settlement_args: ClaimSettlementV2Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimSettlementArguments {
        pub claim_settlement_args: ClaimSettlementArgs,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitConfig {
        accounts: InitConfigAccounts,
        args: InitConfigArguments,
    },
    ConfigureConfig {
        accounts: ConfigureConfigAccounts,
        args: ConfigureConfigArguments,
    },
    CloseSettlementClaim {
        accounts: CloseSettlementClaimAccounts,
        args: CloseSettlementClaimArguments,
    },
    InitBond {
        accounts: InitBondAccounts,
        args: InitBondArguments,
    },
    ConfigureBond {
        accounts: ConfigureBondAccounts,
        args: ConfigureBondArguments,
    },
    ConfigureBondWithMint {
        accounts: ConfigureBondWithMintAccounts,
        args: ConfigureBondWithMintArguments,
    },
    MintBond {
        accounts: MintBondAccounts,
        args: MintBondArguments,
    },
    FundBond {
        accounts: FundBondAccounts,
        args: FundBondArguments,
    },
    InitWithdrawRequest {
        accounts: InitWithdrawRequestAccounts,
        args: InitWithdrawRequestArguments,
    },
    CancelWithdrawRequest {
        accounts: CancelWithdrawRequestAccounts,
        args: CancelWithdrawRequestArguments,
    },
    ClaimWithdrawRequest {
        accounts: ClaimWithdrawRequestAccounts,
        args: ClaimWithdrawRequestArguments,
    },
    InitSettlement {
        accounts: InitSettlementAccounts,
        args: InitSettlementArguments,
    },
    UpsizeSettlementClaims {
        accounts: UpsizeSettlementClaimsAccounts,
        args: UpsizeSettlementClaimsArguments,
    },
    CancelSettlement {
        accounts: CancelSettlementAccounts,
        args: CancelSettlementArguments,
    },
    FundSettlement {
        accounts: FundSettlementAccounts,
        args: FundSettlementArguments,
    },
    MergeStake {
        accounts: MergeStakeAccounts,
        args: MergeStakeArguments,
    },
    ResetStake {
        accounts: ResetStakeAccounts,
        args: ResetStakeArguments,
    },
    WithdrawStake {
        accounts: WithdrawStakeAccounts,
        args: WithdrawStakeArguments,
    },
    EmergencyPause {
        accounts: EmergencyPauseAccounts,
        args: EmergencyPauseArguments,
    },
    EmergencyResume {
        accounts: EmergencyResumeAccounts,
        args: EmergencyResumeArguments,
    },
    CloseSettlement {
        accounts: CloseSettlementAccounts,
        args: CloseSettlementArguments,
    },
    CloseSettlementV2 {
        accounts: CloseSettlementV2Accounts,
        args: CloseSettlementV2Arguments,
    },
    ClaimSettlementV2 {
        accounts: ClaimSettlementV2Accounts,
        args: ClaimSettlementV2Arguments,
    },
    ClaimSettlement {
        accounts: ClaimSettlementAccounts,
        args: ClaimSettlementArguments,
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
            [23u8, 235u8, 115u8, 232u8, 168u8, 96u8, 1u8, 231u8] => {
                let mut rdr: &[u8] = rest;
                let init_config_args: InitConfigArgs = InitConfigArgs::deserialize(&mut rdr)?;
                let args = InitConfigArguments { init_config_args };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = InitConfigAccounts {
                    config,
                    rentPayer,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitConfig { accounts, args });
            }
            [198u8, 98u8, 161u8, 165u8, 137u8, 200u8, 230u8, 203u8] => {
                let mut rdr: &[u8] = rest;
                let configure_config_args: ConfigureConfigArgs =
                    ConfigureConfigArgs::deserialize(&mut rdr)?;
                let args = ConfigureConfigArguments {
                    configure_config_args,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let config = keys.next().unwrap().clone();
                let adminAuthority = keys.next().unwrap().clone();
                let eventAuthority = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = ConfigureConfigAccounts {
                    config,
                    adminAuthority,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ConfigureConfig { accounts, args });
            }
            [103u8, 26u8, 247u8, 104u8, 254u8, 134u8, 218u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseSettlementClaimArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let settlement = keys.next().unwrap().clone();
                let settlementClaim = keys.next().unwrap().clone();
                let rentCollector = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseSettlementClaimAccounts {
                    settlement,
                    settlementClaim,
                    rentCollector,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CloseSettlementClaim { accounts, args });
            }
            [95u8, 93u8, 93u8, 181u8, 221u8, 36u8, 126u8, 64u8] => {
                let mut rdr: &[u8] = rest;
                let init_bond_args: InitBondArgs = InitBondArgs::deserialize(&mut rdr)?;
                let args = InitBondArguments { init_bond_args };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let config = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let validatorIdentity = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = InitBondAccounts {
                    config,
                    voteAccount,
                    validatorIdentity,
                    bond,
                    rentPayer,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitBond { accounts, args });
            }
            [228u8, 108u8, 79u8, 242u8, 82u8, 54u8, 105u8, 65u8] => {
                let mut rdr: &[u8] = rest;
                let configure_bond_args: ConfigureBondArgs =
                    ConfigureBondArgs::deserialize(&mut rdr)?;
                let args = ConfigureBondArguments {
                    configure_bond_args,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let voteAccount = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = ConfigureBondAccounts {
                    config,
                    bond,
                    authority,
                    voteAccount,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ConfigureBond { accounts, args });
            }
            [48u8, 189u8, 230u8, 39u8, 112u8, 33u8, 227u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let args: ConfigureBondWithMintArgs =
                    ConfigureBondWithMintArgs::deserialize(&mut rdr)?;
                let args = ConfigureBondWithMintArguments { args };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 9usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let tokenAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfigureBondWithMintAccounts {
                    config,
                    bond,
                    mint,
                    voteAccount,
                    tokenAccount,
                    tokenAuthority,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ConfigureBondWithMint { accounts, args });
            }
            [234u8, 94u8, 85u8, 225u8, 167u8, 102u8, 169u8, 32u8] => {
                let mut rdr: &[u8] = rest;
                let args = MintBondArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 13usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let validatorIdentity = keys.next().unwrap().clone();
                let validatorIdentityTokenAccount = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = MintBondAccounts {
                    config,
                    bond,
                    mint,
                    validatorIdentity,
                    validatorIdentityTokenAccount,
                    voteAccount,
                    metadata,
                    rentPayer,
                    systemProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    metadataProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::MintBond { accounts, args });
            }
            [58u8, 44u8, 212u8, 175u8, 30u8, 17u8, 68u8, 62u8] => {
                let mut rdr: &[u8] = rest;
                let args = FundBondArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let stakeAuthority = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let eventAuthority = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = FundBondAccounts {
                    config,
                    bond,
                    bondsWithdrawerAuthority,
                    stakeAccount,
                    stakeAuthority,
                    clock,
                    stakeHistory,
                    stakeProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::FundBond { accounts, args });
            }
            [142u8, 31u8, 222u8, 215u8, 83u8, 79u8, 34u8, 49u8] => {
                let mut rdr: &[u8] = rest;
                let create_withdraw_request_args: InitWithdrawRequestArgs =
                    InitWithdrawRequestArgs::deserialize(&mut rdr)?;
                let args = InitWithdrawRequestArguments {
                    create_withdraw_request_args,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 9usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let withdrawRequest = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitWithdrawRequestAccounts {
                    config,
                    bond,
                    voteAccount,
                    authority,
                    withdrawRequest,
                    rentPayer,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitWithdrawRequest { accounts, args });
            }
            [167u8, 100u8, 110u8, 128u8, 113u8, 154u8, 224u8, 77u8] => {
                let mut rdr: &[u8] = rest;
                let args = CancelWithdrawRequestArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let withdrawRequest = keys.next().unwrap().clone();
                let rentCollector = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelWithdrawRequestAccounts {
                    config,
                    bond,
                    voteAccount,
                    authority,
                    withdrawRequest,
                    rentCollector,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CancelWithdrawRequest { accounts, args });
            }
            [48u8, 232u8, 23u8, 52u8, 20u8, 134u8, 122u8, 118u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimWithdrawRequestArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 16usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let withdrawRequest = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let withdrawer = keys.next().unwrap().clone();
                let splitStakeAccount = keys.next().unwrap().clone();
                let splitStakeRentPayer = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimWithdrawRequestAccounts {
                    config,
                    bond,
                    voteAccount,
                    authority,
                    withdrawRequest,
                    bondsWithdrawerAuthority,
                    stakeAccount,
                    withdrawer,
                    splitStakeAccount,
                    splitStakeRentPayer,
                    stakeProgram,
                    systemProgram,
                    stakeHistory,
                    clock,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimWithdrawRequest { accounts, args });
            }
            [152u8, 178u8, 0u8, 65u8, 52u8, 210u8, 247u8, 58u8] => {
                let mut rdr: &[u8] = rest;
                let init_settlement_args: InitSettlementArgs =
                    InitSettlementArgs::deserialize(&mut rdr)?;
                let args = InitSettlementArguments {
                    init_settlement_args,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 8usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let settlementClaims = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let operatorAuthority = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitSettlementAccounts {
                    config,
                    bond,
                    settlement,
                    settlementClaims,
                    operatorAuthority,
                    rentPayer,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitSettlement { accounts, args });
            }
            [207u8, 46u8, 34u8, 88u8, 141u8, 36u8, 63u8, 132u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpsizeSettlementClaimsArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let settlementClaims = keys.next().unwrap().clone();
                let rentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpsizeSettlementClaimsAccounts {
                    settlementClaims,
                    rentPayer,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::UpsizeSettlementClaims { accounts, args });
            }
            [33u8, 241u8, 96u8, 62u8, 228u8, 178u8, 1u8, 120u8] => {
                let mut rdr: &[u8] = rest;
                let args = CancelSettlementArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 14usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let settlementClaims = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let rentCollector = keys.next().unwrap().clone();
                let splitRentCollector = keys.next().unwrap().clone();
                let splitRentRefundAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelSettlementAccounts {
                    config,
                    bond,
                    settlement,
                    settlementClaims,
                    authority,
                    bondsWithdrawerAuthority,
                    rentCollector,
                    splitRentCollector,
                    splitRentRefundAccount,
                    clock,
                    stakeProgram,
                    stakeHistory,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CancelSettlement { accounts, args });
            }
            [179u8, 146u8, 113u8, 34u8, 30u8, 92u8, 26u8, 19u8] => {
                let mut rdr: &[u8] = rest;
                let args = FundSettlementArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 16usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let voteAccount = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let settlement = keys.next().unwrap().clone();
                let operatorAuthority = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let settlementStakerAuthority = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let splitStakeAccount = keys.next().unwrap().clone();
                let splitStakeRentPayer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let stakeConfig = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FundSettlementAccounts {
                    config,
                    bond,
                    voteAccount,
                    settlement,
                    operatorAuthority,
                    stakeAccount,
                    settlementStakerAuthority,
                    bondsWithdrawerAuthority,
                    splitStakeAccount,
                    splitStakeRentPayer,
                    systemProgram,
                    stakeHistory,
                    clock,
                    rent,
                    stakeProgram,
                    stakeConfig,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::FundSettlement { accounts, args });
            }
            [14u8, 3u8, 146u8, 23u8, 163u8, 105u8, 246u8, 99u8] => {
                let mut rdr: &[u8] = rest;
                let merge_args: MergeStakeArgs = MergeStakeArgs::deserialize(&mut rdr)?;
                let args = MergeStakeArguments { merge_args };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 9usize;
                let config = keys.next().unwrap().clone();
                let sourceStake = keys.next().unwrap().clone();
                let destinationStake = keys.next().unwrap().clone();
                let stakerAuthority = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MergeStakeAccounts {
                    config,
                    sourceStake,
                    destinationStake,
                    stakerAuthority,
                    stakeHistory,
                    clock,
                    stakeProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::MergeStake { accounts, args });
            }
            [183u8, 37u8, 69u8, 159u8, 163u8, 139u8, 212u8, 235u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResetStakeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 12usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let voteAccount = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let stakeConfig = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResetStakeAccounts {
                    config,
                    bond,
                    settlement,
                    stakeAccount,
                    bondsWithdrawerAuthority,
                    voteAccount,
                    stakeHistory,
                    stakeConfig,
                    clock,
                    stakeProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ResetStake { accounts, args });
            }
            [153u8, 8u8, 22u8, 138u8, 105u8, 176u8, 87u8, 66u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawStakeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 11usize;
                let config = keys.next().unwrap().clone();
                let operatorAuthority = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let stakeAccount = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let withdrawTo = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStakeAccounts {
                    config,
                    operatorAuthority,
                    settlement,
                    stakeAccount,
                    bondsWithdrawerAuthority,
                    withdrawTo,
                    stakeHistory,
                    clock,
                    stakeProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::WithdrawStake { accounts, args });
            }
            [21u8, 143u8, 27u8, 142u8, 200u8, 181u8, 210u8, 255u8] => {
                let mut rdr: &[u8] = rest;
                let args = EmergencyPauseArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let config = keys.next().unwrap().clone();
                let pauseAuthority = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EmergencyPauseAccounts {
                    config,
                    pauseAuthority,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::EmergencyPause { accounts, args });
            }
            [0u8, 243u8, 48u8, 185u8, 6u8, 73u8, 190u8, 83u8] => {
                let mut rdr: &[u8] = rest;
                let args = EmergencyResumeArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let config = keys.next().unwrap().clone();
                let pauseAuthority = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EmergencyResumeAccounts {
                    config,
                    pauseAuthority,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::EmergencyResume { accounts, args });
            }
            [45u8, 247u8, 83u8, 183u8, 24u8, 102u8, 0u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseSettlementArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 12usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let rentCollector = keys.next().unwrap().clone();
                let splitRentCollector = keys.next().unwrap().clone();
                let splitRentRefundAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseSettlementAccounts {
                    config,
                    bond,
                    settlement,
                    bondsWithdrawerAuthority,
                    rentCollector,
                    splitRentCollector,
                    splitRentRefundAccount,
                    clock,
                    stakeProgram,
                    stakeHistory,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CloseSettlement { accounts, args });
            }
            [125u8, 212u8, 89u8, 37u8, 31u8, 244u8, 191u8, 179u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseSettlementV2Arguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 13usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let settlementClaims = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let rentCollector = keys.next().unwrap().clone();
                let splitRentCollector = keys.next().unwrap().clone();
                let splitRentRefundAccount = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseSettlementV2Accounts {
                    config,
                    bond,
                    settlement,
                    settlementClaims,
                    bondsWithdrawerAuthority,
                    rentCollector,
                    splitRentCollector,
                    splitRentRefundAccount,
                    clock,
                    stakeProgram,
                    stakeHistory,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CloseSettlementV2 { accounts, args });
            }
            [188u8, 53u8, 132u8, 151u8, 88u8, 50u8, 52u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let claim_settlement_args: ClaimSettlementV2Args =
                    ClaimSettlementV2Args::deserialize(&mut rdr)?;
                let args = ClaimSettlementV2Arguments {
                    claim_settlement_args,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 12usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let settlementClaims = keys.next().unwrap().clone();
                let stakeAccountFrom = keys.next().unwrap().clone();
                let stakeAccountTo = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimSettlementV2Accounts {
                    config,
                    bond,
                    settlement,
                    settlementClaims,
                    stakeAccountFrom,
                    stakeAccountTo,
                    bondsWithdrawerAuthority,
                    stakeHistory,
                    clock,
                    stakeProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimSettlementV2 { accounts, args });
            }
            [85u8, 208u8, 73u8, 229u8, 143u8, 98u8, 83u8, 212u8] => {
                let mut rdr: &[u8] = rest;
                let claim_settlement_args: ClaimSettlementArgs =
                    ClaimSettlementArgs::deserialize(&mut rdr)?;
                let args = ClaimSettlementArguments {
                    claim_settlement_args,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 12usize;
                let config = keys.next().unwrap().clone();
                let bond = keys.next().unwrap().clone();
                let settlement = keys.next().unwrap().clone();
                let settlementClaims = keys.next().unwrap().clone();
                let stakeAccountFrom = keys.next().unwrap().clone();
                let stakeAccountTo = keys.next().unwrap().clone();
                let bondsWithdrawerAuthority = keys.next().unwrap().clone();
                let stakeHistory = keys.next().unwrap().clone();
                let clock = keys.next().unwrap().clone();
                let stakeProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimSettlementAccounts {
                    config,
                    bond,
                    settlement,
                    settlementClaims,
                    stakeAccountFrom,
                    stakeAccountTo,
                    bondsWithdrawerAuthority,
                    stakeHistory,
                    clock,
                    stakeProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimSettlement { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitBondEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vote_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub validator_identity: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cpmpe: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_stake_wanted: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigureBondEvent {
        pub bond_authority: Option<PubkeyValueChange>,
        pub cpmpe: Option<U64ValueChange>,
        pub max_stake_wanted: Option<U64ValueChange>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigureBondWithMintEvent {
        #[serde(with = "pubkey_serde")]
        pub validator_identity: [u8; 32usize],
        pub bond_authority: Option<PubkeyValueChange>,
        pub cpmpe: Option<U64ValueChange>,
        pub max_stake_wanted: Option<U64ValueChange>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundBondEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vote_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_authority_signer: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposited_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MintBondEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub validator_identity: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub validator_identity_token_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_metadata: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitConfigEvent {
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub admin_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub operator_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_lockup_epochs: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epochs_to_claim_settlement: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_stake_lamports: u64,
        #[serde(with = "pubkey_serde")]
        pub bonds_withdrawer_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slots_to_start_settlement_claiming: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConfigureConfigEvent {
        pub admin_authority: Option<PubkeyValueChange>,
        pub operator_authority: Option<PubkeyValueChange>,
        pub pause_authority: Option<PubkeyValueChange>,
        pub epochs_to_claim_settlement: Option<U64ValueChange>,
        pub minimum_stake_lamports: Option<U64ValueChange>,
        pub withdraw_lockup_epochs: Option<U64ValueChange>,
        pub slots_to_start_settlement_claiming: Option<U64ValueChange>,
        pub min_bond_max_stake_wanted: Option<U64ValueChange>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EmergencyPauseEvent {
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub admin_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub operator_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epochs_to_claim_settlement: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_lockup_epochs: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_stake_lamports: u64,
        #[serde(with = "pubkey_serde")]
        pub pause_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EmergencyResumeEvent {
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub admin_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub operator_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epochs_to_claim_settlement: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_lockup_epochs: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_stake_lamports: u64,
        #[serde(with = "pubkey_serde")]
        pub pause_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimSettlementV2Event {
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        pub settlement_lamports_claimed: U64ValueChange,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub settlement_merkle_nodes_claimed: u64,
        #[serde(with = "pubkey_serde")]
        pub stake_account_to: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_withdrawer: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_staker: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitSettlementEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vote_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub staker_authority: [u8; 32usize],
        pub merkle_root: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_merkle_nodes: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch_created_for: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub slot_created_at: u64,
        #[serde(with = "pubkey_serde")]
        pub rent_collector: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseSettlementEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        pub merkle_root: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_merkle_nodes: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_funded: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_claimed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub merkle_nodes_claimed: u64,
        #[serde(with = "pubkey_serde_option")]
        pub split_rent_collector: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub split_rent_refund: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde")]
        pub rent_collector: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub expiration_epoch: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub current_epoch: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CancelSettlementEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        pub merkle_root: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_merkle_nodes: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_funded: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_claimed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub merkle_nodes_claimed: u64,
        #[serde(with = "pubkey_serde_option")]
        pub split_rent_collector: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde_option")]
        pub split_rent_refund: Option<[u8; 32usize]>,
        #[serde(with = "pubkey_serde")]
        pub rent_collector: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundSettlementEvent {
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub funding_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub stake_account: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_funded: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_claimed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub merkle_nodes_claimed: u64,
        pub split_stake_account: Option<SplitStakeData>,
        #[serde(with = "pubkey_serde_option")]
        pub split_rent_collector: Option<[u8; 32usize]>,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub split_rent_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MergeStakeEvent {
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub staker_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub destination_stake: [u8; 32usize],
        pub destination_delegation: Option<DelegationInfo>,
        #[serde(with = "pubkey_serde")]
        pub source_stake: [u8; 32usize],
        pub source_delegation: Option<DelegationInfo>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ResetStakeEvent {
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vote_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement_staker_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawStakeEvent {
        #[serde(with = "pubkey_serde")]
        pub config: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub operator_authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub withdraw_to: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement_staker_authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdrawn_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitWithdrawRequestEvent {
        #[serde(with = "pubkey_serde")]
        pub withdraw_request: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vote_account: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub requested_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CancelWithdrawRequestEvent {
        #[serde(with = "pubkey_serde")]
        pub withdraw_request: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub requested_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdrawn_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimWithdrawRequestEvent {
        #[serde(with = "pubkey_serde")]
        pub withdraw_request: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub bond: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vote_account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account: [u8; 32usize],
        pub split_stake: Option<SplitStakeData>,
        #[serde(with = "pubkey_serde")]
        pub new_stake_account_owner: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdrawing_amount: u64,
        pub withdrawn_amount: U64ValueChange,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimSettlementEvent {
        #[serde(with = "pubkey_serde")]
        pub settlement_claim: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        pub settlement_lamports_claimed: U64ValueChange,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub settlement_merkle_nodes_claimed: u64,
        #[serde(with = "pubkey_serde")]
        pub stake_account_to: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_withdrawer: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub stake_account_staker: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        #[serde(with = "pubkey_serde")]
        pub rent_collector: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseSettlementClaimEvent {
        #[serde(with = "pubkey_serde")]
        pub settlement: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub rent_collector: [u8; 32usize],
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        InitBondEvent { args: InitBondEvent },
        ConfigureBondEvent { args: ConfigureBondEvent },
        ConfigureBondWithMintEvent { args: ConfigureBondWithMintEvent },
        FundBondEvent { args: FundBondEvent },
        MintBondEvent { args: MintBondEvent },
        InitConfigEvent { args: InitConfigEvent },
        ConfigureConfigEvent { args: ConfigureConfigEvent },
        EmergencyPauseEvent { args: EmergencyPauseEvent },
        EmergencyResumeEvent { args: EmergencyResumeEvent },
        ClaimSettlementV2Event { args: ClaimSettlementV2Event },
        InitSettlementEvent { args: InitSettlementEvent },
        CloseSettlementEvent { args: CloseSettlementEvent },
        CancelSettlementEvent { args: CancelSettlementEvent },
        FundSettlementEvent { args: FundSettlementEvent },
        MergeStakeEvent { args: MergeStakeEvent },
        ResetStakeEvent { args: ResetStakeEvent },
        WithdrawStakeEvent { args: WithdrawStakeEvent },
        InitWithdrawRequestEvent { args: InitWithdrawRequestEvent },
        CancelWithdrawRequestEvent { args: CancelWithdrawRequestEvent },
        ClaimWithdrawRequestEvent { args: ClaimWithdrawRequestEvent },
        ClaimSettlementEvent { args: ClaimSettlementEvent },
        CloseSettlementClaimEvent { args: CloseSettlementClaimEvent },
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
                [56u8, 106u8, 209u8, 158u8, 171u8, 85u8, 159u8, 200u8] => {
                    let mut rdr = &payload[..];
                    let args = InitBondEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InitBondEvent { args });
                }
                [183u8, 119u8, 162u8, 244u8, 82u8, 182u8, 114u8, 228u8] => {
                    let mut rdr = &payload[..];
                    let args = ConfigureBondEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ConfigureBondEvent { args });
                }
                [209u8, 167u8, 200u8, 198u8, 99u8, 71u8, 4u8, 96u8] => {
                    let mut rdr = &payload[..];
                    let args = ConfigureBondWithMintEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ConfigureBondWithMintEvent { args });
                }
                [156u8, 63u8, 156u8, 252u8, 109u8, 181u8, 73u8, 110u8] => {
                    let mut rdr = &payload[..];
                    let args = FundBondEvent::deserialize(&mut rdr)?;
                    return Ok(Event::FundBondEvent { args });
                }
                [82u8, 190u8, 245u8, 33u8, 35u8, 128u8, 142u8, 197u8] => {
                    let mut rdr = &payload[..];
                    let args = MintBondEvent::deserialize(&mut rdr)?;
                    return Ok(Event::MintBondEvent { args });
                }
                [125u8, 127u8, 160u8, 86u8, 247u8, 110u8, 50u8, 238u8] => {
                    let mut rdr = &payload[..];
                    let args = InitConfigEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InitConfigEvent { args });
                }
                [121u8, 240u8, 38u8, 122u8, 0u8, 102u8, 203u8, 122u8] => {
                    let mut rdr = &payload[..];
                    let args = ConfigureConfigEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ConfigureConfigEvent { args });
                }
                [159u8, 241u8, 192u8, 232u8, 29u8, 208u8, 51u8, 21u8] => {
                    let mut rdr = &payload[..];
                    let args = EmergencyPauseEvent::deserialize(&mut rdr)?;
                    return Ok(Event::EmergencyPauseEvent { args });
                }
                [19u8, 211u8, 43u8, 129u8, 45u8, 168u8, 226u8, 200u8] => {
                    let mut rdr = &payload[..];
                    let args = EmergencyResumeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::EmergencyResumeEvent { args });
                }
                [114u8, 201u8, 131u8, 134u8, 182u8, 165u8, 237u8, 47u8] => {
                    let mut rdr = &payload[..];
                    let args = ClaimSettlementV2Event::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimSettlementV2Event { args });
                }
                [187u8, 195u8, 46u8, 129u8, 116u8, 83u8, 231u8, 241u8] => {
                    let mut rdr = &payload[..];
                    let args = InitSettlementEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InitSettlementEvent { args });
                }
                [226u8, 173u8, 111u8, 111u8, 105u8, 218u8, 118u8, 103u8] => {
                    let mut rdr = &payload[..];
                    let args = CloseSettlementEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CloseSettlementEvent { args });
                }
                [80u8, 190u8, 161u8, 61u8, 97u8, 7u8, 242u8, 92u8] => {
                    let mut rdr = &payload[..];
                    let args = CancelSettlementEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CancelSettlementEvent { args });
                }
                [104u8, 161u8, 6u8, 77u8, 82u8, 236u8, 4u8, 114u8] => {
                    let mut rdr = &payload[..];
                    let args = FundSettlementEvent::deserialize(&mut rdr)?;
                    return Ok(Event::FundSettlementEvent { args });
                }
                [111u8, 6u8, 45u8, 208u8, 79u8, 53u8, 119u8, 57u8] => {
                    let mut rdr = &payload[..];
                    let args = MergeStakeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::MergeStakeEvent { args });
                }
                [255u8, 49u8, 219u8, 199u8, 119u8, 10u8, 195u8, 177u8] => {
                    let mut rdr = &payload[..];
                    let args = ResetStakeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ResetStakeEvent { args });
                }
                [47u8, 85u8, 239u8, 214u8, 207u8, 29u8, 151u8, 88u8] => {
                    let mut rdr = &payload[..];
                    let args = WithdrawStakeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::WithdrawStakeEvent { args });
                }
                [122u8, 40u8, 131u8, 105u8, 70u8, 35u8, 119u8, 128u8] => {
                    let mut rdr = &payload[..];
                    let args = InitWithdrawRequestEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InitWithdrawRequestEvent { args });
                }
                [221u8, 97u8, 104u8, 35u8, 19u8, 137u8, 248u8, 246u8] => {
                    let mut rdr = &payload[..];
                    let args = CancelWithdrawRequestEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CancelWithdrawRequestEvent { args });
                }
                [201u8, 210u8, 144u8, 108u8, 235u8, 209u8, 85u8, 58u8] => {
                    let mut rdr = &payload[..];
                    let args = ClaimWithdrawRequestEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimWithdrawRequestEvent { args });
                }
                [135u8, 253u8, 145u8, 233u8, 227u8, 29u8, 188u8, 141u8] => {
                    let mut rdr = &payload[..];
                    let args = ClaimSettlementEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimSettlementEvent { args });
                }
                [131u8, 120u8, 161u8, 252u8, 249u8, 136u8, 153u8, 53u8] => {
                    let mut rdr = &payload[..];
                    let args = CloseSettlementClaimEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CloseSettlementClaimEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
