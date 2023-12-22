import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import { PublicKey } from '@solana/web3.js';

describe("auction-space", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

  it("Is initialized!", async () => {
    // Add your test here.
    // create a new keypair
    const myWallet = anchor.web3.Keypair.generate();


    const [publisherPDA, _] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('publisher'),
        myWallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    // call createPublisher with the new keypair
    const tx = await program.methods
    .newPublisher()
    .accounts({
      publisher: publisherPDA,
      user: myWallet.publicKey,
    })
    .signers([myWallet])
    .rpc();
    // const tx = await program.methods.initialize().rpc();

    console.log("Your transaction signature", tx);
  });
});
