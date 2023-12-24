use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program};

use crate::publisher::Publisher;
use crate::advertiser::Advertiser;
use crate::auction::{Auction, AuctionErrors};
use solana_program::system_instruction;


// fn payout(auction: &Account<Auction>, system_program: Program<System>, account: Account<Signer>, amount: u64) {
//     let transfer = system_instruction::transfer(
//         &auction.key(),
//         &account.key(),
//         amount,
//     );
//     msg!("transferring {} lamports from {} to {}", amount, auction.key(), account.key());
//     solana_program::program::invoke(
//         &transfer,
//         &[
//             auction.to_account_info().clone(),
//             account.to_account_info().clone(),
//             system_program.to_account_info().clone(),
//         ],
//         // &[&[&user.key().to_bytes()]],
//     )?;
// }
pub fn publisher_backout(ctx: Context<PublisherBackout>) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    let authority = &mut ctx.accounts.authority;
    if !auction.completed {
        return err!(AuctionErrors::AuctionNotCompleted);
    }
    let clock = Clock::get()?;
    let cur_time = clock.unix_timestamp.unsigned_abs();
    if auction.aborted {
        return err!(AuctionErrors::AuctionAlreadyAborted);
    }
    if cur_time >= auction.effect_end_time {
        return err!(AuctionErrors::AuctionAlreadyEnded);
    }
    auction.aborted = true;
    auction.aborted_by = publisher.key();
    auction.aborted_at = cur_time;
    // auction is completed, effect note done, effect hasn't necessarily started (either pre-start or in progress)
    let mut portion_time_elapsed = 0;
    if cur_time > auction.effect_start_time {
        // get the time since the auction started
        portion_time_elapsed = (cur_time - auction.effect_start_time) / (auction.effect_end_time - auction.effect_start_time);
    }
    // note deposits are not paid back
    let publisher_payout = auction.highest_bid * portion_time_elapsed;
    let advertiser_payout = auction.highest_bid - publisher_payout;


    // get the time since the auction started
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