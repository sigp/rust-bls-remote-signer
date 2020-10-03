# Builds the Remote Signer binary in release (optimized).
#
# Binaries will most likely be found in `./target/release`
install:
ifeq ($(PORTABLE), true)
	cargo build --release --manifest-path bls-remote-signer/Cargo.toml --features "portable"
else
	cargo build --release --manifest-path bls-remote-signer/Cargo.toml
endif

# Builds the Remote Signer binary in release (optimized),
# using the slower Milagro BLS library, which is written in native Rust.
build-milagro:
	cargo build --release --manifest-path bls-remote-signer/Cargo.toml --features "milagro"

# Performs a `cargo` clean.
clean:
	cargo clean

# Runs cargo-fmt (linter).
cargo-fmt:
	cargo fmt --all -- --check

# Lints the code.
lint:
	cargo clippy --all

# Runs the entire test suite.
test-full: cargo-fmt test-release test-debug test-milagro

# Runs the full workspace tests in **release**.
test:
	cargo test --all --release

# Runs the full workspace tests in **release**.
test-release:
	cargo test --all --release

# Runs the full workspace tests in **debug**.
test-debug:
	cargo test --all

test-milagro:
	cargo test --manifest-path bls-remote-signer/Cargo.toml --all --features "milagro"

# Checks for unused dependencies.
udeps:
	cargo +nightly udeps --tests --all-targets --release
