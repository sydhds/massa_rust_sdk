use std::collections::VecDeque;
use std::ops::Deref;
use std::str::FromStr;
// third-party
use jsonrpsee::core::{Serialize, client};
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder, rpc_params};
use serde::Deserialize;
// massa
use massa_api_exports::node::NodeStatus;
// massa re exports
pub use massa_api_exports::execution::{ExecuteReadOnlyResponse, ReadOnlyCall, ReadOnlyResult};
pub use massa_models::{
    address::Address, amount::Amount, execution::EventFilter, output_event::SCOutputEvent,
    slot::Slot,
};

pub const BUILDNET_URL: &str = "https://buildnet.massa.net/api/v2";

pub async fn get_status(url: impl AsRef<str>) -> Result<NodeStatus, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;
    let params = rpc_params![];
    client.request("get_status", params).await
}

pub async fn get_filtered_sc_output_event(
    url: impl AsRef<str>,
    event_filter: EventFilter,
) -> Result<Vec<SCOutputEvent>, client::Error> {
    let client = HttpClientBuilder::default().build(url)?;

    client
        .request("get_filtered_sc_output_event", rpc_params![event_filter])
        .await
}

/// Shortcut for `get_filtered_sc_output_event`
pub async fn get_events(
    url: impl AsRef<str>,
    event_filter: EventFilter,
) -> Result<Vec<SCOutputEvent>, client::Error> {
    get_filtered_sc_output_event(url, event_filter).await
}

#[derive(Debug, Clone)]
pub struct ReadOnlyCallParams(pub ReadOnlyCall);

impl Default for ReadOnlyCallParams {
    fn default() -> Self {
        // TODO: Zero address
        Self(ReadOnlyCall {
            max_gas: 0,
            target_address: Address::from_str(
                "AU12fZLkHnLED3okr8Lduyty7dz9ZKkd24xMCc2JJWPcdmfn2eUEx",
            )
            .unwrap(),
            target_function: "".to_string(),
            parameter: vec![],
            caller_address: None,
            coins: None,
            fee: None,
        })
    }
}

impl Deref for ReadOnlyCallParams {
    type Target = ReadOnlyCall;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Lightweight version of `ExecuteReadOnlyResponse` (Massa struct)
///
/// execute_read_only_call response deserialization will fail without this
/// Might be related to Massa issue: https://github.com/massalabs/massa/issues/4775
/// massa-web3 (js) has its own formatter formatReadOnlyExecuteSCParams to circumvent this
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecuteReadOnlyResponseLw {
    pub executed_at: Slot,
    pub result: ReadOnlyResult,
    pub output_events: VecDeque<SCOutputEvent>,
    pub gas_cost: u64,
}

pub async fn execute_read_only_call(
    url: impl AsRef<str>,
    read_params: Vec<ReadOnlyCall>,
) -> Result<Vec<ExecuteReadOnlyResponseLw>, client::Error> {
    // Note: Massa issue: https://github.com/massalabs/massa/issues/4775

    let client = HttpClientBuilder::default().build(url)?;

    // response type: Result<Vec<ExecuteReadOnlyResponseLw>, _>
    client
        .request("execute_read_only_call", rpc_params![read_params])
        .await
}
