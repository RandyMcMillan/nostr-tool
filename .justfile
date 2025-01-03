alias d := doc
alias l := lint
alias uf := update-flake-dependencies
alias uc := update-cargo-dependencies
alias r := run
alias t := cargo-test
alias b := build
alias c := check
alias i := install
alias ia := install-all
alias in := install-ngit
alias rr := run-release
alias cw := cargo-watch

default:
    @just --choose || true

clippy:
    cargo clippy --all-targets --all-features

actionlint:
    nix develop .#actionlintShell --command actionlint

deny:
    cargo deny check

cargo-test:
    cargo test

cargo-diet:
    nix develop .#lintShell --command cargo diet

cargo-tarpaulin:
    nix develop .#lintShell --command cargo tarpaulin --out html --exclude-files "benches/*"

cargo-public-api:
    nix develop .#lintShell --command cargo public-api

cargo-diff:
    nix develop .#lintShell --command cargo public-api diff

lint:
    nix develop .#lintShell --command cargo diet
    nix develop .#lintShell --command cargo deny check licenses sources
    nix develop .#lintShell --command typos
    nix develop .#lintShell --command lychee *.md
    nix develop .#fmtShell --command treefmt --fail-on-change
    nix develop .#lintShell --command cargo udeps
    nix develop .#lintShell --command cargo machete
    nix develop .#lintShell --command cargo outdated
    nix develop .#lintShell --command taplo lint
    nix develop .#actionlintShell --command actionlint --ignore SC2002
    cargo check --future-incompat-report
    nix flake check

run:
    cargo run --bin gnostr -- -h || true

install-all:install install-ngit
install:
    cargo install --path .

install-ngit:
    cargo install --path crates/ngit

build:
    cargo build --bins || true
    cargo b --manifest-path crates/ngit/Cargo.toml || true

build-examples:
    cargo build --examples || true

check:
    cargo check || true

run-release:
    cargo run --release --bin gnostr -h || true

doc:
    cargo doc --open --offline

# Update and then commit the `Cargo.lock` file
update-cargo-dependencies:
    cargo update
    git add Cargo.lock
    git commit Cargo.lock -m "update(cargo): \`Cargo.lock\`"

# Future incompatibility report, run regularly
cargo-future:
    cargo check --future-incompat-report

update-flake-dependencies:
    nix flake update --commit-lock-file

cargo-watch:
    cargo watch -x check -x test -x build

# build all examples
examples:
    nix develop --command $SHELL
    example_list=$(cargo build --example 2>&1 | sed '1,2d' | awk '{print $1}')

    # Build each example
    # shellcheck disable=SC2068
    for example in ${example_list[@]}; do
    cargo build --example "$example"
    done

examples-msrv:
    set -x
    nix develop .#msrvShell --command
    rustc --version
    cargo --version
    example_list=$(cargo build --example 2>&1 | grep -v ":")

    # Build each example
    # shellcheck disable=SC2068
    for example in ${example_list[@]}; do
    cargo build --example "$example"
    done
