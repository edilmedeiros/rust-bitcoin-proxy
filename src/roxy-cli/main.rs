mod cli;
use clap::Parser;
use cli::Cli;
use cli::Commands;
use reqwest;
use roxy::json_rpc_types::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    let roxyd_client = reqwest::Client::new();
    let address = "http://127.0.0.1:8080/proxy";

    match &cli.command {
        Commands::GetBlockchainInfo => {
            let request = RpcRequest::new("testing", None, 0);
            let response = roxyd_client.post(address).json(&request).send().await?;

            println!("{response:?}");
        }
    }

    Ok(())
}
