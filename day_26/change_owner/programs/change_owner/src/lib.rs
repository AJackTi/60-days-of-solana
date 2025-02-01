use anchor_lang::prelude::*;
use core::mem::size_of;
use anchor_lang::system_program;

declare_id!("7vavnkdV4UFNtdDWEo67Caf2eQNSjy9C6BG2QAJPuL3C");

#[program]
pub mod change_owner {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn change_owner(ctx: Context<ChangeOwner>) -> Result<()> {
        let account_info = &mut ctx.accounts.my_storage.to_account_info();

        // assign is the function to transfer ownership
        account_info.assign(&system_program::ID);

        // we must erase all the data in the account or the transfer with fail
        let res = account_info.realloc(0, false); // The false means don’t zero out the data, but it makes no difference because there is no data anymore.

        if !res.is_ok() {
            return err!(Err::ReallocFailed);
        }

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

#[derive(Accounts)]
pub struct ChangeOwner<'info> {
    #[account(mut)]
    pub my_storage: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[error_code]
pub enum Err {
    #[msg("realloc failed")]
    ReallocFailed,
}
