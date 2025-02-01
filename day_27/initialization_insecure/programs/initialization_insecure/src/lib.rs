use anchor_lang::prelude::*;
use borsh::{ BorshDeserialize, BorshSerialize };

declare_id!("BbLHbr2L494H57b1a3yobSBedfkgnC3SHpZKHKhqT9Ez");

#[program]
pub mod initialization_insecure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        user.authority = ctx.accounts.authority.key();
        user.serialize(&mut *ctx.accounts.user.data.borrow_mut())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    user: AccountInfo<'info>,
    #[account(mut)]
    authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    authority: Pubkey,
}
