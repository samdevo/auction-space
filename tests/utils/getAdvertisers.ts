import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";
import getWallets from "./getWallets";
import requestAirdrops from "./requestAirdrops";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

type KeyPairAddressPair = [anchor.web3.Keypair, anchor.Address];

export default async function getAdvertisers(
  numAdvertisers: number
): Promise<KeyPairAddressPair[]> {
  const wallets = await getWallets(numAdvertisers);
  const promises = wallets.map((wallet) => {
    return requestAirdrops([wallet], 10000000000)
      .then(() => {
        return program.methods
          .newAdvertiser()
          .accounts({
            advertiserWallet: wallet.publicKey,
          })
          .signers([wallet])
          .rpc();
      })
      .then(() => {
        const [advertiserPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("advertiser"), wallet.publicKey.toBuffer()],
          program.programId
        );
        return [wallet, advertiserPDA] as KeyPairAddressPair;
      });
  });
  return Promise.all(promises);
}
