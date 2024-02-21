import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import getWallets from "./utils/getWallets";
import getAdvertisers from "./utils/getAdvertisers";
import getPublishers from "./utils/getPublishers";
import requestAirdrops from "./utils/requestAirdrops";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import getAuction from "./utils/getAuction";
import getItem from "./utils/getItem";

anchor.setProvider(anchor.AnchorProvider.env());

const BN = anchor.BN;
const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

describe("bid", async () => {
    it("simpleBid", async () => {
        const publishers = await getPublishers(1);
        const [publisherWallet, publisherPDA] = publishers[0];
        const advertisers = await getAdvertisers(2);
        const [advertiserWallet, advertiserPDA] = advertisers[0];
        const [aWal2, aPDA2] = advertisers[1];
        const timestampNow = Date.now() / 1000;
        const item = await getItem(publisherWallet, "item1", "google.com");
        const auctionPDA = await getAuction(
            publisherWallet,
            item as PublicKey,
            10000,
            timestampNow - 100,
            timestampNow + 1000,
            timestampNow + 2000,
            timestampNow + 3000
        );
        expect(auctionPDA).to.not.be.null;
        
        const tx = await program.methods.bid(
            new BN(15000),
            "google.com"
        ).accounts({
            auction: auctionPDA,
            advertiserWallet: advertiserWallet.publicKey,
            prevBidderWallet: publisherWallet.publicKey,
        }).signers([advertiserWallet]).rpc();
        const auction = await program.account.auction.fetch(auctionPDA);
        // console.log(auction)
        expect(auction.curWinnerBid.toNumber()).to.be.equal(15000);
        expect(auction.curWinnerWallet.toString()).to.be.equal(advertiserWallet.publicKey.toString())
    });
    it("rebid", async () => {
        const publishers = await getPublishers(1);
        const [publisherWallet, publisherPDA] = publishers[0];
        const advertisers = await getAdvertisers(2);
        const [advertiserWallet, advertiserPDA] = advertisers[0];
        const [aWal2, aPDA2] = advertisers[1];
        const timestampNow = Date.now() / 1000;
        const item = await getItem(publisherWallet, "item1", "google.com");
        const auctionPDA = await getAuction(
            publisherWallet,
            item as PublicKey,
            10000,
            timestampNow - 100,
            timestampNow + 1000,
            timestampNow + 2000,
            timestampNow + 3000
        );
        expect(auctionPDA).to.not.be.null;
        
        const tx = await program.methods.bid(
            new BN(15000),
            "google.com"
        ).accounts({
            auction: auctionPDA,
            advertiserWallet: advertiserWallet.publicKey,
            prevBidderWallet: publisherWallet.publicKey,
        }).signers([advertiserWallet]).rpc();

        // aWal2 balance before
        const balanceBefore = await program.provider.connection.getBalance(aWal2.publicKey);
        const balanceBefore2 = await program.provider.connection.getBalance(advertiserWallet.publicKey);

        const tx2 = await program.methods.bid(
            new BN(16000),
            "yahoo.com"
        ).accounts({
            auction: auctionPDA,
            advertiserWallet: aWal2.publicKey,
            prevBidderWallet: advertiserWallet.publicKey,
        }).signers([aWal2]).rpc().catch((err) => console.log(err));

        const auction = await program.account.auction.fetch(auctionPDA);

        const balanceAfter = await program.provider.connection.getBalance(aWal2.publicKey);
        const balanceAfter2 = await program.provider.connection.getBalance(advertiserWallet.publicKey);

        expect(balanceBefore2 - balanceAfter2).to.be.at.equal(-15000)
        expect(balanceBefore - balanceAfter).to.be.equal(16000)
        expect(auction.curWinnerAdUrl).to.be.equal("yahoo.com")
        expect(auction.curWinnerBid.toNumber()).to.be.equal(16000)
        expect(auction.curWinnerWallet.toString()).to.be.equal(aWal2.publicKey.toString())
    })
});
