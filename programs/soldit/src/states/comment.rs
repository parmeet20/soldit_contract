use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Comment {
    pub authority: Pubkey,
    pub tid: u64,
    #[max_len(264)]
    pub comment:String,
    pub upvotes_count:u64,
    pub downvotes_count:u64,
    pub timestamp:u64,
}
