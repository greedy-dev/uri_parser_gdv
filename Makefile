.DEFAULT_GOAL := build
all:
    fmt clippy test build

fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

build:
	cargo build

run:
	cargo run
