import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AuctionSpace } from "../../target/types/auction_space";
import getWallets from "./getWallets";
import { PublicKey } from "@solana/web3.js";

import requestAirdrops from "./requestAirdrops";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.AuctionSpace as Program<AuctionSpace>;

export default async function getItem(
    wallet: anchor.web3.Keypair,
    title: string,
    url: string,
): Promise<anchor.Address> {
    const [publisherPDA, _] = PublicKey.findProgramAddressSync(
        [Buffer.from("publisher"), wallet.publicKey.toBuffer()],
        program.programId
    )
    return program.methods.newItem(
        title,
        url
    ).accounts({
        publisherWallet: wallet.publicKey,
        publisher: publisherPDA
    }).signers([wallet])
    .rpc()
    .then(() => program.account.publisher.fetch(publisherPDA))
    .then((publisher) => {
        const seedBuffer = Buffer.alloc(8);
        seedBuffer.writeUInt32LE(publisher.numItems.toNumber() - 1, 0);
        const [itemPDA, _] = PublicKey.findProgramAddressSync(
            [Buffer.from("item"), wallet.publicKey.toBuffer(), seedBuffer],
            program.programId
        );
        // return program.account.item.fetch(itemPDA);
        return itemPDA;
    })
}