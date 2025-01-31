use anchor_lang::prelude::*;

declare_id!("6mwNiZ4Coxy1ir4aXyhcfG7g7bCaaKFsXPY4UACExPUp");

#[program]
pub mod day_21 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.account.to_account_info().lamports();

        msg!("balance in Lamports: {:?}", balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don't do anything with the information
    pub account: UncheckedAccount<'info>,
}
