mod cli;
use clap::Parser;
use cli::{Args, Network};
use roxy::error::*;
use roxy::json_rpc_client::BitcoindClient;
// use serde_json::value::RawValue;
// use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use roxy::json_rpc_types::*;

use std::process::exit;
use std::sync::Arc;

// TODO: share client between services
#[post("/proxy")]
async fn proxy(
    bitcoind_client: web::Data<BitcoindClient>,
    json_rpc: web::Json<RpcRequest>,
) -> Result<HttpResponse, Error> {
    // TODO: add proper error handling
    println!("Hello from proxy!");
    // Steps
    // Consult the read payload and do some verification
    // Use username and password from file (read the file in main)
    // Check response from bitcoind to treat it back to roxy-cli
    let response = bitcoind_client
        .call_method(&json_rpc.method, json_rpc.params.clone())
        .await?;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&response.clone()).unwrap()))
}

fn login_with_cookie(datadir_path: &str) -> (String, String) {
    let mut full_path: String = "/regtest/.cookie".to_owned();
    full_path.insert_str(0, datadir_path);
    let (user, pass) = match std::fs::read_to_string(&full_path)
        .map_err(Error::from)
        .and_then(|s| {
            s.split_once(":")
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .ok_or(Error::Err("Can't parse cookie file".to_string()))
        }) {
        Ok(t) => t,
        Err(Error::IO(e)) => {
            println!(
                "{}: {} (Make sure you are running the bitcoin node)",
                full_path, e
            );
            exit(-1);
        }
        Err(Error::Err(e)) => {
            println!("{}", e);
            exit(-2);
        }
        _ => {
            println! {"Unexpected error..."};
            exit(-3);
        }
    };
    (user, pass)
}

fn get_rpc_port(cli: &Args) -> u16 {
    match cli.rpc_port {
        None => match cli.network {
            Network::Mainnet => 8332,
            Network::Regtest => 18443,
            Network::Testnet => 38332,
            Network::Signet => 38332,
        },
        Some(port) => port,
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Args::parse();
    if cli.debug {
        env_logger::init();
    }
    // TODO: share resources (DB, username, password) with services
    // let args: Vec<String> = env::args().collect();

    // TODO: get cookie path via cli
    let (user, pass) = login_with_cookie(&cli.datadir);

    let client =
        Arc::new(BitcoindClient::new(&cli.rpc_addr, get_rpc_port(&cli), &user, &pass).unwrap());
    match client.call_method("getblockchaininfo", None).await {
        Ok(_) => println!("Connected to Bitcoin Core!"),
        Err(e) => {
            println!("Can't connect to Bitcoin Core: {}", e);
            exit(-1);
        }
    }

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(client.clone()))
            .service(proxy)
    })
    .bind((cli.roxy_bind, cli.roxy_port))?
    .run()
    .await;

    Ok(())
}
