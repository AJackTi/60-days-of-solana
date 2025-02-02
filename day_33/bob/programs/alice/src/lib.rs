use anchor_lang::prelude::*;
// account struct for add_and_storeuse bob::cpi::accounts::BobAddOp;

// The program definition for Bob
use bob::program::Bob;

// the account where Bob is storing the sum
use bob::BobData;
use bob::cpi::accounts::BobAddOp;

declare_id!("6jHZ7WW4gcuhBXAL86WVvtSrWnjkYsVusGgH3z4pxayP");

#[program]
pub mod alice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn ask_bob_to_add(ctx: Context<AliceOp>, a: u64, b: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(ctx.accounts.bob_program.to_account_info(), BobAddOp {
            bob_data_account: ctx.accounts.bob_data_account.to_account_info(),
        });

        let res = bob::cpi::add_and_store(cpi_ctx, a, b);

        // return an error if the CPI failed
        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::CPIToBobFailed);
        }
    }

    pub fn ask_bob_to_add_2(ctx: Context<AliceOp>, a: u64, b: u64) -> Result<()> {
        // Calls the `bob_add_operation` function in bob program
        let res = bob::cpi::bob_add_operation(ctx.accounts.add_function_ctx(), a, b);

        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::CPIToBobFailed);
        }
    }
}

impl<'info> AliceOp<'info> {
    pub fn add_function_ctx(&self) -> CpiContext<'_, '_, '_, 'info, BobAddOp<'info>> {
        // The bob program we are interacting with
        let cpi_program = self.bob_program.to_account_info();

        // Passing the necessary account(s) to the `BobAddOp` account struct in Bob program
        let cpi_account = BobAddOp {
            bob_data_account: self.bob_data_account.to_account_info(),
        };

        // Creates a `CpiContext` object using the new method
        CpiContext::new(cpi_program, cpi_account)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum Errors {
    #[msg("cpi to bob failed")]
    CPIToBobFailed,
}

#[derive(Accounts)]
pub struct AliceOp<'info> {
    #[account(mut)]
    pub bob_data_account: Account<'info, BobData>,

    pub bob_program: Program<'info, Bob>,
}
