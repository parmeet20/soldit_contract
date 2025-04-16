use anchor_lang::prelude::*;

use crate::errors::SolditErrors::*;
use crate::states::thread::Thread;
use crate::states::user::User;

pub fn update_thread(
    ctx: Context<UpdateThreadCtx>,
    title: String,
    description: String,
    image: String,
) -> Result<()> {
    let thread = &mut ctx.accounts.thread;
    let user = &mut ctx.accounts.user;

    if user.authority.key() != thread.authority.key() {
        return Err(UnauthorizedAccess.into());
    }

    if title.len() > 64 {
        return Err(TitleTooLong.into());
    }
    if description.len() > 642 {
        return Err(DescriptionTooLong.into());
    }
    if image.len() > 164 {
        return Err(ImageTooLong.into());
    }
    thread.title = title;
    thread.description = description;
    thread.image = image;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateThreadCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"thread",authority.key().as_ref(),
        user.thread_count.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub thread: Account<'info, Thread>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
