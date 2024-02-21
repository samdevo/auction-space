use anchor_lang::prelude::*;
use crate::{state::*, AuctionErrors};


pub fn payout(
    auction: &mut Account<Auction>,
    publisher_wallet: &AccountInfo,
    advertiser_wallet: &AccountInfo,
    publisher: &mut Account<Publisher>,
    advertiser: &mut Account<Advertiser>,
    is_publisher: bool,
) -> Result<()> {
    let clock = Clock::get()?;
    let cur_time = clock.unix_timestamp.unsigned_abs();
    if auction.aborted {
        return err!(AuctionErrors::AuctionAlreadyAborted);
    }
    if cur_time > auction.effect_end_time {
        return err!(AuctionErrors::AuctionAlreadyEnded);
    }
    auction.aborted = true;
    if is_publisher {
        publisher.publisher_backouts += 1;
        advertiser.publisher_backouts += 1;
    } else {
        publisher.advertiser_backouts += 1;
        advertiser.advertiser_backouts += 1;
    }

    let mut portion_time_elapsed = 0;
    if cur_time > auction.effect_start_time {
        // get the time since the auction started
        portion_time_elapsed = (cur_time - auction.effect_start_time) / (auction.effect_end_time - auction.effect_start_time);
    } 
    // TODO deal with deposits
    let publisher_payout = auction.cur_winner_bid * portion_time_elapsed;
    let advertiser_payout = auction.cur_winner_bid - publisher_payout;

    if publisher_payout > 0 {
        **publisher_wallet.to_account_info().try_borrow_mut_lamports()? += publisher_payout;
    }

    if advertiser_payout > 0 {
        **advertiser_wallet.to_account_info().try_borrow_mut_lamports()? += advertiser_payout;
    }

    **auction.to_account_info().try_borrow_mut_lamports()? -= auction.cur_winner_bid;
    Ok(())
}