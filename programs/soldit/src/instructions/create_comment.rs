use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::SolditErrors::*;
use crate::states::comment::Comment;
use crate::states::thread::Thread;
use crate::states::user::User;

pub fn create_comment(ctx: Context<CreateCommentCtx>,tid:u64, cmnt: String) -> Result<()> {
    let thread = &mut ctx.accounts.thread;

    if tid != thread.tid{
        return Err(ThreadNotFound.into());
    }

    let comment = &mut ctx.accounts.comment;

    
    comment.authority = ctx.accounts.comment_authority.key();
    thread.comment_count += 1;
    comment.tid = thread.tid;
    comment.comment = cmnt;
    comment.upvotes_count = 0;
    comment.downvotes_count = 0;
    comment.timestamp = Clock::get()?.unix_timestamp as u64;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateCommentCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub comment_authority: Signer<'info>,
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
        payer = comment_authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Comment::INIT_SPACE,
        seeds = [
            b"comment",
            thread.tid.to_le_bytes().as_ref(),
            (thread.comment_count+1).to_le_bytes().as_ref(),
            comment_authority.key().as_ref(),
        ],
        bump,
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
