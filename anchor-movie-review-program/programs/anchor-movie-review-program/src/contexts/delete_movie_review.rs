use anchor_lang::prelude::*;

use crate::MovieAccountState;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), initializer.key().as_ref()],
    bump,
    close = initializer
  )]
  pub movie_review: Account<'info, MovieAccountState>,
  #[account(mut)]
  pub initializer: Signer<'info>,
  pub system_program: Program<'info, System>
}