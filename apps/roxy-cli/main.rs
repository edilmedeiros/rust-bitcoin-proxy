mod cli;
use clap::Parser;
use cli::Cli;
use cli::Commands;
use roxy::rpc::types::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    let roxyd_client = reqwest::Client::new();
    let address = "http://localhost:8080/proxy";

    match &cli.command {
        Commands::GetBlockchainInfo => {
            let request = RpcRequest::new("getblockchaininfo", None, 0);
            let response = roxyd_client.post(address).json(&request).send().await?;

            println!("{response:?}");
            println!("{}", response.text().await.unwrap());
        }
    }

    Ok(())
}
