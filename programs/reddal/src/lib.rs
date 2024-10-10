// Import statements
use anchor_lang::prelude::*;

// Rust Macro
declare_id!("BPuZx3QmrJWSkWKQ1fo1rBeuYkP56Nyt5VuYNEPdkTiK");

#[program]
pub mod reddal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


// Type declarations
#[derive(Accounts)]
pub struct Initialize {}
