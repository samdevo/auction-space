use anchor_lang::prelude::*;

use crate::publisher::Publisher;
use crate::advertiser::Advertiser;
use crate::auction::{Auction, AuctionErrors};
use crate::utils::transfer_pda_to_user;

pub fn publisher_backout(ctx: Context<PublisherBackout>) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    let publisher_user = &mut ctx.accounts.publisher_user;
    let advertiser = &mut ctx.accounts.advertiser;
    let advertiser_user = &mut ctx.accounts.advertiser_user;
    
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
    publisher.publisher_backouts += 1;
    advertiser.publisher_backouts += 1;
    auction.aborted_by_publisher = true;

    payout(
        auction,
        &mut publisher_user.to_account_info(),
        advertiser_user,
        &mut ctx.accounts.system_program,
        cur_time
    );
    Ok(())
}

pub fn advertiser_backout(ctx: Context<AdvertiserBackout>) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    let publisher_user = &mut ctx.accounts.publisher_user;
    let advertiser = &mut ctx.accounts.advertiser;
    let advertiser_user = &mut ctx.accounts.advertiser_user;
    
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
    publisher.advertiser_backouts += 1;
    advertiser.advertiser_backouts += 1;
    auction.aborted_by_publisher = false;

    payout(
        auction,
        publisher_user,
        &mut advertiser_user.to_account_info(),
        &mut ctx.accounts.system_program,
        cur_time
    );
    Ok(())
}

fn payout<'info>(
    auction: &mut Account<'info, Auction>, 
    publisher_user: &mut AccountInfo<'info>, 
    advertiser_user: &mut AccountInfo<'info>, 
    system_program: &mut Program<'info, System>,
    cur_time: u64,
) {
    auction.aborted = true;
    auction.aborted_at = cur_time;
    // auction is completed, effect note done, effect hasn't necessarily started (either pre-start or in progress)
    let mut portion_time_elapsed = 0;
    if cur_time > auction.effect_start_time {
        // get the time since the auction started
        portion_time_elapsed = (cur_time - auction.effect_start_time) / (auction.effect_end_time - auction.effect_start_time);
    }
    // note deposits are NOT paid back
    let publisher_payout = auction.winning_bid * portion_time_elapsed;
    let advertiser_payout = auction.winning_bid - publisher_payout;

    let _ = transfer_pda_to_user(
        auction.to_account_info().clone(),
        publisher_user.to_account_info().clone(),
        system_program.to_account_info().clone(),
        publisher_payout,
        &[
            b"auction".as_ref(), 
            auction.publisher_user.key().as_ref(), 
            &auction.id.to_le_bytes()
        ]
    );

    let _ = transfer_pda_to_user(
        auction.to_account_info().clone(),
        advertiser_user.to_account_info().clone(),
        system_program.to_account_info().clone(),
        advertiser_payout,
        &[
            b"auction".as_ref(), 
            auction.publisher_user.key().as_ref(), 
            &auction.id.to_le_bytes()
        ]
    );
}


#[derive(Accounts)]
pub struct PublisherBackout<'info> {
    #[account(
        constraint = auction.publisher == publisher.key() && auction.winning_advertiser == advertiser.key(),
        mut
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        seeds = [b"publisher".as_ref(), publisher_user.key().as_ref()],
        bump = publisher.bump,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub publisher_user: Signer<'info>,
    #[account(
        seeds = [b"advertiser".as_ref(), advertiser_user.key().as_ref()],
        bump = advertiser.bump,
        mut
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    /// CHECK: this is not dangerous because we verify that it is the correct user in the auction struct
    pub advertiser_user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvertiserBackout<'info> {
    #[account(
        constraint = auction.publisher == publisher.key() && auction.winning_advertiser == advertiser.key(),
        mut
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        seeds = [b"advertiser".as_ref(), advertiser.key().as_ref()],
        constraint = advertiser.authority == advertiser_user.key(),
        bump = advertiser.bump,
        mut
    )]
    pub advertiser: Account<'info, Publisher>,
    #[account(mut)]
    pub advertiser_user: Signer<'info>,
    #[account(
        seeds = [b"publisher".as_ref(), publisher_user.key().as_ref()],
        constraint = publisher.authority == publisher_user.key(),
        bump = publisher.bump,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    /// CHECK: this is not dangerous because we verify that it is the correct user in the auction struct
    pub publisher_user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}