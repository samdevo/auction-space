use anchor_lang::prelude::*;
const MAX_URL_LENGTH: usize = 16;
use std::mem::size_of;

pub fn new_advertiser(ctx: Context<NewAdvertiser>) -> Result<()> {
    let advertiser = &mut ctx.accounts.advertiser;
    advertiser.authority = ctx.accounts.user.key();
    advertiser.num_bids = 0;
    advertiser.num_auctions_won = 0;
    // create an empty list of strings
    advertiser.bump = ctx.bumps.advertiser;
    Ok(())
}

#[account]
pub struct Advertiser {
    pub authority: Pubkey,
    pub num_bids: u64,
    pub num_auctions_won: u64,
    // string of length 32
    pub bump: u8,
}

#[derive(Accounts)]
pub struct NewAdvertiser<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + size_of::<Advertiser>(),
        seeds = [b"advertiser", user.key().as_ref()],
        bump
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}