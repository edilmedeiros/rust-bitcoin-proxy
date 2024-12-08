use node::BitcoinD;
use roxy::http_client;
use roxy::json_rpc::Response;

#[tokio::test]
async fn test_get_blockchain_info() {
    // Call with `cargo test -- --nocapture` to see the print outputs.
    let bitcoind = BitcoinD::from_downloaded().unwrap();
    let rpc_url = bitcoind.rpc_url();
    let cookie = std::fs::read_to_string(&bitcoind.params.cookie_file).unwrap();
    let (user, pass) = cookie.split_once(':').unwrap();

    assert_eq!(0, bitcoind.client.get_blockchain_info().unwrap().blocks);
    println!("rpc url: {rpc_url}");
    println!("user: {user}");
    println!("pass: {pass}");

    println!("{:?}", bitcoind.client);
    println!("{:?}", bitcoind.params);

    let rpc_client = http_client::HttpClient::new(&rpc_url, user, pass).unwrap();
    let response = rpc_client.call_method("getblockchaininfo", None).await.unwrap();
    println!("{:?}", response);

    let response_json = response.result.unwrap();
    let response_fields: Vec<&str> = response_json
        .get()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split_terminator(',')
        .collect();
    println!("{response_fields:?}");
    assert_eq!(response_fields[0], "\"chain\":\"regtest\"");
    assert_eq!(response_fields[1], "\"blocks\":0");
}
