use anchor_lang::prelude::*;

#[account]
pub struct Publisher {
    pub publisher_wallet: Pubkey,
    pub num_auctions: u64,
    pub num_backed_out_publisher: u64,
    pub num_backed_out_advertiser: u64,
    pub bump: u8,
}

pub const PUBLISHER_SIZE: usize = 
    8 + // wallet
    8 + // num_auctions
    8 + // num_backed_out_publisher
    8 + // num_backed_out_advertiser
    1 + // bump
    8;  // padding
