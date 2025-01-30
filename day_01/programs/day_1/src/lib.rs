use anchor_lang::prelude::*;

declare_id!("Twmjhv1kkK71cbRA8koTSd2zEjzHxiZrSgwDBpz8Kwv");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize2(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
