use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("6BDnoXdxM8oPpCLVFt7w75yexLYinnkQ3aij9L8AhXpZ");

#[program]
pub mod soldit {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUserCtx>, username: String) -> Result<()> {
        instructions::initialize_user(ctx, username)
    }
    pub fn create_thread(
        ctx: Context<CreateThreadCtx>,
        title: String,
        description: String,
        image: String,
    ) -> Result<()> {
        instructions::create_thread(ctx, title, description, image)
    }
    pub fn update_thread(
        ctx: Context<UpdateThreadCtx>,
        title: String,
        description: String,
        image: String,
    ) -> Result<()> {
        instructions::update_thread(ctx, title, description, image)
    }
    pub fn upvote_thread(ctx: Context<UpvoteThreadCtx>, tid: u64) -> Result<()> {
        instructions::upvote_thread(ctx, tid)
    }
    pub fn create_comment(ctx: Context<CreateCommentCtx>,tid:u64, cmnt: String) -> Result<()> {
        instructions::create_comment(ctx, tid, cmnt)
    }
    pub fn update_comment(ctx: Context<UpdateCommentCtx>, tid: u64, cmnt: String) -> Result<()> {
        instructions::update_comment(ctx, tid, cmnt)
    }
    pub fn vote_comment(ctx: Context<VoteCommentCtx>, tid: u64, vote_type: bool) -> Result<()> {
        instructions::vote_comment(ctx, tid, vote_type)
    }
}
