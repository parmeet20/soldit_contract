use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UpvoteThread {
    pub authority: Pubkey,
    pub voted: bool,
    pub tid: u64,
}
