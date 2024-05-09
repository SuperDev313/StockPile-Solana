use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};
use stockpile_v2::{cpi::*, state::project::Project};
use klend::*;

use crate::{error::VaultError, util::{KLend, Stockpile}, YieldVault};

pub fn withdraw(
    ctx: Context<Withdraw>,
    _project_id: u64,
    amount: u64
) -> Result<()> {
    let project_accounts = ctx.remaining_accounts;
    // Create Kamino CPI context
    let cpi_ctx: CpiContext<'_, '_, '_, '_, klend::cpi::accounts::RedeemReserveCollateral<'_>> = CpiContext::new(
        ctx.accounts.kamino_program.to_account_info(), 
        cpi::accounts::RedeemReserveCollateral {
            owner: ctx.accounts.payer.to_account_info(),
            reserve: ctx.accounts.reserve.to_account_info(),
            reserve_collateral_mint: ctx.accounts.mint.to_account_info(),
            lending_market: ctx.accounts.lending_market.to_account_info(),
            lending_market_authority: ctx.accounts.lending_market_authority.to_account_info(),
            reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
            user_source_collateral: ctx.accounts.payer_token_account.to_account_info(),
            user_destination_liquidity: ctx.accounts.payer_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info()
        },
    );

    // Redeem Kamino funds
    cpi::redeem_reserve_collateral(cpi_ctx, amount)
        .map_err(|_e| {
            msg!("Kamino redemption failed.");
            VaultError::WrongVaultAuthority
        })?;

    // TO-DO: Convert the below into a loop that runs through
    // the "project_accounts", validates that they are indeed 
    // project accounts, creates CPI context, invokes, and maybe
    // enforces a max account limit so it doesn't overrun the 1232 limit.

    // Create Stockpile CPI context
    let contribute_ctx: CpiContext<'_, '_, '_, '_, stockpile_v2::cpi::accounts::Contribute<'_>> = CpiContext::new(
        ctx.accounts.stockpile_program.to_account_info(),
        stockpile_v2::cpi::accounts::Contribute {
            project: ctx.accounts.project.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            project_token_account: ctx.accounts.project_token_account.to_account_info(),
            payer_token_account: ctx.accounts.payer_token_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info()
        }
    );

    // Contribute to Stockpile project
    contribute(contribute_ctx, _project_id, amount)
        .map_err(|_e| {
            msg!("Kamino redemption failed.");
            VaultError::ProjectAccountInvalid
        })?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    project_id: u64,
)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, YieldVault>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub project_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub lending_market: AccountInfo<'info>,
    #[account(mut)]
    pub lending_market_authority: AccountInfo<'info>,
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = payer_token_account.owner == payer.key()
    )]
    pub payer_token_account: Account<'info, token::TokenAccount>,
    pub stockpile_program: Program<'info, Stockpile>,
    pub kamino_program: Program<'info, KLend>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}