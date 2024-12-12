use reqwest::{Client, StatusCode, Url};
use serde_json::value::RawValue;
use std::sync::atomic;

use crate::error::*;
use crate::json_rpc::*;

#[derive(Debug)]
pub struct HttpClient {
    client: reqwest::Client,
    url: Url,
    user: String,
    pass: String,
    nonce: atomic::AtomicUsize,
}

impl HttpClient {
    pub fn new(url: &str, user: &str, pass: &str) -> Result<HttpClient, Error> {
        let parsed_url =
            Url::parse(url).map_err(|parse_error| Error::Url(parse_error.to_string()))?;
        Ok(HttpClient {
            client: Client::new(),
            url: parsed_url,
            user: user.to_owned(),
            pass: pass.to_owned(),
            nonce: atomic::AtomicUsize::new(1),
        })
    }

    pub async fn call_method(
        &self,
        method: &str,
        params: Option<Box<RawValue>>,
    ) -> Result<RpcResponse, Error> {
        let request_id = self.nonce.fetch_add(1, atomic::Ordering::Relaxed);
        let request = RpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: Some(request_id),
            method: method.to_owned(),
            params,
        };

        let http_response = self
            .client
            .post(self.url.clone())
            .basic_auth(self.user.clone(), Some(self.pass.clone()))
            .json(&request)
            .header("content-type", "application/json")
            .send()
            .await?;

        // TODO: Better error treatment for other statuses code
        match http_response.status() {
            StatusCode::OK => {
                let payload = http_response.json::<RpcResponse>().await?;
                let expected_version = "2.0".to_owned();
                match (
                    payload
                        .id
                        .as_ref()
                        .and_then(|value| value.as_u64().map(|v| v as usize)),
                    payload.jsonrpc.as_ref(),
                ) {
                    // Before bitcoind v28.0, this match will probably fail.
                    (Some(id), Some(version))
                        if id == request_id && expected_version == *version =>
                    {
                        Ok(payload)
                    }
                    (Some(id), Some(_)) if id != request_id => Err(Error::Err(
                        "Mismatched IDs between request and response".to_owned(),
                    )),
                    (None, _) => Err(Error::Err("Response lacks ID".to_owned())),
                    (_, None) => Err(Error::Err("Response lacks JSON RPC Version".to_owned())),
                    (_, Some(_)) => Err(Error::Err("Wrong JSON RPC Version".to_owned())),
                }
            }
            StatusCode::NO_CONTENT => Err(Error::Err("No content".to_owned())),
            StatusCode::BAD_REQUEST => Err(Error::Err("Bad request".to_owned())),
            StatusCode::UNAUTHORIZED => Err(Error::Err("Unauthorized".to_owned())),
            StatusCode::FORBIDDEN => Err(Error::Err("Forbidden".to_owned())),
            StatusCode::NOT_FOUND => Err(Error::Err("Not found".to_owned())),
            StatusCode::METHOD_NOT_ALLOWED => Err(Error::Err("Method Not Allowed".to_owned())),
            StatusCode::INTERNAL_SERVER_ERROR => {
                Err(Error::Err("Internal Server Error".to_owned()))
            }
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::Err("Service Unavailable".to_owned())),
            // The above are the available bitcoind statuses
            // https://github.com/bitcoin/bitcoin/blob/d6b225f1652526cb053ec32c8ff09160d5a759c5/src/rpc/protocol.h#L10
            _ => Err(Error::Err("Unreachable from Bitcoind".to_owned())),
        }
    }
}
