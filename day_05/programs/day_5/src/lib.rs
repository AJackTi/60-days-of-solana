use anchor_lang::prelude::*;

declare_id!("EGmeRcY1BTCttuvK1CktC3KKUQwEvKTqrWa2y1HovUBU");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Updated message");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
