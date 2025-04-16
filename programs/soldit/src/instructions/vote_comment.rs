use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::SolditErrors::*;
use crate::states::comment::Comment;
use crate::states::thread::Thread;
use crate::states::user::User;
use crate::states::vote_comment::VoteComment;

pub fn vote_comment(ctx: Context<VoteCommentCtx>, tid: u64, vote_type: bool) -> Result<()> {
    let thread = &mut ctx.accounts.thread;
    let comment = &mut ctx.accounts.comment;
    let vote = &mut ctx.accounts.vote_comment;

    // Ensure the thread ID matches
    if tid != thread.tid {
        return Err(ThreadNotFound.into());
    }

    // If the vote hasn't been initialized yet, initialize the vote
    if !vote.initialized {
        vote.initialized = true;
        vote.authority = ctx.accounts.voter.key();
        vote.tid = thread.tid;
        vote.voted = true;
        vote.upvote = vote_type;

        // Update the upvote or downvote count on the comment
        if vote_type {
            comment.upvotes_count += 1;
        } else {
            comment.downvotes_count += 1;
        }
    } else {
        // If the user has already voted, check their current state
        if vote.voted {
            // If the user is toggling their vote (upvote -> downvote or vice versa)
            if vote.upvote == vote_type {
                // Same vote type: cancel the vote
                if vote_type {
                    comment.upvotes_count -= 1;
                } else {
                    comment.downvotes_count -= 1;
                }
                vote.voted = false;
            } else {
                // Different vote type: update counts
                if vote.upvote {
                    comment.upvotes_count -= 1;
                } else {
                    comment.downvotes_count -= 1;
                }

                // Apply the new vote
                if vote_type {
                    comment.upvotes_count += 1;
                } else {
                    comment.downvotes_count += 1;
                }

                vote.upvote = vote_type;
            }
        } else {
            // User is voting again after canceling previous vote
            vote.voted = true;
            vote.upvote = vote_type;

            if vote_type {
                comment.upvotes_count += 1;
            } else {
                comment.downvotes_count += 1;
            }
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct VoteCommentCtx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub comment_authority: Signer<'info>,
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        mut,
        seeds = [b"thread", authority.key().as_ref(), user.thread_count.to_le_bytes().as_ref()],
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
    #[account(
        init_if_needed,
        payer = voter,
        space = ANCHOR_DISCRIMINATOR_SIZE + VoteComment::INIT_SPACE,
        seeds = [
            b"comment_vote",
            comment.key().as_ref(),
            voter.key().as_ref()
        ],
        bump,
    )]
    pub vote_comment: Account<'info, VoteComment>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}