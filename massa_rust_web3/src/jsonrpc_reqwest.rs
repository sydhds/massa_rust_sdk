use serde::{de::DeserializeOwned, Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;
// internal
use crate::MassaJsonRpc;

pub struct MassaRpcClient {
    client: Client,
    url : String,
}

impl MassaRpcClient {
    pub fn new(url: impl AsRef<str>) -> Self {
        Self {
            client: Client::builder()
                .build().unwrap(),
            url: url.as_ref().to_string(),
        }
    }
}

impl MassaJsonRpc for MassaRpcClient {
    type RpcParameters = serde_json::Value;
    type RpcError = reqwest::Error;

    async fn post<R: DeserializeOwned>(&self, method: &str, mut params: Self::RpcParameters) -> Result<R, Self::RpcError> {

        #[allow(dead_code)]
        #[derive(Debug, Deserialize)]
        struct Response<R> {
            jsonrpc: String,
            id: i32,
            result: R,
        }

        let url = format!("{}/{method}", self.url);
        // unwrap safe - prepare_params always returns a JSON object
        params
            .as_object_mut()
            .unwrap()
            .insert("method".to_string(), json!(method));

        let resp: Response<R> = self
            .client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.result)
    }

    fn prepare_params<T: Serialize>(params: T) -> Self::RpcParameters {
        json!({
                "jsonrpc": "2.0",
                "method": "",
                "params": [params],
                "id": 1
            })
    }

    fn empty_params() -> Self::RpcParameters {
        json!({
                "jsonrpc": "2.0",
                "method": "",
                "params": [],
                "id": 1
            })
    }
}