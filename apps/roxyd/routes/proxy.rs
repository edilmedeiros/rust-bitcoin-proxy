use actix_web::{post, web, HttpResponse};
use roxy::error::*;
use roxy::rpc::client::BitcoindClient;
use roxy::rpc::types::*;

// TODO: share client between services
#[post("/proxy")]
async fn route(
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
