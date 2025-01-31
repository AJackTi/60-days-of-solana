use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("orMJEWHq2uxYZfzRvyZF7cbkGKnQdfjSza4Ej3cEr5S");

#[program]
pub mod day_19 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

    pub fn initialize_nested_mapping(
        _ctx: Context<InitializeNestedMapping>,
        key1: u64,
        key2: u64
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_nested_mapping(
        ctx: Context<SetNestedMapping>,
        key1: u64,
        key2: u64,
        val: u64
    ) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

    pub fn initialize_map1(ctx: Context<InitializeMap1>, which_map: u64, key1: u64) -> Result<()> {
        ctx.accounts.val1.value = 0;
        Ok(())
    }

    pub fn set_map1(ctx: Context<SetMap1>, which_map: u64, key1: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

    pub fn initialize_map2(
        ctx: Context<InitializeMap2>,
        which_map: u64,
        key1: u64,
        key2: u64
    ) -> Result<()> {
        ctx.accounts.val2.value = 0;
        Ok(())
    }

    pub fn set_map2(
        ctx: Context<SetMap2>,
        which_map: u64,
        key1: u64,
        key2: u64,
        val: u64
    ) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds = [&key.to_le_bytes().as_ref()],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Set<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct InitializeNestedMapping<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds = [&key1.to_le_bytes().as_ref(), &key2.to_le_bytes().as_ref()],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct SetNestedMapping<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

#[account]
pub struct Val1 {
    value: u64,
}

#[account]
pub struct Val2 {
    value: u64,
}

#[derive(Accounts)]
#[instruction(which_map: u64, key1: u64)]
pub struct InitializeMap1<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val1>() + 8,
        seeds = [&which_map.to_le_bytes().as_ref(), &key1.to_le_bytes().as_ref()],
        bump
    )]
    val1: Account<'info, Val1>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(which_map: u64, key1: u64)]
pub struct SetMap1<'info> {
    #[account(mut)]
    val: Account<'info, Val1>,
}

#[derive(Accounts)]
#[instruction(which_map: u64, key1: u64, key2: u64)]
pub struct InitializeMap2<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val2>() + 8,
        seeds = [
            &which_map.to_le_bytes().as_ref(),
            &key1.to_le_bytes().as_ref(),
            &key2.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub val2: Account<'info, Val2>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(which_map: u64, key1: u64, key2: u64)]
pub struct SetMap2<'info> {
    #[account(mut)]
    val: Account<'info, Val2>,
}
