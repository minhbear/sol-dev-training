use anchor_lang::prelude::*;


#[account]
pub struct MovieAccountState {
  pub reviewer: Pubkey,       // 32
  pub rating: u8,             // 1
  pub title: String,          // 4 + len()
  pub description: String     // 4 + len()
}