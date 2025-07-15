mod cli;
use clap::Parser;
use cli::Cli;
use cli::Commands;
use roxy::rpc::types::*;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();

    let mut buf = Vec::new();
    File::open("config/cert.pem")
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let cert = reqwest::Certificate::from_pem(&buf)?;

    let roxyd_client = reqwest::Client::builder()
        .add_root_certificate(cert)
        .build()?;

    let address = "https://localhost:8080/proxy";

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
