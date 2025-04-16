use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::SolditErrors::*;
use crate::states::user::User;

pub fn initialize_user(ctx: Context<InitializeUserCtx>, username: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    if user.initialized {
        return Err(UserAlreadyInitialized.into());
    }
    user.username = username;
    user.authority = ctx.accounts.authority.key();
    user.thread_count = 0;
    user.initialized = true;
    msg!("user initialized successfully");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUserCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE+User::INIT_SPACE,
        seeds = [b"user",authority.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
