use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Btyg9YJ5z4PY47U9Nz4KnDrWEwCqJzotXCFeKkbA3TWN");

#[program]
pub mod day_25 {
    use super::*;

    pub fn initialize_pda(ctx: Context<InitializePDA>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_keypair_account(ctx: Context<InitializeKeypairAccount>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePDA<'info> {
    // This is the program derived address
    #[account(init, payer = signer, space = size_of::<MyPDA>() + 8, seeds = [], bump)]
    pub my_pda: Account<'info, MyPDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyPDA {
    x: u64,
}

#[derive(Accounts)]
pub struct InitializeKeypairAccount<'info> {
    // This is the program derived address
    #[account(init, payer = signer, space = size_of::<MyKeypairAccount>() + 8)]
    pub my_keypair_account: Account<'info, MyKeypairAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyKeypairAccount {
    x: u64,
}
