use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User{
    pub authority:Pubkey,
    #[max_len(20)]
    pub username:String,
    pub initialized:bool,
    pub thread_count:u64,
}