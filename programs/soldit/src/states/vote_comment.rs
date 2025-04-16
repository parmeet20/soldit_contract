use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VoteComment {
    pub authority:Pubkey,
    pub tid:u64,
    pub voted:bool,
    pub upvote:bool,
    pub initialized:bool,
}
