use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program};

use crate::publisher::Publisher;
use crate::advertiser::Advertiser;
use crate::auction::{Auction, AuctionErrors};
use solana_program::system_instruction;

pub fn publisher_backout(ctx: Context<PublisherBackout>) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    let authority = &mut ctx.accounts.authority;
    if !auction.completed {
        return err!(AuctionErrors::AuctionNotCompleted);
    }
    let clock = Clock::get()?;
    let cur_time = clock.unix_timestamp.unsigned_abs();
    // get the time since the auction started
    if cur_time <= auction.start_time {
        // auction has not started yet
        auction.aborted = true;
        // TODO return the money to the advertiser
        return Ok(());
    }
    Ok(())
}

pub fn advertiser_backout(ctx: Context<AdvertiserBackout>) -> Result<()> {
    // TODO
    Ok(())
}


#[derive(Accounts)]
pub struct PublisherBackout<'info> {
    #[account(
        constraint = auction.publisher == publisher.key(),
        mut
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        seeds = [b"publisher".as_ref(), authority.key().as_ref()],
        bump = publisher.bump,
        has_one = authority,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvertiserBackout<'info> {
    #[account(
        seeds = [b"advertiser".as_ref(), authority.key().as_ref()],
        bump = advertiser.bump,
        has_one = authority,
        mut
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}