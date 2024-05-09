use anchor_lang::prelude::*;

pub mod instructions;
pub mod util;
pub mod state;
pub mod error;

pub use instructions::*;
use crate::state::vault::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stockpile_trusts {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_id: u64, protocol: Protocols, interval: Intervals, initial_amount: u64, projects: Vec<Pubkey>, mint: Pubkey) -> Result<()> {
        instructions::init(ctx, vault_id, protocol, interval, initial_amount, projects, mint)
    }

    pub fn deposit(ctx: Context<Deposit>, project_id: u64, amount: u64) -> Result<()> {
        instructions::deposit(ctx, project_id, amount)
    }

    pub fn withdraw_and_close(ctx: Context<Withdraw>, project_id: u64, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, project_id, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
