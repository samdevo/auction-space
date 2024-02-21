use anchor_lang::prelude::*;

#[account]
pub struct Publisher {
    pub publisher_wallet: Pubkey,
    pub num_auctions: u64,
    pub num_items: u64,
    pub publisher_backouts: u64,
    pub advertiser_backouts: u64,
    pub bump: u8,
}

pub const PUBLISHER_SIZE: usize = 
    8 + // wallet
    8 + // num_auctions
    8 + // num_items
    8 + // num_backed_out_publisher
    8 + // num_backed_out_advertiser
    1 + // bump
    32;  // padding
