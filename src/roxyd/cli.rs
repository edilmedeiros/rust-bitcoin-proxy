use clap::{Parser, ValueEnum};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Parser, ValueEnum, Debug, Clone)]
pub enum Network {
    #[clap(name = "mainnet")]
    Mainnet,
    #[clap(name = "testnet")]
    Testnet,
    #[clap(name = "regtest")]
    Regtest,
    #[clap(name = "signet")]
    Signet,
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let something = match self {
            Network::Mainnet => "mainnet".to_string(),
            Network::Testnet => "testnet".to_string(),
            Network::Regtest => "regtest".to_string(),
            Network::Signet => "signet".to_string(),
        };
        write!(f, "{}", something)
    }
}

const DEFAULT_ADDRESS: &str = "localhost";
const DEFAULT_DATADIR: &str = ".datadir";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to datadir directory
    #[arg(long, value_name = "dir", default_value = DEFAULT_DATADIR)]
    pub datadir: String,

    /// Network    
    #[arg(short, long, value_name = "net", default_value_t = Network::Mainnet)]
    pub network: Network,

    /// RPC Port
    #[arg(long, value_name = "port")]
    pub rpc_port: Option<u16>,

    /// RPC Address
    #[arg(long, value_name = "ip", default_value = DEFAULT_ADDRESS)]
    pub rpc_addr: String,

    /// Roxyd Port
    #[arg(long, value_name = "port", default_value_t = 8080)]
    pub roxy_port: u16,

    /// Roxyd Bind
    #[arg(long, value_name = "ip", default_value = DEFAULT_ADDRESS)]
    pub roxy_bind: String,

    /// Debug
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Roxyd Daemon
    #[arg(short = 'b', long, default_value_t = false)]
    pub daemon: bool,
}
