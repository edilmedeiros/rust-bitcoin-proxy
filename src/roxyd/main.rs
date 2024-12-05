use roxy::error::*;
// use serde_json::value::RawValue;
// use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use roxy::http_client::HttpClient;

// Miner pipeline:
// $CLI -signet getblocktemplate '{"rules": ["signet","segwit"]}' |
//   $MINER --cli="$CLI" genpsbt --address="$ADDR" |
//   $CLI -signet -stdin walletprocesspsbt |
//   jq -r .psbt |
//   $MINER --cli="$CLI" solvepsbt --grind-cmd="$GRIND" |
//   $CLI -signet -stdin submitblock

// TODO: share client between services
#[get("/foo")]
async fn foo() -> impl Responder {
    // TODO: add proper error handling

    // args[1]: rpc user
    // args[2]: rpc password
    // TODO: proper user input

    let client = HttpClient::new("http://127.0.0.1:38332", "foo", "bar").unwrap();

    let response = client.call_method("baz", None).await.unwrap();

    println!("{}", serde_json::to_string(&response.clone()).unwrap());

    HttpResponse::Ok().body(serde_json::to_string(&response.clone()).unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: share resources (DB, username, password) with services
    // let args: Vec<String> = env::args().collect();

    let _ = HttpServer::new(|| App::new().service(foo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;

    Ok(())
}
