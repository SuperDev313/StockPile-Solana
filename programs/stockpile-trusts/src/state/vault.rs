use anchor_lang::prelude::*;

#[account]
pub struct YieldVault {
    pub protocol: Protocols,
    pub interval: Intervals,
    pub initial_amount: u64,
    pub vault_id: u64,
    pub projects: Vec<Pubkey>,
    pub mint: Pubkey,
    pub bump: u8,
}

impl YieldVault {
    pub const SEED_PREFIX: &'static str = "yield_vault";

    pub const SPACE: usize = 8 
        + 4                         // u64
        + 4                         // String
        + 4                         // u64
        + 4                         // u64
        + 4                         // u64
        + 1                         // u8
        + 160                       // Vec<Pubkey> (Max 5)
        + 32                        // Pubkey
        + 1                         // u8
        + 4                         // Enum (Singleton)
        + 250;                      // Padding

    pub fn new(protocol: Protocols, interval: Intervals, initial_amount: u64, vault_id: u64, projects: Vec<Pubkey>, mint: Pubkey, bump: u8) -> Result<Self> {
        Ok(Self {
            protocol,
            interval,
            initial_amount,
            vault_id,
            projects,
            mint,
            bump
        })
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Protocols {
    Kamino,
}

impl Default for Protocols {
    fn default() -> Self {
        Protocols::Kamino
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Intervals {
    Weekly,
    Monthly
}

impl Default for Intervals {
    fn default() -> Self {
        Intervals::Weekly
    }
}