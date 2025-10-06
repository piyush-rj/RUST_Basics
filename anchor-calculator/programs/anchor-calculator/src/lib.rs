use anchor_lang::prelude::*;

declare_id!("2YXW2SLUCsP5G5iLEPUrSfWbh2TFyNAW8H1LcEVXxHN9");

#[program]
pub mod anchor_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_account.data = 1;
        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.account.data = ctx.accounts.account.data * 2;
        Ok(())
    }

    pub fn add(ctx: Context<Add>, number: u32) -> Result<()> {
        ctx.accounts.account.data = ctx.accounts.account.data + number;
        Ok(())
    }

    pub fn subtract(ctx: Context<Sub>, number: u32) -> Result<()> {
        ctx.accounts.account.data = ctx.accounts.account.data - number;
        Ok(())
    }
}

#[account]
pub struct CalculatorAccount {
    data: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4, 
    )]
    pub new_account: Account<'info, CalculatorAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, CalculatorAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Half<'info> {
    #[account(mut)]
    pub account: Account<'info, CalculatorAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, CalculatorAccount>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub account: Account<'info, CalculatorAccount>,
    pub signer: Signer<'info>,
}
