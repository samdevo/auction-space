use anchor_lang::{prelude::*, solana_program};
use solana_program::system_instruction;
use crate::state::*;
use crate::errors::*;
use crate::MAX_STRING_LENGTH;

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(
        seeds = [b"auction", auction.publisher_wallet.as_ref(), &auction.id.to_le_bytes()],
        bump = auction.bump,
        mut
    )]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub advertiser_wallet: Signer<'info>,
    /// CHECK: No need to deserialize. This account will be paid to
    #[account(mut)]
    pub prev_bidder_wallet: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_bid(
    ctx: Context<Bid>,
    bid_amount: u64,
    ad_url: String
) -> Result<()> {
    // let clock = Clock::get().unwrap();
    // let timestamp = clock.unix_timestamp.unsigned_abs();
    // let auction = &mut ctx.accounts.auction;
    // msg!("now: {:?}", clock.unix_timestamp);
    // msg!("timestamp: {:?}", timestamp);
    // msg!("auction start: {:?}", auction.start_time);
    // msg!("auction end: {:?}", auction.end_time);
    // msg!("THIS IS MY MESSAGE");
    let auction = &mut ctx.accounts.auction;
    // let advertiser = &ctx.accounts.advertiser;
    let advertiser_wallet = &mut ctx.accounts.advertiser_wallet;
    let prev_bidder_wallet = &ctx.accounts.prev_bidder_wallet;

    if !auction.cur_winner_wallet.eq(prev_bidder_wallet.key) {
        return err!(AuctionErrors::WrongHighBidder);
    }

    if ad_url.len() > MAX_STRING_LENGTH {
        return err!(AuctionErrors::URLTooLong);
    }
    

    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp.unsigned_abs();

    msg!("timestamp: {:?}", timestamp);
    msg!("auction start: {:?}", auction.start_time);
    msg!("auction end: {:?}", auction.end_time);

    if timestamp > auction.end_time || timestamp < auction.start_time {
        
        return err!(AuctionErrors::AuctionNotActive);
    }

    if bid_amount > auction.cur_winner_bid {
        if bid_amount <= auction.cur_winner_bid {
            return err!(AuctionErrors::NotHighestBid);
        }
        if bid_amount < auction.min_bid {
            return err!(AuctionErrors::BelowMinBid);
        }
        // we have new highest bidder
        let prev_high_bid = auction.cur_winner_bid;

        auction.cur_winner_wallet = advertiser_wallet.key();
        auction.cur_winner_bid = bid_amount;
        auction.cur_winner_ad_url = ad_url;

        // transfer amount from advertiser to auction

        let transfer = system_instruction::transfer(
            &advertiser_wallet.key(),
            &auction.key(),
            bid_amount,
        );
        
        solana_program::program::invoke(
            &transfer,
            &[
                advertiser_wallet.to_account_info().clone(),
                auction.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;

        if prev_high_bid > 0 {
            **auction.to_account_info().try_borrow_mut_lamports()? -= prev_high_bid;
            **prev_bidder_wallet.to_account_info().try_borrow_mut_lamports()? += prev_high_bid;
        }
    }
    Ok(())
}



