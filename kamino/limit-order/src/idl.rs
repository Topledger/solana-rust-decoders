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
    serde_big_array::big_array! { BigArray ; 64 , 51 , 128 , 72 , 256 }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderStatus {
        Active,
        Filled,
        Cancelled,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderType {
        Vanilla,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateGlobalConfigMode {
        UpdateEmergencyMode,
        UpdateFlashTakeOrderBlocked,
        UpdateBlockNewOrders,
        UpdateBlockOrderTaking,
        UpdateHostFeeBps,
        UpdateAdminAuthorityCached,
        UpdateOrderTakingPermissionless,
        UpdateOrderCloseDelaySeconds,
        UpdateTxnFeeCost,
        UpdateAtaCreationCost,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateGlobalConfigValue {
        Bool(bool),
        U16(u16),
        U64(u64),
        Pubkey([u8; 32usize]),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateOrderMode {
        UpdatePermissionless,
        UpdateCounterparty,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeGlobalConfigAccounts {
        pub adminAuthority: String,
        pub pdaAuthority: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeVaultAccounts {
        pub payer: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub mint: String,
        pub vault: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOrderAccounts {
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub makerAta: String,
        pub inputVault: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateOrderAccounts {
        pub maker: String,
        pub globalConfig: String,
        pub order: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseOrderAndClaimTipAccounts {
        pub maker: String,
        pub order: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub inputMint: String,
        pub outputMint: String,
        pub makerInputAta: String,
        pub inputVault: String,
        pub inputTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TakeOrderAccounts {
        pub taker: String,
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputVault: String,
        pub takerInputAta: String,
        pub takerOutputAta: String,
        pub intermediaryOutputTokenAccount: String,
        pub makerOutputAta: String,
        pub expressRelay: String,
        pub expressRelayMetadata: String,
        pub sysvarInstructions: String,
        pub permission: String,
        pub configRouter: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        pub rent: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashTakeOrderStartAccounts {
        pub taker: String,
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputVault: String,
        pub takerInputAta: String,
        pub takerOutputAta: String,
        pub intermediaryOutputTokenAccount: String,
        pub makerOutputAta: String,
        pub expressRelay: String,
        pub expressRelayMetadata: String,
        pub sysvarInstructions: String,
        pub permission: String,
        pub configRouter: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashTakeOrderEndAccounts {
        pub taker: String,
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputVault: String,
        pub takerInputAta: String,
        pub takerOutputAta: String,
        pub intermediaryOutputTokenAccount: String,
        pub makerOutputAta: String,
        pub expressRelay: String,
        pub expressRelayMetadata: String,
        pub sysvarInstructions: String,
        pub permission: String,
        pub configRouter: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminAccounts {
        pub adminAuthorityCached: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawHostTipAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LogUserSwapBalancesStartAccounts {
        pub maker: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputTa: String,
        pub outputTa: String,
        pub pdaReferrer: String,
        pub swapProgramId: String,
        pub userSwapBalanceState: String,
        pub systemProgram: String,
        pub rent: String,
        pub sysvarInstructions: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LogUserSwapBalancesEndAccounts {
        pub maker: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputTa: String,
        pub outputTa: String,
        pub pdaReferrer: String,
        pub swapProgramId: String,
        pub userSwapBalanceState: String,
        pub systemProgram: String,
        pub rent: String,
        pub sysvarInstructions: String,
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
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeGlobalConfigArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeVaultArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOrderArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub output_amount: u64,
        pub order_type: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateOrderArguments {
        pub mode: UpdateOrderMode,
        pub value: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseOrderAndClaimTipArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TakeOrderArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_output_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tip_amount_permissionless_taking: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashTakeOrderStartArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_output_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tip_amount_permissionless_taking: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashTakeOrderEndArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_output_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tip_amount_permissionless_taking: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigArguments {
        pub mode: u16,
        #[serde(with = "BigArray")]
        pub value: [u8; 128usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawHostTipArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LogUserSwapBalancesStartArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LogUserSwapBalancesEndArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_swap_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_ts: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_amount_out_next_best: u64,
        pub aggregator: u8,
        pub next_best_aggregator: u8,
        pub padding: [u8; 2usize],
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeGlobalConfig {
        accounts: InitializeGlobalConfigAccounts,
        args: InitializeGlobalConfigArguments,
    },
    InitializeVault {
        accounts: InitializeVaultAccounts,
        args: InitializeVaultArguments,
    },
    CreateOrder {
        accounts: CreateOrderAccounts,
        args: CreateOrderArguments,
    },
    UpdateOrder {
        accounts: UpdateOrderAccounts,
        args: UpdateOrderArguments,
    },
    CloseOrderAndClaimTip {
        accounts: CloseOrderAndClaimTipAccounts,
        args: CloseOrderAndClaimTipArguments,
    },
    TakeOrder {
        accounts: TakeOrderAccounts,
        args: TakeOrderArguments,
    },
    FlashTakeOrderStart {
        accounts: FlashTakeOrderStartAccounts,
        args: FlashTakeOrderStartArguments,
    },
    FlashTakeOrderEnd {
        accounts: FlashTakeOrderEndAccounts,
        args: FlashTakeOrderEndArguments,
    },
    UpdateGlobalConfig {
        accounts: UpdateGlobalConfigAccounts,
        args: UpdateGlobalConfigArguments,
    },
    UpdateGlobalConfigAdmin {
        accounts: UpdateGlobalConfigAdminAccounts,
        args: UpdateGlobalConfigAdminArguments,
    },
    WithdrawHostTip {
        accounts: WithdrawHostTipAccounts,
        args: WithdrawHostTipArguments,
    },
    LogUserSwapBalancesStart {
        accounts: LogUserSwapBalancesStartAccounts,
        args: LogUserSwapBalancesStartArguments,
    },
    LogUserSwapBalancesEnd {
        accounts: LogUserSwapBalancesEndAccounts,
        args: LogUserSwapBalancesEndArguments,
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
                let args = InitializeGlobalConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let adminAuthority = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGlobalConfigAccounts {
                    adminAuthority,
                    pdaAuthority,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::InitializeGlobalConfig { accounts, args });
            }
            [48u8, 191u8, 163u8, 44u8, 71u8, 129u8, 63u8, 164u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeVaultArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let vault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeVaultAccounts {
                    payer,
                    globalConfig,
                    pdaAuthority,
                    mint,
                    vault,
                    tokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeVault { accounts, args });
            }
            [141u8, 54u8, 37u8, 207u8, 237u8, 210u8, 250u8, 215u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let makerAta = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateOrderAccounts {
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    makerAta,
                    inputVault,
                    inputTokenProgram,
                    outputTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CreateOrder { accounts, args });
            }
            [54u8, 8u8, 208u8, 207u8, 34u8, 134u8, 239u8, 168u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateOrderAccounts {
                    maker,
                    globalConfig,
                    order,
                    remaining,
                };
                return Ok(Instruction::UpdateOrder { accounts, args });
            }
            [244u8, 27u8, 12u8, 226u8, 45u8, 247u8, 230u8, 43u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseOrderAndClaimTipArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let maker = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let makerInputAta = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseOrderAndClaimTipAccounts {
                    maker,
                    order,
                    globalConfig,
                    pdaAuthority,
                    inputMint,
                    outputMint,
                    makerInputAta,
                    inputVault,
                    inputTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CloseOrderAndClaimTip { accounts, args });
            }
            [163u8, 208u8, 20u8, 172u8, 223u8, 65u8, 255u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let args = TakeOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let taker = keys.next().unwrap().clone();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let takerInputAta = keys.next().unwrap().clone();
                let takerOutputAta = keys.next().unwrap().clone();
                let intermediaryOutputTokenAccount = keys.next().unwrap().clone();
                let makerOutputAta = keys.next().unwrap().clone();
                let expressRelay = keys.next().unwrap().clone();
                let expressRelayMetadata = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let permission = keys.next().unwrap().clone();
                let configRouter = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TakeOrderAccounts {
                    taker,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    inputVault,
                    takerInputAta,
                    takerOutputAta,
                    intermediaryOutputTokenAccount,
                    makerOutputAta,
                    expressRelay,
                    expressRelayMetadata,
                    sysvarInstructions,
                    permission,
                    configRouter,
                    inputTokenProgram,
                    outputTokenProgram,
                    rent,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::TakeOrder { accounts, args });
            }
            [126u8, 53u8, 176u8, 15u8, 39u8, 103u8, 97u8, 243u8] => {
                let mut rdr: &[u8] = rest;
                let args = FlashTakeOrderStartArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let taker = keys.next().unwrap().clone();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let takerInputAta = keys.next().unwrap().clone();
                let takerOutputAta = keys.next().unwrap().clone();
                let intermediaryOutputTokenAccount = keys.next().unwrap().clone();
                let makerOutputAta = keys.next().unwrap().clone();
                let expressRelay = keys.next().unwrap().clone();
                let expressRelayMetadata = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let permission = keys.next().unwrap().clone();
                let configRouter = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FlashTakeOrderStartAccounts {
                    taker,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    inputVault,
                    takerInputAta,
                    takerOutputAta,
                    intermediaryOutputTokenAccount,
                    makerOutputAta,
                    expressRelay,
                    expressRelayMetadata,
                    sysvarInstructions,
                    permission,
                    configRouter,
                    inputTokenProgram,
                    outputTokenProgram,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::FlashTakeOrderStart { accounts, args });
            }
            [206u8, 242u8, 215u8, 187u8, 134u8, 33u8, 224u8, 148u8] => {
                let mut rdr: &[u8] = rest;
                let args = FlashTakeOrderEndArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let taker = keys.next().unwrap().clone();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let takerInputAta = keys.next().unwrap().clone();
                let takerOutputAta = keys.next().unwrap().clone();
                let intermediaryOutputTokenAccount = keys.next().unwrap().clone();
                let makerOutputAta = keys.next().unwrap().clone();
                let expressRelay = keys.next().unwrap().clone();
                let expressRelayMetadata = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let permission = keys.next().unwrap().clone();
                let configRouter = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FlashTakeOrderEndAccounts {
                    taker,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    inputVault,
                    takerInputAta,
                    takerOutputAta,
                    intermediaryOutputTokenAccount,
                    makerOutputAta,
                    expressRelay,
                    expressRelayMetadata,
                    sysvarInstructions,
                    permission,
                    configRouter,
                    inputTokenProgram,
                    outputTokenProgram,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::FlashTakeOrderEnd { accounts, args });
            }
            [164u8, 84u8, 130u8, 189u8, 111u8, 58u8, 250u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAccounts {
                    adminAuthority,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateGlobalConfig { accounts, args });
            }
            [184u8, 87u8, 23u8, 193u8, 156u8, 238u8, 175u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigAdminArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let adminAuthorityCached = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAdminAccounts {
                    adminAuthorityCached,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateGlobalConfigAdmin { accounts, args });
            }
            [140u8, 246u8, 105u8, 165u8, 80u8, 85u8, 143u8, 18u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawHostTipArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawHostTipAccounts {
                    adminAuthority,
                    globalConfig,
                    pdaAuthority,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawHostTip { accounts, args });
            }
            [133u8, 108u8, 23u8, 15u8, 226u8, 215u8, 176u8, 95u8] => {
                let mut rdr: &[u8] = rest;
                let args = LogUserSwapBalancesStartArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let maker = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let pdaReferrer = keys.next().unwrap().clone();
                let swapProgramId = keys.next().unwrap().clone();
                let userSwapBalanceState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LogUserSwapBalancesStartAccounts {
                    maker,
                    inputMint,
                    outputMint,
                    inputTa,
                    outputTa,
                    pdaReferrer,
                    swapProgramId,
                    userSwapBalanceState,
                    systemProgram,
                    rent,
                    sysvarInstructions,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::LogUserSwapBalancesStart { accounts, args });
            }
            [140u8, 42u8, 198u8, 82u8, 147u8, 144u8, 44u8, 113u8] => {
                let mut rdr: &[u8] = rest;
                let args = LogUserSwapBalancesEndArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let maker = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let pdaReferrer = keys.next().unwrap().clone();
                let swapProgramId = keys.next().unwrap().clone();
                let userSwapBalanceState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LogUserSwapBalancesEndAccounts {
                    maker,
                    inputMint,
                    outputMint,
                    inputTa,
                    outputTa,
                    pdaReferrer,
                    swapProgramId,
                    userSwapBalanceState,
                    systemProgram,
                    rent,
                    sysvarInstructions,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::LogUserSwapBalancesEnd { accounts, args });
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
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OrderDisplay {
        pub initial_input_amount: u64,
        pub expected_output_amount: u64,
        pub remaining_input_amount: u64,
        pub filled_output_amount: u64,
        pub tip_amount: u64,
        pub number_of_fills: u64,
        pub on_event_output_amount_filled: u64,
        pub on_event_tip_amount: u64,
        pub order_type: u8,
        pub status: u8,
        pub last_updated_timestamp: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UserSwapBalanceDiffs {
        pub user_lamports_before: u64,
        pub input_ta_balance_before: u64,
        pub output_ta_balance_before: u64,
        pub user_lamports_after: u64,
        pub input_ta_balance_after: u64,
        pub output_ta_balance_after: u64,
        #[serde(with = "pubkey_serde")]
        pub swap_program: [u8; 32usize],
        pub simulated_swap_amount_out: u64,
        pub simulated_ts: u64,
        pub minimum_amount_out: u64,
        pub swap_amount_in: u64,
        pub simulated_amount_out_next_best: u64,
        pub aggregator: u8,
        pub next_best_aggregator: u8,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        OrderDisplay { args: OrderDisplay },
        UserSwapBalanceDiffs { args: UserSwapBalanceDiffs },
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
                [92u8, 101u8, 6u8, 158u8, 248u8, 152u8, 241u8, 60u8] => {
                    let mut rdr = &payload[..];
                    let args = OrderDisplay::deserialize(&mut rdr)?;
                    return Ok(Event::OrderDisplay { args });
                }
                [139u8, 203u8, 35u8, 31u8, 25u8, 8u8, 62u8, 143u8] => {
                    let mut rdr = &payload[..];
                    let args = UserSwapBalanceDiffs::deserialize(&mut rdr)?;
                    return Ok(Event::UserSwapBalanceDiffs { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
