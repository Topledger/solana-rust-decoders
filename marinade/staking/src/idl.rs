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
}
pub mod accounts_data {
    use serde::Serialize;
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
        pub marinadeFinanceProgram: String,
        pub referralState: String,
        pub msolTokenPartnerAccount: String,
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
        pub marinadeFinanceProgram: String,
        pub referralState: String,
        pub msolTokenPartnerAccount: String,
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
        pub marinadeFinanceProgram: String,
        pub referralState: String,
        pub msolTokenPartnerAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub adminAccount: String,
        pub globalState: String,
        pub msolMintAccount: String,
        pub foreman1: String,
        pub foreman2: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitReferralAccountAccounts {
        pub globalState: String,
        pub signer: String,
        pub referralState: String,
        pub partnerAccount: String,
        pub msolTokenPartnerAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateReferralAccounts {
        pub globalState: String,
        pub adminAccount: String,
        pub referralState: String,
        pub newPartnerAccount: String,
        pub newMsolTokenPartnerAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateOperationFeesAccounts {
        pub globalState: String,
        pub signer: String,
        pub referralState: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ChangeAuthorityAccounts {
        pub globalState: String,
        pub adminAccount: String,
        pub newAdminAccount: String,
        pub newForeman1: String,
        pub newForeman2: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AdminRecognizeDepositAccounts {
        pub signer: String,
        pub globalState: String,
        pub referralState: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
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
    pub struct InitializeArgs {
        pub min_keep_pct: u8,
        pub max_keep_pct: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitReferralAccountArgs {
        pub partner_name: String,
        #[serde(with = "pubkey_serde_option")]
        pub validator_vote_key: Option<[u8; 32usize]>,
        pub keep_self_stake_pct: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateReferralArgs {
        pub pause: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateOperationFeesArgs {
        pub operation_deposit_sol_fee: Option<u8>,
        pub operation_deposit_stake_account_fee: Option<u8>,
        pub operation_liquid_unstake_fee: Option<u8>,
        pub operation_delayed_unstake_fee: Option<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ChangeAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AdminRecognizeDepositArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
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
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    InitReferralAccount {
        accounts: InitReferralAccountAccounts,
        args: InitReferralAccountArgs,
    },
    UpdateReferral {
        accounts: UpdateReferralAccounts,
        args: UpdateReferralArgs,
    },
    UpdateOperationFees {
        accounts: UpdateOperationFeesAccounts,
        args: UpdateOperationFeesArgs,
    },
    ChangeAuthority {
        accounts: ChangeAuthorityAccounts,
        args: ChangeAuthorityArgs,
    },
    AdminRecognizeDeposit {
        accounts: AdminRecognizeDepositAccounts,
        args: AdminRecognizeDepositArgs,
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
                let marinadeFinanceProgram = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let msolTokenPartnerAccount = keys.next().unwrap().clone();
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
                    marinadeFinanceProgram,
                    referralState,
                    msolTokenPartnerAccount,
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
                let marinadeFinanceProgram = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let msolTokenPartnerAccount = keys.next().unwrap().clone();
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
                    marinadeFinanceProgram,
                    referralState,
                    msolTokenPartnerAccount,
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
                let marinadeFinanceProgram = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let msolTokenPartnerAccount = keys.next().unwrap().clone();
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
                    marinadeFinanceProgram,
                    referralState,
                    msolTokenPartnerAccount,
                    remaining,
                };
                return Ok(Instruction::LiquidUnstake { accounts, args });
            }
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let adminAccount = keys.next().unwrap().clone();
                let globalState = keys.next().unwrap().clone();
                let msolMintAccount = keys.next().unwrap().clone();
                let foreman1 = keys.next().unwrap().clone();
                let foreman2 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    adminAccount,
                    globalState,
                    msolMintAccount,
                    foreman1,
                    foreman2,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [113u8, 173u8, 228u8, 216u8, 175u8, 224u8, 23u8, 187u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitReferralAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalState = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let partnerAccount = keys.next().unwrap().clone();
                let msolTokenPartnerAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitReferralAccountAccounts {
                    globalState,
                    signer,
                    referralState,
                    partnerAccount,
                    msolTokenPartnerAccount,
                    remaining,
                };
                return Ok(Instruction::InitReferralAccount { accounts, args });
            }
            [123u8, 215u8, 252u8, 27u8, 184u8, 244u8, 151u8, 197u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateReferralArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalState = keys.next().unwrap().clone();
                let adminAccount = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let newPartnerAccount = keys.next().unwrap().clone();
                let newMsolTokenPartnerAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateReferralAccounts {
                    globalState,
                    adminAccount,
                    referralState,
                    newPartnerAccount,
                    newMsolTokenPartnerAccount,
                    remaining,
                };
                return Ok(Instruction::UpdateReferral { accounts, args });
            }
            [229u8, 36u8, 92u8, 199u8, 165u8, 230u8, 7u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateOperationFeesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalState = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateOperationFeesAccounts {
                    globalState,
                    signer,
                    referralState,
                    remaining,
                };
                return Ok(Instruction::UpdateOperationFees { accounts, args });
            }
            [50u8, 106u8, 66u8, 104u8, 99u8, 118u8, 145u8, 88u8] => {
                let mut rdr: &[u8] = rest;
                let args = ChangeAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalState = keys.next().unwrap().clone();
                let adminAccount = keys.next().unwrap().clone();
                let newAdminAccount = keys.next().unwrap().clone();
                let newForeman1 = keys.next().unwrap().clone();
                let newForeman2 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ChangeAuthorityAccounts {
                    globalState,
                    adminAccount,
                    newAdminAccount,
                    newForeman1,
                    newForeman2,
                    remaining,
                };
                return Ok(Instruction::ChangeAuthority { accounts, args });
            }
            [114u8, 31u8, 225u8, 142u8, 124u8, 126u8, 182u8, 92u8] => {
                let mut rdr: &[u8] = rest;
                let args = AdminRecognizeDepositArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap().clone();
                let globalState = keys.next().unwrap().clone();
                let referralState = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AdminRecognizeDepositAccounts {
                    signer,
                    globalState,
                    referralState,
                    remaining,
                };
                return Ok(Instruction::AdminRecognizeDeposit { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
