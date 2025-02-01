use anchor_lang::prelude::*;

declare_id!("CzzSG7kQkVvn9RYcjQuexaatWJ3B12XGXNFHECdrhL17");

#[program]
pub mod crowfund {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
