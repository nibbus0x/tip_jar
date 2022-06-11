use crate::constant::*;
use anchor_lang::prelude::*;

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum AccountType {
    TipJarStateV1,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::TipJarStateV1
    }
}

// Structs
pub const TIPPEE_SIZE: usize = 32 + 2;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq)]
pub struct Tippee {
    pub address: Pubkey,
    // in basis points
    pub share: u16,
}

// Accounts
pub const TIP_JAR_ACCOUNT_SIZE: usize = 8 + 1 + 32 + 2 + (1 + (1 + TIPPEE_SIZE) * MAX_TIPPEES) + 1;

#[account]
#[derive(Default)]
pub struct TipJarState {
    pub key: AccountType,
    pub owner: Pubkey,
    pub tip_percentage: u16,
    pub tippees: Option<[Option<Tippee>; 10]>,
    pub num_tippees: u8,
}
