use anchor_lang::prelude::*;
use crate::state::pool::*;

pub fn realloc_pool(
    ctx: Context<ReallocPool>,
    _pool_id: u64,

) -> Result<()> {
    ctx.accounts.pool.to_account_info().realloc((ctx.accounts.pool.try_to_vec()?).len() + 9000, false)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    pool_id: u64,
)]
pub struct ReallocPool<'info> {
    #[account( 
        mut,
        realloc = Pool::SPACE + 9000,
        realloc::payer = payer,
        realloc::zero = false,
        seeds = [
            Pool::SEED_PREFIX.as_bytes(),
            pool_id.to_le_bytes().as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool: Box<Account<'info, Pool>>,
    pub gatekeeper_network: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}