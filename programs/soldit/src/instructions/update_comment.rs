use anchor_lang::prelude::*;

use crate::errors::SolditErrors::*;
use crate::states::comment::Comment;
use crate::states::thread::Thread;
use crate::states::user::User;

pub fn update_comment(ctx: Context<UpdateCommentCtx>, tid: u64, cmnt: String) -> Result<()> {
    let thread = &mut ctx.accounts.thread;

    if tid != thread.tid {
        return Err(ThreadNotFound.into());
    }

    let comment = &mut ctx.accounts.comment;

    if comment.authority != ctx.accounts.comment_authority.key() {
        return Err(UnauthorizedAccess.into());
    }
    comment.comment = cmnt;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateCommentCtx<'info> {
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
        mut,
        seeds = [
            b"comment",
            thread.tid.to_le_bytes().as_ref(),
            thread.comment_count.to_le_bytes().as_ref(),
            comment_authority.key().as_ref(),
        ],
        bump,
    )]
    pub comment: Account<'info, Comment>,

    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
