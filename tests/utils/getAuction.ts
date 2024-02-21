import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";
import getWallets from "./getWallets";
import { PublicKey } from "@solana/web3.js";

import requestAirdrops from "./requestAirdrops";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

export default async function getAuction(
    wallet: anchor.web3.Keypair,
    title: string,
    minBid: number,
    startTime: number,
    endTime: number,
    effectStart: number,
    effectEnd: number
): Promise<anchor.Address> {
    const [publisherPDA, _] = PublicKey.findProgramAddressSync(
        [Buffer.from("publisher"), wallet.publicKey.toBuffer()],
        program.programId
    )
    return program.methods.newAuction(
        title,
        new anchor.BN(minBid),
        new anchor.BN(startTime),
        new anchor.BN(endTime),
        new anchor.BN(effectStart),
        new anchor.BN(effectEnd)
    ).accounts({
        publisherWallet: wallet.publicKey,
        publisher: publisherPDA
    }).signers([wallet])
    .rpc()
    .then(() => program.account.publisher.fetch(publisherPDA))
    .then((publisher) => {
        const seedBuffer = Buffer.alloc(8);
        seedBuffer.writeUInt32LE(publisher.numAuctions.toNumber() - 1, 0);
        const [auctionPDA, _] = PublicKey.findProgramAddressSync(
            [Buffer.from("auction"), wallet.publicKey.toBuffer(), seedBuffer],
            program.programId
        );
        return auctionPDA;
    })
}