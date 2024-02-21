use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("41vpQQjubJn25HVHTLTt6YZViRM9hefnd9DJ1PXvt5NE");

const MAX_STRING_LENGTH: usize = 32;
const PUBLISHER_DEPOSIT: u64 = 10000;

#[program]
pub mod auction_space {
    use super::*;

    pub fn new_publisher(ctx: Context<NewPublisher>) -> Result<()> {
        instructions::handle_new_publisher(ctx)
    }

    pub fn new_advertiser(ctx: Context<NewAdvertiser>) -> Result<()> {
        instructions::handle_new_advertiser(ctx)
    }

    // pub fn new_auction(ctx: Context<NewAuction>, title: String) -> Result<()> {
    //     new_auction(
    //         ctx, 
    //         title
    //     )
    // }

    pub fn new_auction(ctx: Context<NewAuction>, item: Pubkey, min_bid: u64, auction_start: u64, auction_end: u64, effect_start: u64, effect_end: u64) -> Result<()> {
        instructions::handle_new_auction(
            ctx,
            item,
            min_bid, 
            auction_start, 
            auction_end, 
            effect_start, 
            effect_end
        )
    }

    // pub fn upload_ad(ctx: Context<UploadAd>, url: String) -> Result<()> {
    //     auction::upload_ad(ctx, url)
    // }

    pub fn new_item(ctx: Context<NewItem>, title: String, url: String) -> Result<()> {
        instructions::handle_new_item(ctx, title, url)
    }

    pub fn bid(ctx: Context<Bid>, bid_amount: u64, ad_url: String) -> Result<()> {
        instructions::handle_bid(ctx, bid_amount, ad_url)
    }

    pub fn pub_backout(ctx: Context<PubBackout>) -> Result<()> {
        instructions::handle_pub_backout(ctx)
    }

    pub fn adv_backout(ctx: Context<AdvBackout>) -> Result<()> {
        instructions::handle_adv_backout(ctx)
    }

}
