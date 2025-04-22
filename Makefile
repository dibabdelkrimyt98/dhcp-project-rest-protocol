.PHONY: build run clean test

build:
	cargo build

run:
	cargo run

clean:
	cargo clean

test:
	cargo test

run-client:
	cargo run --bin client

run-server:
	cargo run --bin server 