test-stake:
	cd program; cargo test-bpf --test stake

test-unstake:
	cd program; cargo test-bpf --test unstake

test-claim:
	cd program; cargo test-bpf --test claim

test-generate-vault:
	cd program; cargo test-bpf --test generate_vault

test-add-to-whitelist:
	cd program; cargo test-bpf --test add_to_whitelist

test-reward:
	cd program; cargo test-bpf --test reward_calculation

test: test-generate-vault test-add-to-whitelist test-stake test-claim test-unstake test-reward

build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo  fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: test fmt lint
	cd program; cargo build-bpf