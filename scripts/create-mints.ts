import { Keypair, PublicKey } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import {
    tokenMintKeypair,
    stakeMintKeypair,
    connection,
    randomPayer,
    findStakeMintAuthorityPDA,
} from "./config";

const createMints = async () => {
    const tokenMintAddress = await createMintAcct(
        tokenMintKeypair,
        tokenMintKeypair.publicKey
    )

    const [stakePDA, _] =  await findStakeMintAuthorityPDA();

    const stakeMintAddress = await createMintAcct(
        stakeMintKeypair,
        stakePDA)

    console.log(`Token Mint Address: ${tokenMintAddress}`);
    console.log(`Stake Mint Address: ${stakeMintAddress}`);
}

const createMintAcct = async (keypairToAssign: Keypair, authorityToAssign: PublicKey): Promise<PublicKey> => {
    return await createMint(
        connection,
        await randomPayer(),
        authorityToAssign,
        null,
        8,
        keypairToAssign
    );
}

export {
    createMints
}