.PHONY: all test fmt fmt-check lint build check clean

all: check

test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets -- -D warnings

# soroban-sdk 29 requires the Stellar CLI (v25.2+) to build WASM.
build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

# Mirrors .github/workflows/ci.yml
check: fmt-check lint test build

clean:
	cargo clean
