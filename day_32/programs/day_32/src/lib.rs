use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Az5pB3a221tJGy5vtDCwL67VdoEiZ5Lp4aWqsqaMLDn9");

#[program]
pub mod day_32 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.storage.x = 42;
        Ok(())
    }

    pub fn read_other_data(ctx: Context<ReadOtherData>) -> Result<()> {
        let data_account = &ctx.accounts.other_data;

        if data_account.data_is_empty() {
            return err!(MyError::NoData);
        }

        let mut data_slice: &[u8] = &data_account.data.borrow();

        let data_struct: Storage = AccountDeserialize::try_deserialize(&mut data_slice)?;

        msg!("The value of x is: {}", data_struct.x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<Storage>() + 8, seeds = [], bump)]
    pub storage: Account<'info, Storage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum MyError {
    #[msg("No data")]
    NoData,
}

#[derive(Accounts)]
pub struct ReadOtherData<'info> {
    /// CHECK: We do not own this account so
    // we must be very cautious with how we
    // use the data
    other_data: UncheckedAccount<'info>,
}

#[account]
pub struct Storage {
    x: u64,
}
