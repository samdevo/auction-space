import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";
import getAdvertisers from "./getAdvertisers";
import getPublishers from "./getPublishers";
import getAuction from "./getAuction";
import getWallets from "./getWallets";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

describe("util-test", () => {
  it("newAdvertiserUtil", async () => {
    const publishers = await getAdvertisers(10);
    const PDA = publishers[0][1];
    const advertiser = program.account.advertiser.fetch(PDA);
  });
  it("newPublisherUtil", async () => {
    const publishers = await getPublishers(10);
    const PDA = publishers[0][1];
    const publisher = program.account.publisher.fetch(PDA);
  });
  it("newAuctionUtil", async () => {
    const publishers = await getPublishers(1);
    const [wallet, _] = publishers[0];
    const curSeconds = new Date().getTime() / 1000;
    const auctionPDA = await getAuction(
      wallet,
      "testAuction",
      curSeconds,
      curSeconds + 100,
      curSeconds + 200,
      curSeconds + 300,
      curSeconds + 400,
    );
    const auction = program.account.auction.fetch(auctionPDA);
  });
});
