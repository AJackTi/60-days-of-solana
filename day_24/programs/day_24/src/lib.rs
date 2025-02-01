use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("DdE5E3yBMLskKbJvBznmztfwu5oximGL1MQebLHHixR");

const STARTING_POINTS: u32 = 10;

#[program]
pub mod day_24 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn update_value(ctx: Context<UpdateValue>, new_value: u64) -> Result<()> {
        ctx.accounts.my_storage.x = new_value;
        Ok(())
    }

    pub fn initialize2(ctx: Context<Initialize2>) -> Result<()> {
        ctx.accounts.player.points = STARTING_POINTS;
        ctx.accounts.player.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn transfer_points(ctx: Context<TransferPoints>, amount: u32) -> Result<()> {
        require!(
            ctx.accounts.from.authority == ctx.accounts.authority.key(),
            Errors::SignerIsNotAuthority
        );
        require!(ctx.accounts.from.points >= amount, Errors::InsufficientPoints);

        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<MyStorage>() + 8, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

    // THIS FIELD MUST BE INCLUDED
    #[account(signer)]
    pub signer: Signer<'info>,
}

#[error_code]
pub enum Errors {
    #[msg("SignerIsNotAuthority")]
    SignerIsNotAuthority,
    #[msg("InsufficientPoints")]
    InsufficientPoints,
}

#[derive(Accounts)]
pub struct Initialize2<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Player>() + 8,
        seeds = [&signer.as_ref().key().to_bytes()],
        bump
    )]
    player: Account<'info, Player>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u32)] // amount must be passed as an instruction
pub struct TransferPoints<'info> {
    #[account(mut, has_one = authority @ Errors::SignerIsNotAuthority, constraint = from.points >= amount @ Errors::InsufficientPoints)]
    from: Account<'info, Player>,

    #[account(mut)]
    to: Account<'info, Player>,

    #[account(mut)]
    authority: Signer<'info>,
}

#[account]
pub struct Player {
    points: u32,
    authority: Pubkey,
}
