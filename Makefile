SHELL := /bin/bash
.POSIX:
.PHONY: help init clean
.DEFAULT_GOAL := help

help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

init: ## Initialize the development environment (setup Git hooks)
	@chmod u+x .githooks/pre-commit
	@chmod u+x .githooks/commit-msg
	@git config --local core.hooksPath .githooks/
	@cargo install cocogitto
	@echo "Environment is ready!"

clean: ## Clean development environment (remove profiling files and such)
	@cargo clean

coverage: clean ## Code coverage
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='./target/debug/cargo-test-%p-%m.profraw' cargo test
	mkdir ./target/coverage
	grcov ./target/debug --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
