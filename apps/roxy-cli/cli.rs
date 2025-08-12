use clap::{Parser, Subcommand};
use std::path::PathBuf;

const DEFAULT_TLS_CERT_PATH: &str = "certs/cert.pem";
const DEFAULT_TLS_KEY_PATH: &str = "certs/key.pem";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// TLS certificate file path
    #[arg(long, env = "ROXYCLI_TLS_CERT_PATH", default_value = DEFAULT_TLS_CERT_PATH)]
    pub tls_cert_path: PathBuf,

    /// TLS key file path
    #[arg(long, env = "ROXYCLI_TLS_KEY_PATH", default_value = DEFAULT_TLS_KEY_PATH)]
    pub tls_key_path: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get current blockchain info
    #[command(name = "getblockchaininfo")]
    GetBlockchainInfo,
}
