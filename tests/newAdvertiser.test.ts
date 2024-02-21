import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import getWallets from "./utils/getWallets";
import requestAirdrops from "./utils/requestAirdrops";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

it("newAdvertiser", async () => {
  const [myWallet] = await getWallets(1);
  await requestAirdrops([myWallet], 10000000000);
  const tx1 = await program.methods
    .newAdvertiser()
    .accounts({
      advertiserWallet: myWallet.publicKey,
    })
    .signers([myWallet])
    .rpc();

  const [advertiserPDA, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("advertiser"), myWallet.publicKey.toBuffer()],
    program.programId
  );
  const advertiser = await program.account.advertiser.fetch(advertiserPDA);
  expect(advertiser.advertiserWallet).to.eql(myWallet.publicKey);
});
