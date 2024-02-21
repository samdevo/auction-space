import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../target/types/auction_space";
import { PublicKey } from "@solana/web3.js";

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

async function newPublisher() {
  const myWallet = anchor.web3.Keypair.generate();
  const tx1 = await program.provider.connection.confirmTransaction(
    await program.provider.connection.requestAirdrop(
      myWallet.publicKey,
      10000000000
    ),
    "confirmed"
  );
  const [publisherPDA, _] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("publisher"),
      myWallet.publicKey.toBuffer(),
    ],
    program.programId
  );
  const tx = await program.methods
    .newPublisher()
    .accounts({
      publisher: publisherPDA,
      user: myWallet.publicKey,
    })
    .signers([myWallet])
    .rpc();
  return [myWallet, publisherPDA];
}
