clean:
	cargo clean

lint:
	cargo clippy --all

signer:
	cargo build --release --manifest-path bls-remote-signer/Cargo.toml

test:
	cargo test --all
	cargo test --all --release

udeps:
	cargo +nightly udeps --tests --all-targets --release
