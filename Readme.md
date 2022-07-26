# Infrastructure

## Client
`cd client`
- Library with gettable methods.
- Server with useful endpoints for FE side
- NOTE: set variable program_id in `client/src/client.ts`

## Program
`cd program`
> program/src
- Source files for staking smart contract program

>program/tests
- Tests for all instructions in devnet cluster and for reward calculation flow

## Rust Client
`cd rust-client`
- Simple service to call SC instructions via command line

# Staking Setup + Commands

## Reward token

- NOTE: Only have to do this in development. In production, the reward token should already exist and you just have to transfer some into the vault later.

`spl-token create-token --decimals 0`

- NOTE: Any decimal spl token will work. Just using 0 for development purposes.

`spl-token create-account <mint>`

- NOTE: replace <mint> with the returned mint address from above

`spl-token mint <mint> <amount>`

## Set ADMIN and REWARD_MINT consts in `program/src/consts.rs` and  enter program id to declare_id macro in `program/src/lib.rs`

`cd program && cargo build-bpf`

- NOTE: If `cargo build-bpf` doesn't work for you, run `rm -rf ~/.cache/solana` and then re-run the build command again. This should force solana to re-download and link the bpf utilities.

## Deployment will cost 1.75046512 sol

`solana program deploy /path/to/nft-staking/program/target/deploy/staking.so`

## Set REWARD_MINT and PROGRAM_ID consts in `rust-client/src/consts.rs`

## Run commands below in `rust-client` directory

`cargo build`

- NOTE: if you want to call devnet contract, just add `-e dev` to commands in command line

## Generate vault and transfer reward tokens into the vault

`cargo run -- generate_vault_address -s /path/to/deployer/id.json`

`spl-token transfer <reward_mint> <amount> <vault-address> --fund-recipient`

- NOTE: second address is the vault address returned from `generate_vault_address` cmd

## Add creator ID to whitelist

`cargo run -- add_to_whitelist -s /path/to/deployer/id.json --creator <creator-address>`

- `<creator-address>` is the first creator address on the NFTs in your collection. This should be a creator with 0% share.

## Client commands

`cargo run -- stake -s /path/to/deployer/id.json --nft <nft-token-mint-address>`

- Stakes your NFT into the program vault

`cargo run -- claim -s /path/to/deployer/id.json --nft <nft-token-mint-address>`

- "Claims" your tokens on your nft without unstaking

`cargo run -- unstake -s /path/to/deployer/id.json --nft <nft-token-mint-address>`

- Unstakes your NFT and claims tokens at the same time