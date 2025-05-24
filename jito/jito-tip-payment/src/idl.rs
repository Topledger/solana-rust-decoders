pub use accounts_data::*;
pub use ix_data::*;
#[allow(dead_code)]
use std::convert::TryInto;
use std::mem;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    use serde::Serialize;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InitBumps {
        pub config: u8,
        pub tip_payment_account0: u8,
        pub tip_payment_account1: u8,
        pub tip_payment_account2: u8,
        pub tip_payment_account3: u8,
        pub tip_payment_account4: u8,
        pub tip_payment_account5: u8,
        pub tip_payment_account6: u8,
        pub tip_payment_account7: u8,
    }
}
pub mod accounts_data {
    use super::*;
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub config: String,
        pub tipPaymentAccount0: String,
        pub tipPaymentAccount1: String,
        pub tipPaymentAccount2: String,
        pub tipPaymentAccount3: String,
        pub tipPaymentAccount4: String,
        pub tipPaymentAccount5: String,
        pub tipPaymentAccount6: String,
        pub tipPaymentAccount7: String,
        pub systemProgram: String,
        pub payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimTipsAccounts {
        pub config: String,
        pub tipPaymentAccount0: String,
        pub tipPaymentAccount1: String,
        pub tipPaymentAccount2: String,
        pub tipPaymentAccount3: String,
        pub tipPaymentAccount4: String,
        pub tipPaymentAccount5: String,
        pub tipPaymentAccount6: String,
        pub tipPaymentAccount7: String,
        pub tipReceiver: String,
        pub blockBuilder: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ChangeTipReceiverAccounts {
        pub config: String,
        pub oldTipReceiver: String,
        pub newTipReceiver: String,
        pub blockBuilder: String,
        pub tipPaymentAccount0: String,
        pub tipPaymentAccount1: String,
        pub tipPaymentAccount2: String,
        pub tipPaymentAccount3: String,
        pub tipPaymentAccount4: String,
        pub tipPaymentAccount5: String,
        pub tipPaymentAccount6: String,
        pub tipPaymentAccount7: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ChangeBlockBuilderAccounts {
        pub config: String,
        pub tipReceiver: String,
        pub oldBlockBuilder: String,
        pub newBlockBuilder: String,
        pub tipPaymentAccount0: String,
        pub tipPaymentAccount1: String,
        pub tipPaymentAccount2: String,
        pub tipPaymentAccount3: String,
        pub tipPaymentAccount4: String,
        pub tipPaymentAccount5: String,
        pub tipPaymentAccount6: String,
        pub tipPaymentAccount7: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeArgs {
        pub bumps: InitBumps,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimTipsArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ChangeTipReceiverArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ChangeBlockBuilderArgs {
        pub block_builder_commission: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    ClaimTips {
        accounts: ClaimTipsAccounts,
        args: ClaimTipsArgs,
    },
    ChangeTipReceiver {
        accounts: ChangeTipReceiverAccounts,
        args: ChangeTipReceiverArgs,
    },
    ChangeBlockBuilder {
        accounts: ChangeBlockBuilderAccounts,
        args: ChangeBlockBuilderArgs,
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
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<InitializeArgs>());
                let args = InitializeArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let tipPaymentAccount0 = keys.next().unwrap().clone();
                let tipPaymentAccount1 = keys.next().unwrap().clone();
                let tipPaymentAccount2 = keys.next().unwrap().clone();
                let tipPaymentAccount3 = keys.next().unwrap().clone();
                let tipPaymentAccount4 = keys.next().unwrap().clone();
                let tipPaymentAccount5 = keys.next().unwrap().clone();
                let tipPaymentAccount6 = keys.next().unwrap().clone();
                let tipPaymentAccount7 = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    config,
                    tipPaymentAccount0,
                    tipPaymentAccount1,
                    tipPaymentAccount2,
                    tipPaymentAccount3,
                    tipPaymentAccount4,
                    tipPaymentAccount5,
                    tipPaymentAccount6,
                    tipPaymentAccount7,
                    systemProgram,
                    payer,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [247u8, 28u8, 193u8, 228u8, 55u8, 238u8, 31u8, 113u8] => {
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<ClaimTipsArgs>());
                let args = ClaimTipsArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let tipPaymentAccount0 = keys.next().unwrap().clone();
                let tipPaymentAccount1 = keys.next().unwrap().clone();
                let tipPaymentAccount2 = keys.next().unwrap().clone();
                let tipPaymentAccount3 = keys.next().unwrap().clone();
                let tipPaymentAccount4 = keys.next().unwrap().clone();
                let tipPaymentAccount5 = keys.next().unwrap().clone();
                let tipPaymentAccount6 = keys.next().unwrap().clone();
                let tipPaymentAccount7 = keys.next().unwrap().clone();
                let tipReceiver = keys.next().unwrap().clone();
                let blockBuilder = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimTipsAccounts {
                    config,
                    tipPaymentAccount0,
                    tipPaymentAccount1,
                    tipPaymentAccount2,
                    tipPaymentAccount3,
                    tipPaymentAccount4,
                    tipPaymentAccount5,
                    tipPaymentAccount6,
                    tipPaymentAccount7,
                    tipReceiver,
                    blockBuilder,
                    signer,
                    remaining,
                };
                return Ok(Instruction::ClaimTips { accounts, args });
            }
            [69u8, 99u8, 22u8, 71u8, 11u8, 231u8, 86u8, 143u8] => {
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<ChangeTipReceiverArgs>());
                let args = ChangeTipReceiverArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let oldTipReceiver = keys.next().unwrap().clone();
                let newTipReceiver = keys.next().unwrap().clone();
                let blockBuilder = keys.next().unwrap().clone();
                let tipPaymentAccount0 = keys.next().unwrap().clone();
                let tipPaymentAccount1 = keys.next().unwrap().clone();
                let tipPaymentAccount2 = keys.next().unwrap().clone();
                let tipPaymentAccount3 = keys.next().unwrap().clone();
                let tipPaymentAccount4 = keys.next().unwrap().clone();
                let tipPaymentAccount5 = keys.next().unwrap().clone();
                let tipPaymentAccount6 = keys.next().unwrap().clone();
                let tipPaymentAccount7 = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ChangeTipReceiverAccounts {
                    config,
                    oldTipReceiver,
                    newTipReceiver,
                    blockBuilder,
                    tipPaymentAccount0,
                    tipPaymentAccount1,
                    tipPaymentAccount2,
                    tipPaymentAccount3,
                    tipPaymentAccount4,
                    tipPaymentAccount5,
                    tipPaymentAccount6,
                    tipPaymentAccount7,
                    signer,
                    remaining,
                };
                return Ok(Instruction::ChangeTipReceiver { accounts, args });
            }
            [134u8, 80u8, 38u8, 137u8, 165u8, 21u8, 114u8, 123u8] => {
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<ChangeBlockBuilderArgs>());
                let args = ChangeBlockBuilderArgs::try_from_slice(args_bytes)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let tipReceiver = keys.next().unwrap().clone();
                let oldBlockBuilder = keys.next().unwrap().clone();
                let newBlockBuilder = keys.next().unwrap().clone();
                let tipPaymentAccount0 = keys.next().unwrap().clone();
                let tipPaymentAccount1 = keys.next().unwrap().clone();
                let tipPaymentAccount2 = keys.next().unwrap().clone();
                let tipPaymentAccount3 = keys.next().unwrap().clone();
                let tipPaymentAccount4 = keys.next().unwrap().clone();
                let tipPaymentAccount5 = keys.next().unwrap().clone();
                let tipPaymentAccount6 = keys.next().unwrap().clone();
                let tipPaymentAccount7 = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ChangeBlockBuilderAccounts {
                    config,
                    tipReceiver,
                    oldBlockBuilder,
                    newBlockBuilder,
                    tipPaymentAccount0,
                    tipPaymentAccount1,
                    tipPaymentAccount2,
                    tipPaymentAccount3,
                    tipPaymentAccount4,
                    tipPaymentAccount5,
                    tipPaymentAccount6,
                    tipPaymentAccount7,
                    signer,
                    remaining,
                };
                return Ok(Instruction::ChangeBlockBuilder { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
