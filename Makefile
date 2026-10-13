# Makefile for ls (LeanSpec Parser & CAS Listing Engine)
# Standard POSIX wrapper that delegates directly to `cargo xtask` and `cargo -j $(JOBS)`.
SHELL := bash
.SHELLFLAGS := -euo pipefail -c
.DEFAULT_GOAL := help

JOBS ?= 2
export JOBS

.PHONY: help default build fmt fmt-check clippy test doc-test check ci spec-check projectfile-check toolchain-check xwork deny audit smoke doc

default: help

help: ## Show available targets
	@echo "Usage: make <target> [JOBS=2]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'

build: ## Compile the workspace with locked dependencies and host throttling
	cargo build --workspace -j $(JOBS)

fmt: ## Format all Rust code
	cargo fmt --all

fmt-check: ## Check code formatting without modifying files
	cargo fmt --all -- --check

clippy: ## Lint with clippy (warnings denied)
	cargo clippy --workspace --all-targets -j $(JOBS) -- -D warnings

test: ## Run unit and integration tests with bounded threads
	cargo test --workspace -j $(JOBS)

doc-test: ## Run documentation tests
	cargo test --doc --workspace -j $(JOBS)

check: fmt-check clippy test doc-test ## Push gate: fmt-check, clippy, test, doc-test

ci: check ## Alias for check

spec-check: ## Validate repository specifications with lsc
	cargo xtask spec-check

projectfile-check: ## Validate projectfile.yaml with lsc
	cargo xtask projectfile-check

toolchain-check: ## Verify active Rust matches rust-toolchain.toml
	cargo xtask toolchain-check

xwork: ## Full verification pipeline via cargo xtask
	cargo xtask xwork

deny: ## Audit licenses, bans, and advisories
	cargo deny check

audit: ## Check dependencies for known advisories
	cargo audit

smoke: ## Build CLI/library and verify domain crate has zero I/O
	cargo build --workspace -j $(JOBS)
	@if grep -rnE 'std::net|std::fs|tokio|std::process' crates/ls-domain/src; then \
		echo "smoke: ls-domain must not use I/O" >&2; \
		exit 1; \
	fi; \
	echo "smoke: ok"

doc: ## Build API docs
	cargo doc --workspace --no-deps -j $(JOBS)
