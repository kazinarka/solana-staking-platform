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
    clusterApiUrl,
} from '@solana/web3.js';
import {deserialize, serialize} from "borsh"
import {TOKEN_PROGRAM_ID, Metadata} from "@solana/spl-token";
import { Metaplex, Nft } from '@metaplex-foundation/js'

import {Chain} from "./chain";
import { programs } from '@metaplex/js'
const { metadata: { Metadata } } = programs
const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
)
export const DAY = 24 * 60 * 60;
export const M = 130.171446306642;
export const X = 2;
export const NFT_AMOUNT = 3500;

//TODO
//rename to camel-case
//refactoring
//think about fun orginizing
//TODO !!!
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
    nfts: Nft[];
    expectedUserInterest: number;
    staked: number;

    constructor(
        expectedInterests: number[],
        stakingPeriods: number[],
        nfts: Nft[],
        expectedUserInterest: number,
        staked: number,
    ) {
        this.expectedInterests = expectedInterests;
        this.stakingPeriods = stakingPeriods;
        this.nfts = nfts;
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
        this.connection = new Connection(clusterApiUrl('devnet'))
        this.chain = new Chain(this.connection)
    }

    public async getStakingPageInfo(owner: PublicKey): Promise<StakingPageInfo> {
        let nfts: Nft[] = await this.getStakedNftsForOwner(owner);
        let expectedInterests: number[] = [];
        let stakingPeriods: number[] = [];
        let expectedUserInterest: number = 0;
        let staked: number = nfts.length;

        for (const nft of nfts) {
            let expectedInterest = await this.getExpectedInterest(nft.mint);
            expectedInterests.push(expectedInterest);
            stakingPeriods.push(await this.getStakePeriod(nft.mint));
            expectedUserInterest += expectedInterest;
        }

        return new StakingPageInfo(expectedInterests, stakingPeriods, nfts, expectedUserInterest, staked)
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
        const periods = time_in_stake / DAY;

        return periods * 0.075
    }

    public async getWalletPixelNFTs(pubkey: PublicKey): Promise<Nft[]> {
        const allWalletNFTs = await Metaplex.make(this.connection).nfts().findAllByOwner(pubkey)
        return allWalletNFTs.filter((nft) => nft.collection?.key.toJSON() === process.env.REACT_APP_NFT_COLLECTION_KEY)
    }

    public async getVault(): Promise<PublicKey> {
        const result = await PublicKey.findProgramAddress([new Buffer('vault')], this.programId)
        result[0]
    }

    public async getStakedNftsForOwner(
        owner : PublicKey,
    ): Promise<Nft[]> {
        const vault = await this.getVault()
        const Nfts: Nft[] = await this.getWalletPixelNFTs(vault)

        let ownerNfts: Nft[] = []
        for (const nft of Nfts) {
            const stakeInfo = await this.getStakeInfo(nft.mint)
            if (stakeInfo != undefined) {
                if (stakeInfo.staker.toString() == owner.toString()) {
                    ownerNfts.push(nft)
                }
            }
        }
        return ownerNfts
    }

    public async getStakedNftsAmount(): Promise<number> {
        const vault = await this.getVault()
        const Nfts: Nft[] = await this.getWalletPixelNFTs(vault)


        return Nfts.length
    }

    public getStakedNftsSupply(
        amount : number,
    ): number {
        return amount / (NFT_AMOUNT / 100)
    }

    public async getStakedNftsAmountAndSupply(): Promise<[number, number]> {
        let amount = await this.getStakedNftsAmount()

        return [amount, this.getStakedNftsSupply(amount)]
    }
}