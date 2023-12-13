use anchor_lang::prelude::*;

use crate::MovieAccountState;

#[derive(Accounts)]
#[instruction(title:String, description:String)]
pub struct UpdateMoviewReview<'info> {
  #[account(
    mut,
    seeds = [title.as_bytes(), initializer.key().as_ref()],
    bump,
    realloc = 8 + 32 + 1 + 4 + title.len() + 4 + description.len(),
    realloc::payer = initializer,
    realloc::zero = true
  )]
  pub movie_review: Account<'info, MovieAccountState>,
  #[account(mut)]
  pub initializer: Signer<'info>,
  pub system_program: Program<'info, System>
}