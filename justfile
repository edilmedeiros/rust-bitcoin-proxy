# Lists all available targets
default:
    just --list

# Simulate Bitcoind
bitcoind:
    nc -l 38332

# Run (with cargo) roxyd
server:
	cargo run -- roxyd

# Run (with cargo) roxy-cli
client:
	echo "Roxy-cli is yet not implemented"

# Update Cargo.nix
update-nix:
	nix run github:cargo2nix/cargo2nix
