default: build

build:
    cargo build --release

test:
    cargo test

lint:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

run *args:
    cargo run -- {{args}}

install:
    cargo install --path .
