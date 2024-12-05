use reqwest::{Client, Url, StatusCode};
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
        let parsed_url = Url::parse(url)
            .map_err(|parse_error| {Error::Url(parse_error.to_string())})?;
        Ok(HttpClient {
            client: Client::new(),
            url: parsed_url,
            user: user.to_owned(),
            pass: pass.to_owned(),
            nonce: atomic::AtomicUsize::new(1),
        })
    }

    pub async fn call_method(&self,
                             method: &str,
                             params: Option<Box<RawValue>>) -> Result<Response, Error> {
        let request = Request {
            jsonrpc: "2.0",
            id: Some(self.nonce.fetch_add(1, atomic::Ordering::Relaxed)),
            method: method,
            params: params.as_deref(),
        };

        let http_response = self.client.post(self.url.clone())
            .basic_auth(self.user.clone(), Some(self.pass.clone()))
            .json(&request)
            .header("content-type", "application/json")
            .send().await?;

	// TODO: Better error treatment for other statuses code
        if http_response.status() == StatusCode::OK {
            // TODO: check response nonce is equal to request nonce.
            // TODO: check response jsonrpc version.
            Ok(serde_json::from_str(&http_response.text().await?)?)
        } else {
            Err(Error::Err(format!("{:?}", http_response)))
        }
    }
}
