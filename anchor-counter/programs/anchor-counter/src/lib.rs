use anchor_lang::prelude::*;

declare_id!("91EVVU4XLk4kn8ZhrUZUq8vMWh2r8JFvtKa8KiDtK3EU");

// Instruction logic
#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter Account Created");
        msg!("Current Count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add_signed(-1).unwrap();
        msg!("Counter decrement. Current count: {}", counter.count);
        Ok(())
    }
}

// Validate incoming accountes for instructions
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>
}

// Define custom program account type
#[account]
pub struct Counter {
    pub count: u64,
}