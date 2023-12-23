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
    // return;
    const [publisherWallet, publisherPDA, auctionPDA] = await createAuction();
    const auction = await program.account.auction.fetch(auctionPDA);
    expect(auction.duration.toNumber()).eq(0);
    const tx = await program.methods.activateAuction(new BN(100), new BN(10)).accounts({
        auction: auctionPDA,
        authority: publisherWallet.publicKey,
        })
        .signers([publisherWallet])
        .rpc();
    const auctionNew = await program.account.auction.fetch(auctionPDA);
    expect(auctionNew.duration.toNumber()).eq(100);
    expect(auctionNew.active.valueOf()).eq(true);
    // const publisher = await program.account.publisher.fetch(publisherPDA);

  })
  
});
