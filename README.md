# Auction Space

## *Trustless auctions built on Solana*

Traditionally, web advertising is handled by an intermediary, such as Google Ads, which takes a cut of the revenue. Auction Space is a trustless auctioning platform that allows advertisers to bid on ad space directly from the publisher. This allows publishers to maximize their revenue and advertisers to minimize their costs, with no uncertainty about the fairness of the auction.

## Why trustless?

- This auction system is meant to prevent the publisher from being able to manipulate the auction in their favor. However, it does not prevent the publisher from simply not displaying the ad, or displaying it in a way that is not visible to the user.
    - For this reason, we provide the methods `advertiserBackout()` and `auctionBackout()`, which allow the advertiser and publisher to abort the agreement at any time.

- For example, an advertiser might want to abort the agreement if they find out that the publisher is not displaying their ad. Or, with bad intentions, they might want to win an auction and then abort the agreement, preventing the publisher from displaying any ads at all (a denial of service attack).
    - We design this system such that if the advertiser aborts the aggreement, they they will lose some small deposit (which is returned at the end if the ad is not aborted). This is to disincentivize advertisers from aborting the agreement. However, if we hand this penalty to the publisher, then the publisher could just not display the ad until the advertiser aborts the agreement, and then collect the penalty. 
    - Instead, we keep the penalty in a personal vault (if the advertiser aborts the agreement, the penalty is sent to the vault)
    - Money from this vault can be collected gradually by fully completing auctions (i.e. neither side aborting the agreement)

- The same logic applies to the publisher, who may want to abort the agreement if they are unhappy with the ad that the advertiser has uploaded. With bad intentions, they might want to win an auction and then abort the agreement immediately. 
    - Similarly, we design this system such that if the publisher aborts the aggreement, they they will lose some small deposit. This is to disincentivize publishers from aborting the agreement. Over time with successful auctions, the publisher can collect money from their vault.

## How to use

### For publishers

1. Create a publisher account with `newPublisher()`
2. Create an auction with `createAuction()`, and start it with `activateAuction(auction_end: u64, effect_start: u64, effect_end: u64)`
3. At any time, you can abort the agreement with `auctionBackout()`

### For advertisers

1. Create an advertiser account with `newAdvertiser()`
2. Bid on an auction with `bid(amount)`
3. If you win the auction, you can upload your ad with `uploadAd()`
4. If someone else outbids you, your bid will be refunded automatically
5. If you win an auction, you can abort the agreement with `advertiserBackout()`

## How it's built

Auction Space is built on Solana, written in Rust using the [Anchor](https://www.anchor-lang.com/) framework, and tests are in TypeScript.
