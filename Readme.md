# Staking Setup + Commands

## Reward token

- NOTE: Only have to do this in development. In production, the reward token should already exist and you just have to transfer some into the vault later.

`spl-token create-token --decimals 0`

- NOTE: Any decimal spl token will work. Just using 0 for development purposes.

spl-token create-account <mint>

- NOTE: replace <mint> with the returned mint address from above

`spl-token mint <mint> 1000000`

## Set admin and reward_mint variables in `program/src/consts.rs`

`cd program && cargo build-bpf`

- NOTE: If `cargo build-bpf` doesn't work for you, run `rm -rf ~/.cache/solana` and then re-run the build command again. This should force solana to re-download and link the bpf utilities.

## Deployment will cost about 3.31 sol

`solana program deploy /path/to/nft-staking/program/target/deploy/staking.so`

## Set reward_mint and program_id variables in `client/src/main.rs`

## Run commands below in `rust-client` directory

`cargo build`

## Generate vault and transfer reward tokens into the vault

`cargo run -- generate_vault_address -e dev -s /path/to/deployer/id.json`

`spl-token transfer <reward_mint> 1000000 <vault-address> --fund-recipient`

- NOTE: second address is the vault address returned from `generate_vault_address` cmd

## Add creator ID to whitelist

`cargo run -- add_to_whitelist -e dev -s /path/to/deployer/id.json --creator <creator-address>`

- `<creator-address>` is the first creator address on the NFTs in your collection. This should be a creator with 0% share.

## Client commands

`cargo run -- stake -e dev -s /path/to/deployer/id.json --nft <nft-token-mint-address>`

- Stakes your NFT into the program vault

`cargo run -- stake3 -e dev -s /home/ideasoft/.config/solana/testnet-pixel-painters.json --nft1 <nft1-token-mint-address> --nft2 <nft2-token-mint-address> --nft3 <nft3-token-mint-address>`

- Stakes 3 your NFTs into the program vault

`cargo run -- claim -e dev -s /path/to/deployer/id.json --nft <nft-token-mint-address>`

- "Claims" your tokens on your nft without unstaking

`cargo run -- unstake -e dev -s /path/to/deployer/id.json --nft <nft-token-mint-address>`

- Unstakes your NFT and claims tokens at the same time