import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import { PublicKey } from '@solana/web3.js';
import { newPublisher, newAdvertiser, createAuction } from "./init-methods";
import { BN } from "bn.js";
import { expect } from "chai";

describe("auction-methods", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;


  it("startAuction", async () => {
    console.log("creating auction")
    const [publisherWallet, publisherPDA, auctionPDA] = await createAuction();
    const auction = await program.account.auction.fetch(auctionPDA);
    expect(auction.duration).eq(new BN(0));
    console.log("done creating auction")
    const tx = await program.methods.activateAuction(new BN(100), new BN(10)).accounts({
        auction: auctionPDA,
        authority: publisherWallet.publicKey,
        })
        .signers([publisherWallet])
        .rpc();
    console.log("fetching auction")
    const auctionNew = await program.account.auction.fetch(auctionPDA);
    expect(auction.duration).eq(new BN(100));

  })
  
});
