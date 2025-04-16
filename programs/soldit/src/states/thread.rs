use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Thread {
    pub authority: Pubkey,
    pub tid: u64,
    #[max_len(64)]
    pub title: String,
    #[max_len(642)]
    pub description: String,
    #[max_len(164)]
    pub image: String,
    pub total_votes: u64,
    pub timestamp: u64,
    pub comment_count: u64,
}
