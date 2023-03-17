default:
  just --list

alias f := fmt
alias r := run

ci: build test clippy fmt-check

build:
	cargo build

test:
	cargo test

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo fmt --all -- --check
  @echo formatting check done

run *args:
	cargo run -- --{{args}}

fmt:
	cargo fmt

check:
 cargo check

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"

usage:
	cargo run -- --help | pbcopy
