import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";
import getWallets from "./getWallets";
import requestAirdrops from "./requestAirdrops";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

type KeyPairAddressPair = [anchor.web3.Keypair, anchor.Address];

export default async function getPublishers(
  numPublishers: number
): Promise<KeyPairAddressPair[]> {
  const wallets = await getWallets(numPublishers);
  const promises = wallets.map((wallet) => {
    return requestAirdrops([wallet], 10000000000)
      .then(() => {
        return program.methods
          .newPublisher()
          .accounts({
            publisherWallet: wallet.publicKey,
          })
          .signers([wallet])
          .rpc();
      })
      .then(() => {
        const [PublisherPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("publisher"), wallet.publicKey.toBuffer()],
          program.programId
        );
        return [wallet, PublisherPDA] as KeyPairAddressPair;
      });
  });
  return Promise.all(promises);
}
