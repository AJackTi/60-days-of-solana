use anchor_lang::prelude::*;

declare_id!("8nbEK4yrCvxv1ZyYR6cQSAFxbyb7qy8SHF19Qb74U1ZD");

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
