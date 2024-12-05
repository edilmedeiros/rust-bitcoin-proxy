use crate::error::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;

/// A JSONRPC request object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<'a> {
    /// jsonrpc field, MUST be "2.0".
    pub jsonrpc: &'a str,
    /// The name of the RPC call.
    pub method: &'a str,
    /// Parameters to the RPC call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<&'a RawValue>,
    /// Identifier for this request, which should appear in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
}

/// A JSONRPC response object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    /// jsonrpc field, MUST be "2.0".
    pub jsonrpc: Option<String>,
    /// A result if there is one, or [`None`].
    pub result: Option<Box<RawValue>>,
    /// An error if there is one, or [`None`].
    pub error: Option<RpcError>,
    /// Identifier for this response, which should match that of the request.
    pub id: Option<serde_json::Value>,
}

impl Response {
    /// Returns the result, checking for errors.
    pub fn result<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        if let Some(ref e) = self.error {
            return Err(Error::Rpc(e.clone()));
        }

        if let Some(ref res) = self.result {
            serde_json::from_str(res.get()).map_err(Error::Json)
        } else {
            serde_json::from_value(serde_json::Value::Null).map_err(Error::Json)
        }
    }

    /// Returns the RPC error, if there's one.
    pub fn get_error(self) -> Result<(), Error> {
        if let Some(e) = self.error {
            Err(Error::Rpc(e))
        } else {
            Ok(())
        }
    }

    /// Check if the result field is empty
    pub fn is_none(&self) -> bool {
        self.result.is_none()
    }
}

/// Convert an argument into a boxed [`serde_json::value::RawValue`].
/// Since serializers rarely fail, it's probably easier to use [`arg`] instead.
pub fn try_arg<T: serde::Serialize>(arg: T) -> Result<Box<RawValue>, serde_json::Error> {
    RawValue::from_string(serde_json::to_string(&arg)?)
}

/// Convert an argument into a boxed [`serde_json::value::RawValue`].
///
/// This conversion should not fail, so to avoid returning a [`Result`],
/// in case of an error, the error is serialized as the return value.
pub fn arg<T: serde::Serialize>(arg: T) -> Box<RawValue> {
    match try_arg(arg) {
        Ok(v) => v,
        Err(e) => RawValue::from_string(format!("<<ERROR SERIALIZING ARGUMENT: {}>>", e))
            .unwrap_or_else(|_| {
                RawValue::from_string("<<ERROR SERIALIZING ARGUMENT>>".to_owned()).unwrap()
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getblocktemplate() {
        let params = Some(
            serde_json::value::RawValue::from_string(
                "[{\"rules\":[\"segwit\",\"signet\"]}]".to_owned(),
            )
            .unwrap(),
        );
        let request = Request {
            jsonrpc: "2.0",
            method: "getblocktemplate",
            params: params.as_deref(),
            id: Some(0),
        };
        assert_eq!(
            serde_json::to_string(&request).unwrap(),
            r#"{"jsonrpc":"2.0","method":"getblocktemplate","params":[{"rules":["segwit","signet"]}],"id":0}"#
        );
    }

    #[test]
    fn response_is_none() {
        let r0 = Response {
            jsonrpc: Some(String::from("2.0")),
            result: Some(RawValue::from_string(serde_json::to_string("signet").unwrap()).unwrap()),
            error: None,
            id: Some(From::from(0)),
        };

        let r1 = Response {
            jsonrpc: Some(String::from("2.0")),
            result: None,
            error: None,
            id: Some(From::from(1)),
        };

        assert!(!r0.is_none());
        assert!(r1.is_none());
    }

    #[test]
    fn response_result() {
        let obj = vec!["signet", "miner"];
        let r2 = Response {
            jsonrpc: Some(String::from("2.0")),
            result: Some(RawValue::from_string(serde_json::to_string(&obj).unwrap()).unwrap()),
            error: None,
            id: Some(From::from(2)),
        };
        let recovered: Vec<String> = r2.result().unwrap();
        assert!(r2.get_error().is_ok());
        assert_eq!(obj, recovered);

        let r3 = Response {
            jsonrpc: Some(String::from("2.0")),
            result: None,
            error: Some(RpcError {
                code: -32700,
                message: "Parse error".to_owned(),
                data: None,
            }),
            id: Some(From::from(2)),
        };
        assert!(r3.clone().get_error().is_err());
        assert!(r3.result::<String>().is_err());
    }

    #[test]
    fn null_result() {
        let s = r#"{"jsonrpc":"2.0","result":null,"error":null,"id":"test"}"#;
        let response: Response = serde_json::from_str(s).unwrap();
        let recovered1: Result<(), _> = response.result();
        let recovered2: Result<(), _> = response.result();
        assert!(recovered1.is_ok());
        assert!(recovered2.is_ok());

        let recovered1: Result<String, _> = response.result();
        let recovered2: Result<String, _> = response.result();
        assert!(recovered1.is_err());
        assert!(recovered2.is_err());
    }

    #[test]
    fn batch_response() {
        // from the jsonrpc.org spec example
        let s = r#"[
            {"jsonrpc": "2.0", "result": 7, "id": "1"},
            {"jsonrpc": "2.0", "result": 19, "id": "2"},
            {"jsonrpc": "2.0", "error": {"code": -32600, "message": "Invalid Request"}, "id": null},
            {"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": "5"},
            {"jsonrpc": "2.0", "result": ["hello", 5], "id": "9"}
        ]"#;
        let batch_response: Vec<Response> = serde_json::from_str(s).unwrap();
        assert_eq!(batch_response.len(), 5);
    }

    #[test]
    fn test_arg() {
        macro_rules! test_arg {
            ($val:expr, $t:ty) => {{
                let val1: $t = $val;
                let arg = super::arg(val1.clone());
                let val2: $t = serde_json::from_str(arg.get()).expect(stringify!($val));
                assert_eq!(val1, val2, "failed test for {}", stringify!($val));
            }};
        }

        test_arg!(true, bool);
        test_arg!(42, u8);
        test_arg!(42, usize);
        test_arg!(42, isize);
        test_arg!(vec![42, 35], Vec<u8>);
        test_arg!(String::from("test"), String);

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct Test {
            v: String,
        }
        test_arg!(
            Test {
                v: String::from("test")
            },
            Test
        );
    }
}
