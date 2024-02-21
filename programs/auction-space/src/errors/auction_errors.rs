use anchor_lang::prelude::*;

#[error_code]
pub enum AuctionErrors {
    TitleTooLong,
    AuctionAlreadyActive,
    AuctionNotActive,
    NotHighestBid,
    AuctionEndsBeforeStart,
    AuctionEffectBeforeEnd,
    AuctionEffectEndBeforeStart,
    AuctionNotCompleted,
    AuctionAborted,
    AuctionAlreadyAborted,
    AuctionAlreadyEnded,
    WrongHighBidder,
    BelowMinBid
}