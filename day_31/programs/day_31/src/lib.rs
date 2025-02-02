use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("GLoh9562fxifh5k9o4UAweW2RSxqDepNjHGehS9TSVTV");

#[program]
pub mod day_31 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn foo(ctx: Context<Foo>) -> Result<()> {
        let data = &ctx.accounts.some_account.try_borrow_data()?;
        msg!("data: {:?}", data);
        Ok(())
    }

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        let lamports = ctx.accounts.signer.lamports();
        let address = &ctx.accounts.signer.signer_key().unwrap();
        msg!("hello {:?} you have {} lamports", address, lamports);
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
pub struct MyStorage {}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account balance
    #[account(mut)]
    pub acct: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Foo<'info> {
    /// CHECK: we are just printing the data
    some_account: AccountInfo<'info>,
}

#[account]
pub struct SomeAccount {}

#[derive(Accounts)]
pub struct Hello<'info> {
    pub signer: Signer<'info>,
}
