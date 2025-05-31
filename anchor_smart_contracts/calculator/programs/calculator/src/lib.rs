use anchor_lang::prelude::*;

declare_id!("9dbjM2HrhidE7t1t3337iDBd7ucvBU9DGFGCf8Pc3jbA");

#[program]
pub mod calculator {
    use super::*;

    pub fn init(ctx: Context<Initialize>, init_value: u32) -> Result<()> {
        ctx.accounts.account.num = init_value;
        Ok(())
    }

    pub fn add(ctx: Context<Add>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num + num;
        Ok(())
    }

    pub fn double(ctx: Context<Double>, num: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * 2;
        Ok(())
    }

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
}

#[account]
struct DataShape {
    num: u32,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // initialize the DataAccount with DatShape type
    //
    #[account(init, payer=signer,space=8+4)]
    pub account: Account<'info, DataShape>,
    // here we are telling System program to create our calculator smartcontator
    pub system_program: Program<'info, System>,
    // signer of the contractor the real account that deploy the contact on chain
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
