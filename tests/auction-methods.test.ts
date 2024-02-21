// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { AuctionSpace } from "../target/types/auction_space";
// import { PublicKey } from "@solana/web3.js";
// import {
//   newPublisher,
//   newAdvertiser,
//   createAuction,
// } from "./init-methods.test";
// import { BN } from "bn.js";
// import { expect } from "chai";

// anchor.setProvider(anchor.AnchorProvider.env());

// const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

// async function createAndActivateAuction(auction_end, effect_start, effect_end) {
//   const [publisherWallet, publisherPDA, auctionPDA] = await createAuction();
//   const auction = await program.account.auction.fetch(auctionPDA);
//   const tx = await program.methods
//     .activateAuction(auction_end, effect_start, effect_end)
//     .accounts({
//       auction: auctionPDA,
//       authority: publisherWallet.publicKey,
//     })
//     .signers([publisherWallet])
//     .rpc();
//   return [publisherWallet, publisherPDA, auctionPDA] as [
//     anchor.web3.Keypair,
//     PublicKey,
//     PublicKey
//   ];
// }
// describe("auction-methods", () => {
//   // Configure the client to use the local cluster.

//   it("startAuction", async () => {
//     // return;
//     // expect(auction.duration.toNumber()).eq(0);
//     const timestamp_now = new BN(Date.now() / 1000);
//     // 100 seconds from now (in milliseconds)
//     const auction_end = timestamp_now.addn(100);
//     const effect_start = auction_end.addn(200);
//     const effect_end = effect_start.addn(100);

//     const [publisherWallet, publisherPDA, auctionPDA] =
//       await createAndActivateAuction(auction_end, effect_start, effect_end);
//     const auctionNew = await program.account.auction.fetch(auctionPDA);
//     // expect(auctionNew.duration.toNumber()).eq(100);
//     expect(auctionNew.active.valueOf()).eq(true);
//     // const publisher = await program.account.publisher.fetch(publisherPDA);
//     console.log("startAuction done");
//   });

//   it("bid", async () => {
//     console.log("TEST1 START");
//     const timestamp_now = new BN(Date.now() / 1000);
//     // 100 seconds from now (in milliseconds)
//     const auction_end = timestamp_now.addn(100);
//     const effect_start = auction_end.addn(200);
//     const effect_end = effect_start.addn(100);

//     const [publisherWallet, publisherPDA, auctionPDA] =
//       await createAndActivateAuction(auction_end, effect_start, effect_end);
//     const [advertiserWallet, advertiserPDA] = await newAdvertiser();
//     const advertiserWalletBalanceBefore =
//       await program.provider.connection.getBalance(advertiserWallet.publicKey);
//     // console.log("advertiserWalletBalanceBefore", advertiserWalletBalanceBefore)
//     const tx = await program.methods
//       .bid(new BN(95000))
//       .accounts({
//         auction: auctionPDA,
//         // advertiser: advertiserPDA,
//         curHighBid: PublicKey.default,
//         bidder: advertiserWallet.publicKey,
//       })
//       .signers([advertiserWallet])
//       .rpc();
//     const auctionWithBid = await program.account.auction.fetch(auctionPDA);
//     console.log("auctionWithBid", auctionWithBid);
//     console.log("newHighestBid", auctionWithBid.winningBid.toString());

//     const advertiserWalletBalanceAfter =
//       await program.provider.connection.getBalance(advertiserWallet.publicKey);
//     // console.log("advertiserWalletBalanceAfter", advertiserWalletBalanceAfter)
//     expect(advertiserWalletBalanceAfter).eq(
//       advertiserWalletBalanceBefore - (95000 * 21) / 20
//     );
//     expect(auctionWithBid.winningBid.toNumber()).eq(95000);
//     expect(auctionWithBid.winningUser.toString()).eq(
//       advertiserWallet.publicKey.toString()
//     );
//     // const default pubkey is
//     console.log("TEST1 DONE");
//     const [newAdvertiserWallet, newAdvertiserPDA] = await newAdvertiser();
//     // bid below previous high bid, so should fail
//     try {
//       const tx2 = await program.methods
//         .bid(new BN(90000))
//         .accounts({
//           auction: auctionPDA,
//           // advertiser: newAdvertiserPDA,
//           curHighBid: advertiserWallet.publicKey,
//           bidder: newAdvertiserWallet.publicKey,
//         })
//         .signers([newAdvertiserWallet])
//         .rpc();
//       throw new Error("bid should have failed");
//     } catch (e) {
//       expect(e.error.errorMessage).eq("Not highest bid");
//     }
//     const auctionWithBid2 = await program.account.auction.fetch(auctionPDA);
//     // console.log("auctionWithBid2", auctionWithBid2);
//     // console.log("newHighestBid", auctionWithBid2.highestBid.toString());
//     expect(auctionWithBid2.winningBid.toNumber()).eq(95000);
//     expect(auctionWithBid2.winningUser.toString()).eq(
//       advertiserWallet.publicKey.toString()
//     );

//     console.log("TEST2 DONE");

//     // print balance before tx
//     const newAdvertiserWalletBalanceBefore =
//       await program.provider.connection.getBalance(
//         newAdvertiserWallet.publicKey
//       );
//     console.log("balance before re bid", newAdvertiserWalletBalanceBefore);
//     // bid above previous high bid, so should succeed
//     const tx3 = await program.methods
//       .bid(new BN(100000))
//       .accounts({
//         auction: auctionPDA,
//         advertiser: newAdvertiserPDA,
//         curHighBid: advertiserWallet.publicKey,
//         user: newAdvertiserWallet.publicKey,
//       })
//       .signers([newAdvertiserWallet])
//       .rpc();
//     const auctionWithBid3 = await program.account.auction.fetch(auctionPDA);
//     // console.log("auctionWithBid3", auctionWithBid3);
//     // console.log("newHighestBid", auctionWithBid3.highestBid.toString());
//     expect(auctionWithBid3.winningBid.toNumber()).eq(100000);
//     // check balance after
//     const newAdvertiserWalletBalanceAfter =
//       await program.provider.connection.getBalance(
//         newAdvertiserWallet.publicKey
//       );
//     console.log("balance after bid", newAdvertiserWalletBalanceAfter);
//   });
// });
