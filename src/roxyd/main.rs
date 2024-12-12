use roxy::error::*;
use roxy::http_client::HttpClient;
// use serde_json::value::RawValue;
// use std::env;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use roxy::json_rpc::*;
use std::sync::Arc;

// TODO: share client between services
#[post("/proxy")]
async fn proxy(
    json_rpc: web::Json<RpcRequest>,
    client: web::Data<HttpClient>,
) -> Result<HttpResponse, Error> {
    // TODO: add proper error handling

    // Steps
    // Consult the read payload and do some verification
    // Use username and password from file (read the file in main)
    // Check response from bitcoind to treat it back to roxy-cli

    let response = client
        .call_method(&json_rpc.method, json_rpc.params.clone())
        .await?;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&response.clone()).unwrap()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: share resources (DB, username, password) with services
    // let args: Vec<String> = env::args().collect();

    let client = Arc::new(HttpClient::new("http://127.0.0.1:38332", "foo", "bar").unwrap());

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(proxy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
