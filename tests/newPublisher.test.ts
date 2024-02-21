import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import getWallets from "./utils/getWallets";
import requestAirdrops from "./utils/requestAirdrops";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

it("newPublisher", async () => {
  const [myWallet] = await getWallets(1);
  await requestAirdrops([myWallet], 10000000000);
  const tx1 = await program.methods
    .newPublisher()
    .accounts({
      publisherWallet: myWallet.publicKey,
    })
    .signers([myWallet])
    .rpc();

  const [publisherPDA, _] = PublicKey.findProgramAddressSync(
    [Buffer.from("publisher"), myWallet.publicKey.toBuffer()],
    program.programId
  );
  const publisher = await program.account.publisher.fetch(publisherPDA);
  expect(publisher.publisherWallet).to.eql(myWallet.publicKey);
});
