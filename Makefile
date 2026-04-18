.PHONY: fmt lint test doc docs-serve check

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace

doc:
	cargo doc --workspace --no-deps

docs-serve:
	mdbook serve book --open

check: fmt lint test
