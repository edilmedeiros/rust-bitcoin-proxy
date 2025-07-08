use crate::cli::{Args, Network};
use crate::routes::proxy;
use roxy::error::*;
use roxy::json_rpc_client::BitcoindClient;
use actix_web::{web, App, HttpServer};
use std::process::exit;
use std::sync::Arc;


pub async fn start(cli: &Args) -> Result<(), Error> {
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
    .bind((cli.roxy_bind.clone(), cli.roxy_port))?
    .run()
    .await;

    Ok(())
}
