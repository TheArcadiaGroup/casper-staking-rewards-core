prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc20 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20.wasm
	cargo build --release -p staking-rewards --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/staking_rewards.wasm

test-only:
	cargo test -p tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/erc20.wasm tests/wasm
	cp target/wasm32-unknown-unknown/release/staking_rewards.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
	
clean:
	cargo clean
	rm -rf tests/wasm/erc20.wasm
	rm -rf tests/wasm/staking_rewards.wasm