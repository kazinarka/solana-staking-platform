import {
    Blockhash,
    Commitment,
    Connection,
    ConnectionConfig,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TokenAmount,
    Transaction,
    TransactionInstruction,
    AccountInfo,
    Struct,
} from '@solana/web3.js';
import {deserialize, serialize} from "borsh"
import {TOKEN_PROGRAM_ID, Metadata} from "@solana/spl-token";

import {Chain} from "./chain";

export const DAY = 24 * 60 * 60;
export const M = 130.171446306642;
export const X = 2;

//TODO
//rename to camel-case
//refactoring
//think about fun orginizing
//redeploy SC
//get staked tokens

export class StakeInfo {
    timestamp: number;
    staker: PublicKey;
    mint: PublicKey;
    active: boolean;
    withdrawn: number;
    harvested: number;

    constructor(buf: Buffer) {
        let offset = 0
        this.timestamp = Number(buf.readBigUInt64LE(offset));
        offset += 8;
        this.staker = new PublicKey(buf.slice(offset, offset + 32));
        offset += 32;
        this.mint = new PublicKey(buf.slice(offset, offset + 32));
        offset += 32;
        this.active = buf.readUInt8(offset) !== 0;
        offset += 1;
        this.withdrawn = Number(buf.readBigUInt64LE(offset));
        offset += 8;
        this.harvested = Number(buf.readBigUInt64LE(offset));
    }
}

export class StakingPageInfo {
    expectedInterests: number[];
    stakingPeriods: number[];
    metadata: PublicKey[];
    expectedUserInterest: number;
    staked: number;

    constructor(
        expectedInterests: number[],
        stakingPeriods: number[],
        metadata: PublicKey[],
        expectedUserInterest: number,
        staked: number,
    ) {
        this.expectedInterests = expectedInterests;
        this.stakingPeriods = stakingPeriods;
        this.metadata = metadata;
        this.expectedUserInterest = expectedUserInterest;
        this.staked = staked;
    }
}

export class Client {
    public programId: PublicKey
    public splAssociatedTokenProgramID: PublicKey
    public connection: Connection
    public chain: Chain

    constructor() {
        this.programId = new PublicKey("A1e6srJtSSpLxce5byNnGu6Azrs29z9G2wAwgorr5yig")
        this.splAssociatedTokenProgramID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
        this.connection = new Connection("https://api.devnet.solana.com")
        this.chain = new Chain(this.connection)
    }

    public async getStakeInfo(nft: PublicKey): Promise<StakeInfo | undefined> {
        const result = await PublicKey.findProgramAddress([new PublicKey(nft).toBuffer()], this.programId)
        let acc = this.connection.getAccountInfo(result[0]);
        if (!acc) {
            return undefined;
        } else {
            return new StakeInfo(acc.data);
        }
    }

    public async getStakePeriod(nft: PublicKey): Promise<number> {
        let stakeInfo = await this.getStakeInfo(nft);
        if (stakeInfo == undefined) {
            return 0;
        }

        let now = await this.chain.timestamp();

        let time_in_stake = now - stakeInfo.timestamp;
        return time_in_stake / DAY
    }

    public async getExpectedInterest(nft: PublicKey): Promise<number> {
        let stakeInfo = await this.getStakeInfo(nft);
        if (stakeInfo == undefined) {
            return 0;
        }

        let now = await this.chain.timestamp();

        let time_in_stake = now - stakeInfo.timestamp;
        let periods = time_in_stake / DAY;

        //add calculation_flow
        let reward = 0;
        // for (var i = 0, i < periods, i++) {
        //
        // }

        return reward
    }

    public async getMetadata(nft: PublicKey): Promise<PublicKey> {
        // add Metadata
        0
    }

    public async getStakingPageInfo(nfts: PublicKey[]): Promise<StakingPageInfo> {
        let metadata: PublicKey[] = [];
        let expectedInterests: number[] = [];
        let stakingPeriods: number[] = [];
        let expectedUserInterest: number = 0;
        let staked = nfts.length;

        for (const nft of nfts) {
            metadata.push(await this.getMetadata(nft));
            let expectedInterest = await this.getExpectedInterest(nft);
            expectedInterests.push(expectedInterest);
            stakingPeriods.push(await this.getStakePeriod(nft));
            expectedUserInterest += expectedInterest;
        }

        return new StakingPageInfo(expectedInterests, stakingPeriods, metadata, expectedUserInterest, staked)
    }
}