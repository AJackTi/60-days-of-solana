use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("6i99aodjTUm6NSkygs75JcyyDZxSrJW621bYAPyB57e1");

#[program]
pub mod day_28 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn set(ctx: Context<Set>, new_val: u32) -> Result<()> {
        ctx.accounts.pda.value = new_val;
        Ok(())
    }

    pub fn set_version_2(ctx: Context<Set>, new_val: u32) -> Result<()> {
        ctx.accounts.pda.value = new_val;
        return err!(Error::AlwaysFails);
    }
}

#[error_code]
pub enum Error {
    #[msg("Always fails")]
    AlwaysFails,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<PDA>() + 8, seeds = [], bump)]
    pub pda: Account<'info, PDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    pub pda: Account<'info, PDA>,
}

#[account]
pub struct PDA {
    pub value: u32,
}
