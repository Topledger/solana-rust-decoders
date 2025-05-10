pub use accounts_data::*;
pub use ix_data::*;
#[allow(dead_code)]
use std::convert::TryInto;
use std::mem;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
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
    #[derive(Debug)]
    pub struct InitializeAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount0: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount1: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount2: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount3: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount4: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount5: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount6: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount7: anchor_lang::prelude::Pubkey,
        pub systemProgram: anchor_lang::prelude::Pubkey,
        pub payer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct ClaimTipsAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount0: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount1: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount2: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount3: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount4: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount5: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount6: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount7: anchor_lang::prelude::Pubkey,
        pub tipReceiver: anchor_lang::prelude::Pubkey,
        pub blockBuilder: anchor_lang::prelude::Pubkey,
        pub signer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct ChangeTipReceiverAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub oldTipReceiver: anchor_lang::prelude::Pubkey,
        pub newTipReceiver: anchor_lang::prelude::Pubkey,
        pub blockBuilder: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount0: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount1: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount2: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount3: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount4: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount5: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount6: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount7: anchor_lang::prelude::Pubkey,
        pub signer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct ChangeBlockBuilderAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub tipReceiver: anchor_lang::prelude::Pubkey,
        pub oldBlockBuilder: anchor_lang::prelude::Pubkey,
        pub newBlockBuilder: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount0: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount1: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount2: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount3: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount4: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount5: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount6: anchor_lang::prelude::Pubkey,
        pub tipPaymentAccount7: anchor_lang::prelude::Pubkey,
        pub signer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
}
pub mod ix_data {
    use super::*;
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeArgs {
        pub bumps: InitBumps,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct ClaimTipsArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct ChangeTipReceiverArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct ChangeBlockBuilderArgs {
        pub block_builder_commission: u64,
    }
}
#[derive(Debug)]
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
    pub fn decode(
        account_keys: &[anchor_lang::prelude::Pubkey],
        data: &[u8],
    ) -> anyhow::Result<Self> {
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
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let tipPaymentAccount0 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount0)))
                })?;
                let tipPaymentAccount1 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount1)))
                })?;
                let tipPaymentAccount2 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount2)))
                })?;
                let tipPaymentAccount3 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount3)))
                })?;
                let tipPaymentAccount4 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount4)))
                })?;
                let tipPaymentAccount5 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount5)))
                })?;
                let tipPaymentAccount6 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount6)))
                })?;
                let tipPaymentAccount7 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount7)))
                })?;
                let systemProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(systemProgram)))
                })?;
                let payer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(payer)))
                })?;
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
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let tipPaymentAccount0 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount0)))
                })?;
                let tipPaymentAccount1 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount1)))
                })?;
                let tipPaymentAccount2 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount2)))
                })?;
                let tipPaymentAccount3 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount3)))
                })?;
                let tipPaymentAccount4 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount4)))
                })?;
                let tipPaymentAccount5 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount5)))
                })?;
                let tipPaymentAccount6 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount6)))
                })?;
                let tipPaymentAccount7 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount7)))
                })?;
                let tipReceiver = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipReceiver)))
                })?;
                let blockBuilder = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(blockBuilder)))
                })?;
                let signer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(signer)))
                })?;
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
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let oldTipReceiver = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(oldTipReceiver)))
                })?;
                let newTipReceiver = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(newTipReceiver)))
                })?;
                let blockBuilder = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(blockBuilder)))
                })?;
                let tipPaymentAccount0 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount0)))
                })?;
                let tipPaymentAccount1 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount1)))
                })?;
                let tipPaymentAccount2 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount2)))
                })?;
                let tipPaymentAccount3 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount3)))
                })?;
                let tipPaymentAccount4 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount4)))
                })?;
                let tipPaymentAccount5 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount5)))
                })?;
                let tipPaymentAccount6 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount6)))
                })?;
                let tipPaymentAccount7 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount7)))
                })?;
                let signer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(signer)))
                })?;
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
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let tipReceiver = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipReceiver)))
                })?;
                let oldBlockBuilder = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(oldBlockBuilder)))
                })?;
                let newBlockBuilder = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(newBlockBuilder)))
                })?;
                let tipPaymentAccount0 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount0)))
                })?;
                let tipPaymentAccount1 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount1)))
                })?;
                let tipPaymentAccount2 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount2)))
                })?;
                let tipPaymentAccount3 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount3)))
                })?;
                let tipPaymentAccount4 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount4)))
                })?;
                let tipPaymentAccount5 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount5)))
                })?;
                let tipPaymentAccount6 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount6)))
                })?;
                let tipPaymentAccount7 = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tipPaymentAccount7)))
                })?;
                let signer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(signer)))
                })?;
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
