lint:
	echo "===> Linting"
	cargo clippy

fmt: 
	echo "===> Formatting"
	cargo fmt

doc:
	cargo doc --no-deps --all-features

build:
	echo "===> Building"
	cargo build --release

test:
	echo "===> Testing"
	cargo test

pr-ready: lint fmt doc build test
