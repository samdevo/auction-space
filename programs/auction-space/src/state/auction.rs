use anchor_lang::prelude::*;

#[account]
pub struct Auction {
    // title of the auction
    pub item: Pubkey,
    pub min_bid: u64,
    // publisher struct and the user that owns the publisher
    pub publisher: Pubkey, // Publi
    pub publisher_wallet: Pubkey, // User
    // advertiser struct and the user that owns the advertiser
    pub cur_winner_wallet: Pubkey, // Advertiser
    pub cur_winner_bid: u64,
    pub cur_winner_ad_url: String,
    // auction start and end time, in Unix seconds
    pub start_time: u64,
    pub end_time: u64,
    // auction EFFECT start and end time, in Unix seconds
    pub effect_start_time: u64,
    pub effect_end_time: u64,
    // is the auction (bidding phase) active -- inferred????
    pub active: bool,
    // is the effect phase completed
    pub completed: bool,
    // was the auction aborted
    pub aborted: bool,
    // who aborted the auction (Publisher or Advertiser)
    pub aborted_by_publisher: bool,
    pub aborted_at: u64,

    // unique identifier for the auction, equal to the number of auctions the publisher has created upon creation
    pub id: u64,

    pub bump: u8,
}

pub const AUCTION_SIZE: usize = 
    8 + // item
    8 + // min_bid
    8 + // publisher
    8 + // publisher_wallet

    8 + // cur_winner_wallet
    8 + // cur_winner
    8 + // cur_winner_bid

    8 + // start_time
    8 + // end_time

    8 + // effect_start_time
    8 + // effect_end_time

    1 + // active
    1 + // completed
    1 + // aborted

    1 + // aborted_by_publisher
    8 + // aborted_at

    8 + // id

    1 + //bump
    128;  // padding]
