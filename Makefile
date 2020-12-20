## Start app
start:
	$ cargo run
.PHONY: start

## Build the app development binary
build:
	$ cargo build --release
.PHONY: build

## Build the app development binary
build-dev:
	$ cargo build
.PHONY: build-dev

## Start app in dev mode and watch for file changes
dev:
	$ cargo watch -x run
.PHONY: dev

## Run tests
test:
	$ cargo test
.PHONY: test

## Run tests and watch for changes on files
test-watch:
	$ cargo watch -x test
.PHONY: test-watch