import {  PublicKey } from '@solana/web3.js';
import { tokenMintAddress, stakeMintAddress, userWallet } from "../scripts/config"
import { TokenHelper } from "./token_helper";
import { Wallet } from "@project-serum/anchor";

class User {
    token: TokenHelper;
    tokenBag: PublicKey;
    stakeToken: TokenHelper;
    stakeTokenBag: PublicKey;
    wallet: Wallet;

    constructor(wallet = userWallet) {
        this.token = new TokenHelper(tokenMintAddress);
        this.stakeToken = new TokenHelper(stakeMintAddress);
        this.wallet = wallet;
    }

    getOrCreateTokenBag = async () => {
        this.tokenBag = (await this.token.getOrCreateTokenBag(this.wallet.publicKey)).address;
    }

    getOrCreateStakeTokenBag = async () => {
        this.stakeTokenBag = (await this.stakeToken.getOrCreateTokenBag(this.wallet.publicKey)).address;
    }

    tokenBalance = async () => {
        return await this.token.balance(this.tokenBag);
    }

    stakeBalance = async () => {
        return await this.token.balance(this.stakeTokenBag);
    }
}

export {
    User
}