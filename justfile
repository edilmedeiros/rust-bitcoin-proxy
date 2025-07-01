# Lists all available targets
default:
    just --list

# Simulate Bitcoind
bitcoind *ARGS:
	bitcoind -regtest -datadir=datadir {{ARGS}}

bcli *ARGS:
	bitcoin-cli -regtest -datadir=datadir {{ARGS}}

# Run (with cargo) roxyd
server *ARGS:
	cargo run -- roxyd {{ARGS}}

# Run (with cargo) roxy-cli
cli *ARGS:
	cargo run -- roxy-cli {{ARGS}}

# Run rust formatter
format:
	cargo fmt

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
	bacon --job run -- roxyd -- {{ ARGS }}
