import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

export default function requestAirdrops(
  wallets: anchor.web3.Keypair[],
  amount_per_wallet: number
): Promise<anchor.web3.Keypair[]> {
  // make a list of promises to request airdrops
  const promises = wallets.map((wallet) => {
    return program.provider.connection
      .requestAirdrop(wallet.publicKey, amount_per_wallet)
      .then((res) => {
        return program.provider.connection.confirmTransaction(res, "confirmed");
      })
      .then(() => {
        return wallet;
      });
  });

  return Promise.all(promises);
}
