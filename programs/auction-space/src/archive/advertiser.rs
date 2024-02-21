use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{auction::{Auction, AuctionErrors}, publisher::Publisher};

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
    pub publisher_backouts: u64,
    pub advertiser_backouts: u64,
    // string of length 32
    pub url: String,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct NewAdvertiser<'info> {
    #[account(
        init, 
        payer = user, 
        space = 24 + size_of::<Advertiser>(),
        seeds = [b"advertiser".as_ref(), user.key().as_ref()],
        bump
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// pub fn collect_payout(ctx: Context<CollectPayout>) -> Result<()> {
//     let auction = &mut ctx.accounts.auction;
//     let advertiser = &mut ctx.accounts.advertiser;
//     let user = &mut ctx.accounts.user;
//     let system_program = &mut ctx.accounts.system_program;
//     let clock = Clock::get()?;
//     let cur_time = clock.unix_timestamp.unsigned_abs();
//     if auction.aborted && auction.aborted_by  {

//     }
//     Ok(())
// }

// #[derive(Accounts)]
// pub struct CollectPayout<'info> {
//     #[account(
//         constraint = auction.publisher == publisher.key(),
//         mut
//     )]
//     pub auction: Account<'info, Auction>,
//     #[account(
//         seeds = [b"advertiser".as_ref(), authority.key().as_ref()],
//         bump = advertiser.bump,
//         has_one = authority,
//         mut
//     )]
//     pub advertiser: Account<'info, Advertiser>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }