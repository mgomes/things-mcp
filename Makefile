.PHONY: test build run

test:
	cargo test

build:
	cargo build --release

run:
	cargo run -- $(ARGS)
