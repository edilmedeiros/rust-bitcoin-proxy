use crate::cli::Args;
use crate::rpc;
use roxy::error::*;
use roxy::rpc::client::*;
use std::process::exit;
use std::sync::Arc;

pub async fn run(cli: &Args) -> Result<Arc<JsonRpcClient<BitcoindRpcTransport>>, Error> {
    let (user, pass) = rpc::read_cookie(&cli.datadir);

    let client =
        Arc::new(BitcoindClient::new(&cli.rpc_addr, rpc::get_rpc_port(cli), &user, &pass).unwrap());

    // TODO: Timeout could not connect and report to the user
    match client.call_method("getblockchaininfo", None).await {
        Ok(_) => println!("Pinged to Bitcoin Core!"),
        Err(e) => {
            println!("Can't connect to Bitcoin Core: {}", e);
            exit(-1);
        }
    }

    Ok(client)
}
