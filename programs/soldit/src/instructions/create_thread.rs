use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::SolditErrors::*;
use crate::states::thread::Thread;
use crate::states::user::User;

pub fn create_thread(
    ctx: Context<CreateThreadCtx>,
    title: String,
    description: String,
    image: String,
) -> Result<()> {
    let thread = &mut ctx.accounts.thread;
    let user = &mut ctx.accounts.user;


    if title.len() > 64 {
        return Err(TitleTooLong.into());
    }
    if description.len() > 642 {
        return Err(DescriptionTooLong.into());
    }
    if image.len() > 164 {
        return Err(ImageTooLong.into());
    }

    user.thread_count += 1;
    thread.tid = user.thread_count;
    thread.authority = ctx.accounts.authority.key();
    thread.title = title;
    thread.description = description;
    thread.image = image;
    thread.timestamp = Clock::get()?.unix_timestamp as u64;
    thread.total_votes = 0;
    thread.comment_count = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateThreadCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE+Thread::INIT_SPACE,
        seeds = [b"thread",authority.key().as_ref(),
        (user.thread_count+1).to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub thread: Account<'info, Thread>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
