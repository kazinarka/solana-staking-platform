test:
	cd program; cargo test-bpf --features "test-bpf" -- --nocapture --test-threads=4

build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo  fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: test fmt lint
	cd program; cargo build-bpf