# Lists all available targets
default:
    just --list

# Build all executables
build:
	nix build

# Run Bitcoind
bitcoind *ARGS:
	bitcoind -regtest -datadir=.datadir {{ARGS}}

# Run Bitcoin-cli
bcli *ARGS:
	bitcoin-cli -regtest -datadir=.datadir {{ARGS}}

# Run (with cargo) roxyd
roxyd *ARGS:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run --bin roxyd -- {{ARGS}}

# Run (with cargo) roxy-cli
rcli *ARGS:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run --bin roxy-cli -- {{ARGS}}

# Run rust formatter
format:
	cargo fmt

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
	bacon --job run -- roxyd -- {{ ARGS }}
