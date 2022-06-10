import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";
import fs from "fs";
import * as anchor from "@project-serum/anchor";

anchor.setProvider(anchor.AnchorProvider.env());
const program = anchor.workspace.PixelStaking;
const connection = anchor.getProvider().connection;
const userWallet = anchor.workspace.PixelStaking.provider.wallet;

const randomPayer = async (lamports = LAMPORTS_PER_SOL) => {
    const wallet = Keypair.generate();
    const signature = await connection.requestAirdrop(wallet.publicKey, lamports);
    await connection.confirmTransaction(signature);
    return wallet;
}

const findTokenMintAuthorityPDA = async (): Promise<[PublicKey, number]> => {
    return await getProgramDerivedAddress(tokenMintAddress);
}

const findStakeMintAuthorityPDA = async (): Promise<[PublicKey, number]> => {
    return await getProgramDerivedAddress(stakeMintAddress);
}

const getProgramDerivedAddress = async (seed: PublicKey): Promise<[PublicKey, number]> => {
    return await PublicKey.findProgramAddress(
        [seed.toBuffer()],
        program.programId
    );
}

// @ts-ignore
const tokenData = JSON.parse(fs.readFileSync(".keys/token_mint.json"));
const tokenMintKeypair = Keypair.fromSecretKey(new Uint8Array(tokenData));
const tokenMintAddress = tokenMintKeypair.publicKey;

// @ts-ignore
const stakeData = JSON.parse(fs.readFileSync(".keys/nft_mint.json"));
const stakeMintKeypair = Keypair.fromSecretKey(new Uint8Array(stakeData))
const stakeMintAddress = stakeMintKeypair.publicKey;

export {
    program,
    connection,
    userWallet,
    randomPayer,
    tokenMintKeypair,
    tokenMintAddress,
    stakeMintKeypair,
    stakeMintAddress,
    findTokenMintAuthorityPDA,
    findStakeMintAuthorityPDA,
}