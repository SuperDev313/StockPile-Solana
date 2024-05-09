use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use klend::*;

use crate::{error::VaultError, util::KLend};

pub fn deposit(
    ctx: Context<Deposit>,
    _project_id: u64,
    amount: u64
) -> Result<()> {
    // Check to make sure the token is supported
    let cpi_ctx: CpiContext<'_, '_, '_, '_, klend::cpi::accounts::DepositReserveLiquidity<'_>> = CpiContext::new(
        ctx.accounts.kamino_program.to_account_info(), 
        cpi::accounts::DepositReserveLiquidity {
            owner: ctx.accounts.payer.to_account_info(),
            reserve: ctx.accounts.reserve.to_account_info(),
            reserve_collateral_mint: ctx.accounts.mint.to_account_info(),
            lending_market: ctx.accounts.lending_market.to_account_info(),
            lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
            reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
            user_source_liquidity: ctx.accounts.payer_token_account.to_account_info(),
            user_destination_collateral: ctx.accounts.payer_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info()
        },
    );

    cpi::deposit_reserve_liquidity(cpi_ctx, amount)
        .map_err(|_e| {
            msg!("Kamino deposit failed.");
            VaultError::WrongVaultAuthority
        })?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    project_id: u64,
)]
pub struct Deposit<'info> {
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub mint: Account<'info, token::Mint>,
    pub reserve_liquidity_supply: AccountInfo<'info>,
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub payer_token_account: Account<'info, token::TokenAccount>,
    pub kamino_program: Program<'info, KLend>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}