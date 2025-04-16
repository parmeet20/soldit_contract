use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::SolditErrors::*;
use crate::states::thread::Thread;
use crate::states::upvote_thread::UpvoteThread;
use crate::states::user::User;

pub fn upvote_thread(ctx: Context<UpvoteThreadCtx>, tid: u64) -> Result<()> {
    let thread = &mut ctx.accounts.thread;
    let upvote = &mut ctx.accounts.upvote;
    if tid != thread.tid {
        return Err(ThreadNotFound.into());
    }

    if upvote.voted {
        return Err(AlreadyVoted.into());
    }
    upvote.authority = ctx.accounts.voter.key();

    upvote.voted = true;

    thread.total_votes += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct UpvoteThreadCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        mut,
        seeds = [b"thread",authority.key().as_ref(),
        user.thread_count.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub thread: Account<'info, Thread>,
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + UpvoteThread::INIT_SPACE,
        seeds = [b"upvote_thread",thread.tid.to_le_bytes().as_ref(),voter.key().as_ref()],
        bump,
    )]
    pub upvote: Account<'info, UpvoteThread>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
