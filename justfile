# Lists all available targets
default:
    just --list

# Build all executables
[group('Nix')]
build:
	nix build

[group('Rust')]
clippy:
	cargo clippy

# Run Bitcoind
[group('Bitcoin')]
bitcoind *ARGS:
	bitcoind -regtest -datadir=.datadir {{ARGS}}

# Run Bitcoin-cli
[group('Bitcoin')]
bcli *ARGS:
	bitcoin-cli -regtest -datadir=.datadir {{ARGS}}

# Run (with cargo) roxyd
[group('Roxy')]
roxyd *ARGS:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run --bin roxyd -- -d --network=regtest {{ARGS}}

# Run (with cargo) roxy-cli
[group('Roxy')]
rcli *ARGS:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run --bin roxy-cli -- {{ARGS}}

# Run rust formatter
[group('Rust')]
format:
	cargo fmt

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
	bacon --job run -- roxyd -- {{ ARGS }}
