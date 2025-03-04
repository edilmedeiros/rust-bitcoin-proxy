use reqwest::{Client, StatusCode, Url};
use serde_json::value::RawValue;
use std::sync::{atomic, Arc};

use crate::error::*;
use crate::json_rpc_types::*;

#[derive(Debug, Clone)]
pub struct BitcoindRpcTransport {
    client: reqwest::Client,
    address: Url,
    user: String,
    pass: String,
}

impl BitcoindRpcTransport {
    fn new(address: Url, user: &str, pass: &str) -> Self {
        BitcoindRpcTransport {
            client: Client::new(),
            address,
            user: user.to_owned(),
            pass: pass.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonRpcClient<T: Transport> {
    nonce: Arc<atomic::AtomicUsize>,
    transport: T,
}

pub type BitcoindClient = JsonRpcClient<BitcoindRpcTransport>;

#[allow(async_fn_in_trait)]
pub trait Transport {
    fn build(address: Url, user: &str, pass: &str) -> Self;
    async fn send(&self, payload: RpcRequest) -> Result<RpcResponse, Error>;
}

impl Transport for BitcoindRpcTransport {
    fn build(address: Url, user: &str, pass: &str) -> BitcoindRpcTransport {
        BitcoindRpcTransport::new(address, user, pass)
    }

    async fn send(&self, payload: RpcRequest) -> Result<RpcResponse, Error> {
        let http_response = self
            .client
            .post(self.address.clone())
            .basic_auth(self.user.clone(), Some(self.pass.clone()))
            .json(&payload)
            .header("content-type", "application/json")
            .send()
            .await?;

        // TODO: Better error treatment for other statuses code
        match http_response.status() {
            StatusCode::OK => {
                let payload = http_response.json::<RpcResponse>().await?;
                let expected_version = "2.0".to_owned();
                let request_id = payload.id.clone().unwrap();
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

impl<T: Transport> JsonRpcClient<T> {
    pub fn new(address: &str, user: &str, pass: &str) -> Result<Self, Error> {
        let parsed_address =
            Url::parse(address).map_err(|parse_error| Error::Url(parse_error.to_string()))?;
        Ok(JsonRpcClient {
            nonce: Arc::new(atomic::AtomicUsize::new(1)),
            transport: T::build(parsed_address, user, pass),
        })
    }

    pub fn create_method(&self, method: &str, params: Option<Box<RawValue>>) -> RpcRequest {
        let request_id = self.nonce.fetch_add(1, atomic::Ordering::Relaxed);
        RpcRequest::new(method, params, request_id)
    }

    pub async fn call_method(
        &self,
        method: &str,
        params: Option<Box<RawValue>>,
    ) -> Result<RpcResponse, Error> {
        self.transport
            .send(self.create_method(method, params))
            .await
    }
}
