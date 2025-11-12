use jsonrpsee::{
    core::{client, client::ClientT, params::ArrayParams},
    http_client::{HttpClient, HttpClientBuilder},
    rpc_params,
};
use serde::{Serialize, de::DeserializeOwned};
// internal
use crate::jsonrpc_common::MassaJsonRpc;

pub struct MassaRpcClient {
    client: HttpClient,
}

impl MassaRpcClient {
    pub fn new(url: impl AsRef<str>) -> Self {
        Self {
            client: HttpClientBuilder::default().build(url).unwrap(),
        }
    }
}

impl MassaJsonRpc for MassaRpcClient {
    type RpcParameters = ArrayParams;
    type RpcError = client::Error;

    async fn post<R: DeserializeOwned>(
        &self,
        method: &str,
        params: Self::RpcParameters,
    ) -> Result<R, Self::RpcError> {
        self.client.request(method, params).await
    }

    fn prepare_params<T: Serialize>(params: T) -> Self::RpcParameters {
        rpc_params![params]
    }

    fn empty_params() -> Self::RpcParameters {
        rpc_params![]
    }
}
