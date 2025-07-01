use roxy::json_rpc::*;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request = RpcRequest::new("getblockchaininfo".to_string());
    let string = serde_json::to_string(&request).unwrap();
    let ret: RpcRequest = serde_json::from_str(&string).unwrap();
    println!("{:?}", ret);

    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:8080/proxy")
        .json(&request)
        .send()
        .await?;

    println!("{response:?}");
    println!("{:?}", response.text().await?);

    Ok(())
}
