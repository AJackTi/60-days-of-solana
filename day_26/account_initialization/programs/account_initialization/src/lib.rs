use anchor_lang::prelude::*;

declare_id!("4cxoZdBdPTnmYekvvv9dfkifQZ29gBVFtvM2uPwzzhNH");

#[program]
pub mod account_initialization {
    use super::*;

    pub fn initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_pda(ctx: Context<InitializePda>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeKeypair<'info> {
    #[account(init, payer = signer, space = 8)]
    keypair: Account<'info, Keypair>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePda<'info> {
    #[account(init, payer = signer, space = 8, seeds = [], bump)]
    pda: Account<'info, Pda>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Keypair();

#[account]
pub struct Pda();
