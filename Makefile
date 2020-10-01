# Builds the Remote Signer binary in release (optimized).
#
# Binaries will most likely be found in `./target/release`
install:
	cargo build --release --manifest-path bls-remote-signer/Cargo.toml

# Performs a `cargo` clean.
clean:
	cargo clean

# Runs cargo-fmt (linter).
cargo-fmt:
	cargo fmt --all -- --check

# Lints the code.
lint:
	cargo clippy --all

# Runs test for both debug and release.
test:
	cargo test --all
	cargo test --all --release

# Checks for unused dependencies.
udeps:
	cargo +nightly udeps --tests --all-targets --release
