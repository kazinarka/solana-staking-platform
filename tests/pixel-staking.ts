import { expect } from 'chai';
import * as anchor from "@project-serum/anchor";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
    stakeMintAddress,
    tokenMintAddress,
    program,
    findStakeMintAuthorityPDA
} from "../scripts/config"
import { User } from "./user";
import { createMints } from "../scripts/create-mints";
import { airdropToken } from "../scripts/airdrop-token";
import { TokenHelper } from "./token_helper";

describe("pixel-staking", () => {

    before(async () => {
        await createMints();
        await airdropToken();
    });

    it('It creates the program token bag', async () => {
        const user = new User();
        const [tokenPDA, _] = await getProgramTokenBagPDA();

        await program.rpc.createTokenBag({
            accounts: {
                tokenMint: tokenMintAddress,
                programTokenBag: tokenPDA,
                payer: user.wallet.publicKey,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            }
        });

        const tokenHelper = new TokenHelper(tokenMintAddress);
        expect(await tokenHelper.balance(tokenPDA)).to.be.eql(0);
    });

    it('It swaps token for stake', async () => {
        const user =  new User();
        await user.getOrCreateStakeTokenBag();
        await user.getOrCreateTokenBag()
        const userStakes = await user.stakeBalance();
        const userTokens = await user.tokenBalance();

        const [stakePDA, stakePDABump] = await findStakeMintAuthorityPDA();
        const [tokenBagPDA, tokenBagBump] = await getProgramTokenBagPDA();

        await program.rpc.stake(
            stakePDABump,
            tokenBagBump,
            new anchor.BN(5_000),
            {
                accounts: {
                    tokenProgram: TOKEN_PROGRAM_ID,
                    stakeMint: stakeMintAddress,
                    stakeMintAuthority: stakePDA,
                    userStakeTokenBag: user.stakeTokenBag,
                    userTokenBag: user.tokenBag,
                    userTokenBagAuthority: user.wallet.publicKey,
                    programTokenBag: tokenBagPDA,
                    tokenMint: tokenMintAddress,
                },
            },
        );

        expect(await user.stakeBalance()).to.be.eql(userStakes + 5_000);
        expect(await user.tokenBalance()).to.be.eql(userTokens - 5_000);
        const tokenHelper = new TokenHelper(tokenMintAddress);
        expect(await tokenHelper.balance(tokenBagPDA)).to.be.eql(5_000)
    });

    it('It redeems stake for token', async () => {
        const user = new User();
        await user.getOrCreateStakeTokenBag();
        await user.getOrCreateTokenBag()
        const [tokenBagPDA, tokenBagBump] = await getProgramTokenBagPDA();
        const userStakes = await user.stakeBalance();
        const userTokens = await user.tokenBalance();

        await program.rpc.unstake(
            tokenBagBump,
            new anchor.BN(5_000),
            {
                accounts: {
                    tokenProgram: TOKEN_PROGRAM_ID,
                    stakeMint: stakeMintAddress,
                    userStakeTokenBag: user.stakeTokenBag,
                    userStakeTokenBagAuthority: user.wallet.publicKey,
                    programTokenBag: tokenBagPDA,
                    userTokenBag: user.tokenBag,
                    tokenMint: tokenMintAddress,
                },
            }
        );

        expect(await user.stakeBalance()).to.be.eql(userStakes - 5_000);
        expect(await user.tokenBalance()).to.be.eql(userTokens + 5_000);
    });
})

const getProgramTokenBagPDA = async (): Promise<[PublicKey, number]> => {
    const seed = tokenMintAddress;

    return await PublicKey.findProgramAddress(
        [seed.toBuffer()],
        program.programId
    );
}