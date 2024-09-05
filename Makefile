install: install-deps install-git-hooks
.PHONY: install

install-deps:
	cargo install --path .
.PHONY: install-deps

install-git-hooks:
	chmod +x githooks/*
	cp -f githooks/* .git/hooks/
.PHONY: install-git-hooks

format:
	cargo fmt --all --
.PHONY: format

format-check:
	cargo fmt --all -- --check
.PHONY: format-check

test:
	cargo test --all --all-features --tests
.PHONY: test

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy

doc:
	cargo doc --all-features
.PHONY: doc

all: format clippy test doc

build:
	cargo build
.PHONY: build

build-release:
	cargo build --release
.PHONY: build-release
