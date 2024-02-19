use anchor_lang::prelude::*;
use crate::state::*;

pub fn handle_new_advertiser(ctx: Context<NewAdvertiser>) -> Result<()> {
    let advertiser = &mut ctx.accounts.advertiser;
    advertiser.advertiser_wallet = ctx.accounts.advertiser_wallet.key();
    advertiser.bump = ctx.bumps.advertiser;
    Ok(())
}

#[derive(Accounts)]
pub struct NewAdvertiser<'info> {
    #[account(
        init, 
        payer = advertiser_wallet, 
        space = ADVERTISER_SIZE,
        seeds = [b"advertiser", advertiser_wallet.key().as_ref()],
        bump
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    pub advertiser_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}