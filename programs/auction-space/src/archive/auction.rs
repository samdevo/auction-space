use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program};

use crate::publisher::*;
use crate::advertiser::Advertiser;
use solana_program::system_instruction;
use crate::utils::{transfer_pda_to_user, transfer_user_to_pda};
// get auction struct from ./structs/auction.rs - NOT ITS OWN CRATE
use crate::structs::auction::Auction;
// use spl_token::token::TokenAccount as TokenAccount;


// float deposit
const DEPOSIT_PCT: f64 = 0.05;
const PUBLISHER_DEPOSIT: u64 = 10000;
// 100000 lamports is 0.0001 SOL = $0.01

fn deposit(amount: u64) -> u64 {
    (amount as f64 * DEPOSIT_PCT) as u64
}



pub fn create_auction(ctx: Context<CreateAuction>, title: String) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    auction.publisher = publisher.key();
    // if title is more than 32 bytes, throw an error
    if title.len() > crate::MAX_STRING_LENGTH {
        return err!(AuctionErrors::TitleTooLong);
    }
    auction.title = title;
    auction.active = false; // is this default?
    auction.bump = ctx.bumps.auction;
    auction.id = publisher.num_auctions;
    // increment the number of auctions for publisher
    publisher.num_auctions += 1;
    Ok(())
}


#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 128 + size_of::<Auction>(),
        seeds = [
            b"auction", 
            authority.key().as_ref(), 
            &publisher.num_auctions.to_le_bytes()
        ],
        bump
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

pub fn activate_auction(ctx: Context<ActivateAuction>, auction_end: u64, effect_start: u64, effect_end: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let authority = &ctx.accounts.authority;
    if auction.active {
        return err!(AuctionErrors::AuctionAlreadyActive);
    }
    
    let clock = Clock::get()?;
    msg!("timestamp: {:?}", clock.unix_timestamp.unsigned_abs());
    msg!("auction_end: {:?}", auction_end);
    msg!("effect_start: {:?}", effect_start);
    msg!("effect_end: {:?}", effect_end);
    let timestamp = clock.unix_timestamp.unsigned_abs();

    if timestamp > auction_end {
        return err!(AuctionErrors::AuctionEndsBeforeStart);
    }
    if auction_end > effect_start {
        return err!(AuctionErrors::AuctionEffectBeforeEnd);
    }
    if effect_start >= effect_end {
        return err!(AuctionErrors::AuctionEffectEndBeforeStart);
    }
    auction.active = true;
    let transfer = system_instruction::transfer(
        &authority.key(),
        &auction.key(),
        PUBLISHER_DEPOSIT,
    );
    msg!("PUBLISHER DEPOSIT: transferring {} lamports from {} to {}", PUBLISHER_DEPOSIT, authority.key(), auction.key());
    solana_program::program::invoke(
        &transfer,
        &[
            authority.to_account_info().clone(),
            auction.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    auction.start_time = timestamp;
    auction.end_time = auction_end;
    auction.effect_start_time = effect_start;
    auction.effect_end_time = effect_end;
    Ok(())
}

#[derive(Accounts)]
pub struct ActivateAuction<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    pub system_program: Program<'info, System>,
}

// check for the end of the auction, and set the winner
fn check_status(auction: &mut Auction) {
    let clock = Clock::get().unwrap();
    if clock.unix_timestamp.unsigned_abs() > auction.end_time {
        auction.active = false;
        auction.completed = true;
        // handle end of auction
        msg!("auction ended. winner is {}", auction.winning_user);
        return;
    }
}

pub fn upload_ad(ctx: Context<UploadAd>, url: String) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    let user = &ctx.accounts.user;
    check_status(auction);
    if auction.winning_user != user.key() {
        return err!(AuctionErrors::NotHighestBid);
    }
    if auction.aborted {
        return err!(AuctionErrors::AuctionAborted);
    }
    // if url is longer than 32 bytes, throw an error
    if url.len() > crate::MAX_STRING_LENGTH {
        return err!(AuctionErrors::TitleTooLong);
    }
    publisher.ad_url = url;
    Ok(())
}

#[derive(Accounts)]
pub struct UploadAd<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub publisher: Account<'info, Publisher>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
    msg!("BIDDING");
    let auction = &mut ctx.accounts.auction;
    // let advertiser = &mut ctx.accounts.advertiser;
    let cur_high_bid = &ctx.accounts.cur_high_bid;
    let user = &ctx.accounts.bidder;
    check_status(auction);
    // TODO if winner, increase auctions won
    if amount <= auction.winning_bid {
        return err!(AuctionErrors::NotHighestBid);
    }
    if !auction.active {
        return err!(AuctionErrors::AuctionNotActive);
    }
    // 5% deposit;

    let _ = transfer_user_to_pda(
        user.to_account_info(), 
        auction.to_account_info(), 
        ctx.accounts.system_program.to_account_info(), 
        amount + deposit(amount)
    );

    // transfer amount from user to auction in case they win
    // let transfer = system_instruction::transfer(
    //     &user.key(),
    //     &auction.key(),
    //     amount + deposit(amount),
    // );
    // msg!("transferring {} lamports from {} to {}", amount, user.key(), auction.key());
    // solana_program::program::invoke(
    //     &transfer,
    //     &[
    //         user.to_account_info().clone(),
    //         auction.to_account_info().clone(),
    //         ctx.accounts.system_program.to_account_info().clone(),
    //     ],
    // )?;

    msg!("bid successful. transferred {} lamports from {} to {}", amount, user.key(), auction.key());

    // refund previous highest bidder
    if auction.winning_user != Pubkey::default() {
        msg!("refunding previous highest bidder");
        let _ = transfer_pda_to_user(
            auction.to_account_info().clone(),
            cur_high_bid.clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            auction.winning_bid + deposit(auction.winning_bid),
            &[
                b"auction".as_ref(), 
                auction.publisher_user.key().as_ref(), 
                &auction.id.to_le_bytes()
            ],
        );
    }
    auction.winning_bid = amount;
    auction.winning_user = user.key();
    // auction.winning_advertiser = advertiser.key();
    // advertiser.num_bids += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(
        seeds = [b"auction".as_ref(), auction.publisher_user.key().as_ref(), &auction.id.to_le_bytes()],
        bump,
        mut
    )]
    pub auction: Account<'info, Auction>,
    // #[account(mut)]
    // pub advertiser: Account<'info, Advertiser>,
    // account of the highest bidder, who we wish to refund
    #[account(
        constraint = cur_high_bid.key() == auction.winning_user,
    )]
    /// CHECK: this is not dangerous because we verify that it is the correct user in the auction struct
    pub cur_high_bid: AccountInfo<'info>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AuctionErrors {
    #[msg("Title too long")]
    TitleTooLong,
    #[msg("Auction is already active")]
    AuctionAlreadyActive,
    #[msg("Auction is not active")]
    AuctionNotActive,
    #[msg("Not highest bid")]
    NotHighestBid,
    AuctionEndsBeforeStart,
    AuctionEffectBeforeEnd,
    AuctionEffectEndBeforeStart,
    AuctionNotCompleted,
    AuctionAborted,
    AuctionAlreadyAborted,
    AuctionAlreadyEnded,

}