use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

// Rust Macro
declare_id!("BPuZx3QmrJWSkWKQ1fo1rBeuYkP56Nyt5VuYNEPdkTiK");

#[program]
pub mod reddal {
    use super::*;
    // pub fn initialize(
    //     ctx: Context<Initialize>,
    //     seed: u64,
    //     initializer_amount: u64,
    //     taker_amount: u64,
    // ) -> Result<()> {
    //     ctx.accounts
    //         .initialize_escrow(seed, &ctx.bumps, initializer_amount, taker_amount)?;
    //     ctx.accounts.deposit(initializer_amount)
    // }

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        reddal_token_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, reddal_token_amount)?;
        ctx.accounts.deposit(reddal_token_amount)
    }
    //
    // pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    //     ctx.accounts.refund_and_close_vault()
    // }
    //
    // pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
    //     ctx.accounts.deposit()?;
    //     ctx.accounts.withdraw_and_close_vault()
    // }
}