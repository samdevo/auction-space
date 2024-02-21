// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { AuctionSpace } from "../target/types/auction_space";
// import { PublicKey } from "@solana/web3.js";
// import { expect } from "chai";

// anchor.setProvider(anchor.AnchorProvider.env());

// const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

// async function newPublisher() {
//   const myWallet = anchor.web3.Keypair.generate();
//   const tx1 = await program.provider.connection.confirmTransaction(
//     await program.provider.connection.requestAirdrop(
//       myWallet.publicKey,
//       10000000000
//     ),
//     "confirmed"
//   );
//   const [publisherPDA, _] = PublicKey.findProgramAddressSync(
//     [
//       anchor.utils.bytes.utf8.encode("publisher"),
//       myWallet.publicKey.toBuffer(),
//     ],
//     program.programId
//   );
//   const tx = await program.methods
//     .newPublisher()
//     .accounts({
//       publisher: publisherPDA,
//       user: myWallet.publicKey,
//     })
//     .signers([myWallet])
//     .rpc();
//   return [myWallet, publisherPDA];
// }

// async function newAdvertiser() {
//   const myWallet = anchor.web3.Keypair.generate();
//   const tx1 = await program.provider.connection.confirmTransaction(
//     await program.provider.connection.requestAirdrop(
//       myWallet.publicKey,
//       10000000000
//     ),
//     "confirmed"
//   );
//   const [advertiserPDA, _] = PublicKey.findProgramAddressSync(
//     [
//       anchor.utils.bytes.utf8.encode("advertiser"),
//       myWallet.publicKey.toBuffer(),
//     ],
//     program.programId
//   );
//   const tx = await program.methods
//     .newAdvertiser()
//     .accounts({
//       advertiser: advertiserPDA,
//       user: myWallet.publicKey,
//     })
//     .signers([myWallet])
//     .rpc();
//   // return my wallet and the advertiserPDA
//   return [myWallet, advertiserPDA] as [anchor.web3.Keypair, PublicKey];
// }

// async function createAuction() {
//   // get publisherPDA and wallet
//   const [publisherWallet, publisherPDA] = (await newPublisher()) as [
//     anchor.web3.Keypair,
//     PublicKey
//   ];
//   // fetch the publisher account
//   const publisher = await program.account.publisher.fetch(publisherPDA);
//   // console.log("publisher", publisher);

//   // include numAuctions, and it is set as num_auctions.to_le_bytes()
//   // put it in the right format for the seed
//   const [auctionPDA, _] = PublicKey.findProgramAddressSync(
//     [
//       anchor.utils.bytes.utf8.encode("auction"),
//       publisherWallet.publicKey.toBuffer(),
//       publisher.numAuctions.toArrayLike(Buffer, "le", 8),
//     ],
//     program.programId
//   );
//   // console.log("auctionPDA", auctionPDA);
//   const tx = await program.methods
//     .createAuction("testAuction")
//     .accounts({
//       auction: auctionPDA,
//       publisher: publisherPDA,
//       authority: publisherWallet.publicKey,
//     })
//     .signers([publisherWallet])
//     .rpc();

//   const auction = await program.account.auction.fetch(auctionPDA);
//   expect(auction.title.toString()).eq("testAuction");
//   return [publisherWallet, publisherPDA, auctionPDA] as [
//     anchor.web3.Keypair,
//     PublicKey,
//     PublicKey
//   ];
// }

// describe("init-methods", () => {
//   // Configure the client to use the local cluster.

//   it("newPublisher", async () => {
//     await newPublisher();
//   });

//   it("newAdvertiser", async () => {
//     await newAdvertiser();
//   });

//   it("createAuction", async () => {
//     await createAuction();
//   });
// });

// // export newPublisher and newAdvertiser for use in other tests
// export { newPublisher, newAdvertiser, createAuction };
