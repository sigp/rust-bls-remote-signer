# Performs a `cargo` clean and cleans the `ef_tests` directory.
clean:
	cargo clean

# Runs cargo-fmt (linter).
cargo-fmt:
	cargo fmt --all -- --check

# Lints the code.
lint:
	cargo clippy --all

# Builds a binary in target/release.
signer:
	cargo build --release --manifest-path bls-remote-signer/Cargo.toml

# Runs test for both debug and release.
test:
	cargo test --all
	cargo test --all --release

# Checks for unused dependencies.
udeps:
	cargo +nightly udeps --tests --all-targets --release
