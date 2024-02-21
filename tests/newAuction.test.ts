import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import getWallets from "./utils/getWallets";
import getAdvertisers from "./utils/getAdvertisers";
import getPublishers from "./utils/getPublishers";
import requestAirdrops from "./utils/requestAirdrops";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import getItem from "./utils/getItem";

anchor.setProvider(anchor.AnchorProvider.env());

const BN = anchor.BN;
const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

it("newAuction", async () => {
    const advertisers = await getAdvertisers(1);
    const publishers = await getPublishers(1);
    const [advertiserWallet, advertiserPDA] = advertisers[0];
    const [publisherWallet, publisherPDA] = publishers[0];
    const item = await getItem(publisherWallet, "item1", "google.com");
    const timestampNow = new BN(Date.now() / 1000);

    const tx1 = await program.methods.newAuction(
        item as PublicKey,
        new BN(1000),
        timestampNow,
        timestampNow.add(new anchor.BN(1000)),
        timestampNow.add(new anchor.BN(2000)),
        timestampNow.add(new anchor.BN(3000))
    ).accounts({
        publisherWallet: publisherWallet.publicKey,
        publisher: publisherPDA,
    }).signers([publisherWallet]).rpc();
    // console.log("newAuction called")
    const seedBuffer = Buffer.alloc(8); // 4 bytes for a 32-bit number
    seedBuffer.writeUInt32LE(0, 0); // Write the number in little endian format
    const [auctionPDA, _] = PublicKey.findProgramAddressSync(
        [Buffer.from("auction"), publisherWallet.publicKey.toBuffer(), seedBuffer],
        program.programId
    );

    const auction = await program.account.auction.fetch(auctionPDA);
});
