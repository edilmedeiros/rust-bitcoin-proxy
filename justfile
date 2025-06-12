# Lists all available targets
default:
    just --list

# Simulate Bitcoind
bitcoind:
    nc -l 38332

# Run (with cargo) roxyd
server *ARGS:
	cargo run -- roxyd {{ARGS}}

# Run rust formatter
format:
	cargo fmt

# Run (with cargo) roxy-cli
client:
	echo "Roxy-cli is yet not implemented"

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'bacon' to run the project (auto-recompiles)
watch *ARGS:
	bacon --job run -- roxyd -- {{ ARGS }}
