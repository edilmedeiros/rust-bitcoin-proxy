use clap::{Parser, ValueEnum};

#[derive(Parser, ValueEnum, Debug, Clone)]
pub enum Network {
    Mainet,
    Testnet,
    Regtest,
    Signet,
}

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Mainet => "mainet".to_string(),
            Network::Testnet => "testnet".to_string(),
            Network::Regtest => "regtest".to_string(),
            Network::Signet => "signet".to_string(),
        }
    }
}

const DEFAULT_ADDRESS: &'static str = "localhost";
const DEFAULT_DATADIR: &'static str = ".datadir";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to datadir directory
    #[arg(long, value_name = "DIR", default_value = DEFAULT_DATADIR)]
    pub datadir: String,

    /// Network    
    #[arg(short, long, value_name = "NET", default_value_t = Network::Mainet)]
    pub network: Network,

    /// RPC Port
    #[arg(long, value_name = "PORT")]
    pub rpc_port: Option<u16>,

    /// RPC Address
    #[arg(long, value_name = "IP", default_value = DEFAULT_ADDRESS)]
    pub rpc_addr: String,

    /// Roxyd Port
    #[arg(long, value_name = "PORT", default_value_t = 8080)]
    pub roxy_port: u16,

    /// Roxyd Bind
    #[arg(long, value_name = "IP", default_value = DEFAULT_ADDRESS)]
    pub roxy_bind: String,

    /// Debug
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Roxyd Daemon
    #[arg(short = 'b', long, default_value_t = false)]
    pub daemon: bool,
}
