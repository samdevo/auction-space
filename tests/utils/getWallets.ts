import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

export default async function getWallets(
  numWallets: number
): Promise<anchor.web3.Keypair[]> {
  const wallets: anchor.web3.Keypair[] = [];
  for (let i = 0; i < numWallets; i++) {
    const myWallet = anchor.web3.Keypair.generate();
    wallets.push(myWallet);
  }
  return wallets;
}
