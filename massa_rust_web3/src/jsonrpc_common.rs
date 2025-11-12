// std
use std::collections::VecDeque;
// third-party
use serde::{Deserialize, Serialize, de::DeserializeOwned};
// Massa
use massa_api_exports::{
    address::AddressInfo,
    execution::{ReadOnlyBytecodeExecution, ReadOnlyCall, ReadOnlyResult},
    node::NodeStatus,
    operation::{OperationInfo, OperationInput},
};
use massa_models::{
    address::Address,
    execution::EventFilter,
    operation::{Operation, OperationId, OperationSerializer, SecureShareOperation},
    output_event::SCOutputEvent,
    secure_share::SecureShareContent,
    slot::Slot,
};
use massa_signature::KeyPair;

use crate::BUILDNET_CHAINID;

#[bitte::bitte]
pub trait MassaJsonRpc {
    type RpcParameters;
    type RpcError;

    // async fn post<R: DeserializeOwned + for<'a> Deserialize<'a>>(&self, method: &str, params: Self::RpcParameters) -> Result<R, Self::RpcError>;
    async fn post<R: DeserializeOwned>(
        &self,
        method: &str,
        params: Self::RpcParameters,
    ) -> Result<R, Self::RpcError>;

    fn prepare_params<T: Serialize>(params: T) -> Self::RpcParameters;

    fn empty_params() -> Self::RpcParameters;

    async fn get_status(&self) -> Result<NodeStatus, Self::RpcError> {
        self.post("get_status", Self::empty_params()).await
    }

    async fn get_addresses(
        &self,
        addresses: Vec<Address>,
    ) -> Result<Vec<AddressInfo>, Self::RpcError> {
        let params = Self::prepare_params(addresses);
        self.post("get_addresses", params).await
    }

    async fn get_filtered_sc_output_event(
        &self,
        event_filter: EventFilter,
    ) -> Result<Vec<SCOutputEvent>, Self::RpcError> {
        let params = Self::prepare_params(event_filter);
        self.post("get_filtered_sc_output_event", params).await
    }

    /// Shortcut for `get_filtered_sc_output_event`
    async fn get_events(
        &self,
        event_filter: EventFilter,
    ) -> Result<Vec<SCOutputEvent>, Self::RpcError> {
        self.get_filtered_sc_output_event(event_filter).await
    }

    async fn execute_read_only_call(
        &self,
        read_params: Vec<ReadOnlyCall>,
    ) -> Result<Vec<ExecuteReadOnlyResponseLw>, Self::RpcError> {
        let params = Self::prepare_params(read_params);
        // Note: Massa issue: https://github.com/massalabs/massa/issues/4775
        // response type: Result<Vec<ExecuteReadOnlyResponseLw>, _>
        self.post("execute_read_only_call", params).await
    }

    async fn execute_read_only_bytecode(
        &self,
        read_params: Vec<ReadOnlyBytecodeExecution>,
    ) -> Result<Vec<ExecuteReadOnlyResponseLw>, Self::RpcError> {
        let params = Self::prepare_params(read_params);
        // Note: Massa issue: https://github.com/massalabs/massa/issues/4775
        self.post("execute_read_only_bytecode", params).await
    }

    async fn get_operations(
        &self,
        operation_ids: Vec<OperationId>,
    ) -> Result<Vec<OperationInfo>, Self::RpcError> {
        let params = Self::prepare_params(operation_ids);
        let response: Vec<OperationInfo> = self.post("get_operations", params).await?;
        Ok(response)
    }

    async fn send_operations(
        &self,
        operation: Operation,
        keypair: &KeyPair,
    ) -> Result<Vec<OperationId>, Self::RpcError> {
        let operation: SecureShareOperation = {
            Operation::new_verifiable(
                operation,
                OperationSerializer::new(),
                keypair,
                BUILDNET_CHAINID,
            )
            .unwrap()
        };

        let input: OperationInput = OperationInput {
            creator_public_key: keypair.get_public_key(),
            signature: operation.signature,
            serialized_content: operation.serialized_data,
        };

        let params = Self::prepare_params(vec![input]);
        let response: Result<Vec<OperationId>, _> = self.post("send_operations", params).await;

        response
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
