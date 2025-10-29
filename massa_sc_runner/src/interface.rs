use function_name::named;
use massa_sc_runtime::{Interface, InterfaceClone, RuntimeModule};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, RwLock},
};

#[derive(Clone, Default)]
pub struct MassaScRunnerInterface {
    sc_storage: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>,
    // other_sc_storage: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>,
}

impl InterfaceClone for MassaScRunnerInterface {
    fn clone_box(&self) -> Box<dyn Interface> {
        Box::new(self.clone())
    }
}

#[allow(unused_variables)]
impl Interface for MassaScRunnerInterface {
    fn increment_recursion_counter(&self) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn decrement_recursion_counter(&self) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn get_interface_version(&self) -> massa_sc_runtime::Result<u32> {
        todo!()
    }

    fn init_call(&self, address: &str, raw_coins: u64) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn init_call_wasmv1(
        &self,
        address: &str,
        raw_coins: massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn finish_call(&self) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn get_balance(&self) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_balance_for(&self, address: &str) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_balance_wasmv1(
        &self,
        address: Option<String>,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeAmount> {
        todo!()
    }

    fn transfer_coins(&self, to_address: &str, raw_amount: u64) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn transfer_coins_for(
        &self,
        from_address: &str,
        to_address: &str,
        raw_amount: u64,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn transfer_coins_wasmv1(
        &self,
        to_address: String,
        raw_amount: massa_proto_rs::massa::model::v1::NativeAmount,
        from_address: Option<String>,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn get_call_coins(&self) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_call_coins_wasmv1(
        &self,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeAmount> {
        todo!()
    }

    fn raw_set_bytecode(&self, bytecode: &[u8]) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn raw_set_bytecode_for(&self, address: &str, bytecode: &[u8]) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn set_bytecode_wasmv1(
        &self,
        bytecode: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn create_module(&self, module: &[u8]) -> massa_sc_runtime::Result<String> {
        todo!()
    }

    fn print(&self, message: &str) -> massa_sc_runtime::Result<()> {
        println!("{message}");
        Ok(())
    }

    fn get_keys(&self, prefix: Option<&[u8]>) -> massa_sc_runtime::Result<BTreeSet<Vec<u8>>> {
        todo!()
    }

    fn get_keys_for(
        &self,
        address: &str,
        prefix: Option<&[u8]>,
    ) -> massa_sc_runtime::Result<BTreeSet<Vec<u8>>> {
        todo!()
    }

    fn get_ds_keys_wasmv1(
        &self,
        prefix: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<BTreeSet<Vec<u8>>> {
        todo!()
    }

    #[named]
    fn raw_get_data(&self, key: &[u8]) -> massa_sc_runtime::Result<Vec<u8>> {
        println!("[{}] key: {:?}", function_name!(), key);
        let guard = self.sc_storage.read().unwrap();
        // Note: Massa get_data bail!("data entry not found") if key not found
        let data = guard.get(key).cloned().unwrap();
        println!("[{}] data: {:?}", function_name!(), data);
        Ok(data)
    }

    fn raw_get_data_for(&self, address: &str, key: &[u8]) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn get_ds_value_wasmv1(
        &self,
        key: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    #[named]
    fn raw_set_data(&self, key: &[u8], value: &[u8]) -> massa_sc_runtime::Result<()> {
        // TODO: debug!
        println!("[{}] key: {:?}, value: {:?}", function_name!(), key, value);
        // TODO: no unwrap
        let mut guard = self.sc_storage.write().unwrap();
        guard.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    fn raw_set_data_for(
        &self,
        address: &str,
        key: &[u8],
        value: &[u8],
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn set_ds_value_wasmv1(
        &self,
        key: &[u8],
        value: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn raw_append_data(&self, key: &[u8], value: &[u8]) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn raw_append_data_for(
        &self,
        address: &str,
        key: &[u8],
        value: &[u8],
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn append_ds_value_wasmv1(
        &self,
        key: &[u8],
        value: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn raw_delete_data(&self, key: &[u8]) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn raw_delete_data_for(&self, address: &str, key: &[u8]) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn delete_ds_entry_wasmv1(
        &self,
        key: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn has_data(&self, key: &[u8]) -> massa_sc_runtime::Result<bool> {
        Ok(self.sc_storage.read().unwrap().contains_key(key))
    }

    fn has_data_for(&self, address: &str, key: &[u8]) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn ds_entry_exists_wasmv1(
        &self,
        key: &[u8],
        address: Option<String>,
    ) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn raw_get_bytecode(&self) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn raw_get_bytecode_for(&self, address: &str) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn get_bytecode_wasmv1(&self, address: Option<String>) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn get_op_keys(&self, prefix: Option<&[u8]>) -> massa_sc_runtime::Result<Vec<Vec<u8>>> {
        todo!()
    }

    fn get_op_keys_wasmv1(&self, prefix: &[u8]) -> massa_sc_runtime::Result<Vec<Vec<u8>>> {
        todo!()
    }

    fn op_entry_exists(&self, key: &[u8]) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn get_op_data(&self, key: &[u8]) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn caller_has_write_access(&self) -> massa_sc_runtime::Result<bool> {
        Ok(true)
    }

    fn hash(&self, data: &[u8]) -> massa_sc_runtime::Result<[u8; 32]> {
        todo!()
    }

    fn hash_blake3(&self, bytes: &[u8]) -> massa_sc_runtime::Result<[u8; 32]> {
        todo!()
    }

    fn signature_verify(
        &self,
        data: &[u8],
        signature: &str,
        public_key: &str,
    ) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn evm_signature_verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn evm_get_address_from_pubkey(&self, public_key: &[u8]) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn evm_get_pubkey_from_signature(
        &self,
        hash: &[u8],
        signature: &[u8],
    ) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn is_address_eoa(&self, address: &str) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn address_from_public_key(&self, public_key: &str) -> massa_sc_runtime::Result<String> {
        todo!()
    }

    fn validate_address(&self, address: &str) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn get_time(&self) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn unsafe_random(&self) -> massa_sc_runtime::Result<i64> {
        todo!()
    }

    fn unsafe_random_f64(&self) -> massa_sc_runtime::Result<f64> {
        todo!()
    }

    fn unsafe_random_wasmv1(&self, num_bytes: u64) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn get_current_period(&self) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_current_thread(&self) -> massa_sc_runtime::Result<u8> {
        todo!()
    }

    fn get_current_slot(&self) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::Slot> {
        todo!()
    }

    fn get_owned_addresses(&self) -> massa_sc_runtime::Result<Vec<String>> {
        todo!()
    }

    fn get_call_stack(&self) -> massa_sc_runtime::Result<Vec<String>> {
        Ok(
            vec![
                "AU1Yvq49utdezr496dHbRj3TMjqsCh2awggjfGraHoddE7XfEkpY".to_string(),
                "AS1GFocKuZKiSr2Gcu8y69fraPEqs7xYWvEFikTKcThYC38sBFBH".to_string(),
            ]
        )
    }

    #[named]
    fn generate_event(&self, _event: String) -> massa_sc_runtime::Result<()> {
        println!("[{}] event: {}", function_name!(), _event);
        Ok(())
    }

    fn generate_event_wasmv1(&self, _event: Vec<u8>) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn get_module(
        &self,
        bytecode: &[u8],
        gas_limit: u64,
    ) -> massa_sc_runtime::Result<RuntimeModule> {
        todo!()
    }

    fn get_tmp_module(
        &self,
        bytecode: &[u8],
        gas_limit: u64,
    ) -> massa_sc_runtime::Result<RuntimeModule> {
        todo!()
    }

    fn send_message(
        &self,
        target_address: &str,
        target_handler: &str,
        validity_start: (u64, u8),
        validity_end: (u64, u8),
        max_gas: u64,
        raw_fee: u64,
        raw_coins: u64,
        data: &[u8],
        filter: Option<(&str, Option<&[u8]>)>,
    ) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn get_origin_operation_id(&self) -> massa_sc_runtime::Result<Option<String>> {
        todo!()
    }

    fn hash_sha256(&self, bytes: &[u8]) -> massa_sc_runtime::Result<[u8; 32]> {
        todo!()
    }

    fn hash_keccak256(&self, bytes: &[u8]) -> massa_sc_runtime::Result<[u8; 32]> {
        todo!()
    }

    fn chain_id(&self) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_deferred_call_quote(
        &self,
        target_slot: (u64, u8),
        gas_limit: u64,
        params_size: u64,
    ) -> massa_sc_runtime::Result<(bool, u64)> {
        todo!()
    }

    fn deferred_call_register(
        &self,
        target_addr: &str,
        target_func: &str,
        target_slot: (u64, u8),
        max_gas: u64,
        params: &[u8],
        coins: u64,
    ) -> massa_sc_runtime::Result<String> {
        todo!()
    }

    fn deferred_call_exists(&self, id: &str) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn deferred_call_cancel(&self, id: &str) -> massa_sc_runtime::Result<()> {
        todo!()
    }

    fn native_amount_from_str_wasmv1(
        &self,
        amount: &str,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeAmount> {
        todo!()
    }

    fn native_amount_to_string_wasmv1(
        &self,
        amount: &massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<String> {
        todo!()
    }

    fn check_native_amount_wasmv1(
        &self,
        amount: &massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn add_native_amount_wasmv1(
        &self,
        amount1: &massa_proto_rs::massa::model::v1::NativeAmount,
        amount2: &massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeAmount> {
        todo!()
    }

    fn sub_native_amount_wasmv1(
        &self,
        amount1: &massa_proto_rs::massa::model::v1::NativeAmount,
        amount2: &massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeAmount> {
        todo!()
    }

    fn scalar_mul_native_amount_wasmv1(
        &self,
        amount: &massa_proto_rs::massa::model::v1::NativeAmount,
        factor: u64,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeAmount> {
        todo!()
    }

    fn scalar_div_rem_native_amount_wasmv1(
        &self,
        dividend: &massa_proto_rs::massa::model::v1::NativeAmount,
        divisor: u64,
    ) -> massa_sc_runtime::Result<(
        massa_proto_rs::massa::model::v1::NativeAmount,
        massa_proto_rs::massa::model::v1::NativeAmount,
    )> {
        todo!()
    }

    fn div_rem_native_amount_wasmv1(
        &self,
        dividend: &massa_proto_rs::massa::model::v1::NativeAmount,
        divisor: &massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<(u64, massa_proto_rs::massa::model::v1::NativeAmount)> {
        todo!()
    }

    fn check_address_wasmv1(&self, to_check: &str) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn check_pubkey_wasmv1(&self, to_check: &str) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn check_signature_wasmv1(&self, to_check: &str) -> massa_sc_runtime::Result<bool> {
        todo!()
    }

    fn get_address_category_wasmv1(
        &self,
        to_check: &str,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::AddressCategory> {
        todo!()
    }

    fn get_address_version_wasmv1(&self, address: &str) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_pubkey_version_wasmv1(&self, pubkey: &str) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn get_signature_version_wasmv1(&self, signature: &str) -> massa_sc_runtime::Result<u64> {
        todo!()
    }

    fn checked_add_native_time_wasmv1(
        &self,
        time1: &massa_proto_rs::massa::model::v1::NativeTime,
        time2: &massa_proto_rs::massa::model::v1::NativeTime,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeTime> {
        todo!()
    }

    fn checked_sub_native_time_wasmv1(
        &self,
        time1: &massa_proto_rs::massa::model::v1::NativeTime,
        time2: &massa_proto_rs::massa::model::v1::NativeTime,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeTime> {
        todo!()
    }

    fn checked_mul_native_time_wasmv1(
        &self,
        time: &massa_proto_rs::massa::model::v1::NativeTime,
        factor: u64,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::NativeTime> {
        todo!()
    }

    fn checked_scalar_div_native_time_wasmv1(
        &self,
        dividend: &massa_proto_rs::massa::model::v1::NativeTime,
        divisor: u64,
    ) -> massa_sc_runtime::Result<(
        massa_proto_rs::massa::model::v1::NativeTime,
        massa_proto_rs::massa::model::v1::NativeTime,
    )> {
        todo!()
    }

    fn checked_div_native_time_wasmv1(
        &self,
        dividend: &massa_proto_rs::massa::model::v1::NativeTime,
        divisor: &massa_proto_rs::massa::model::v1::NativeTime,
    ) -> massa_sc_runtime::Result<(u64, massa_proto_rs::massa::model::v1::NativeTime)> {
        todo!()
    }

    fn base58_check_to_bytes_wasmv1(&self, s: &str) -> massa_sc_runtime::Result<Vec<u8>> {
        todo!()
    }

    fn bytes_to_base58_check_wasmv1(&self, bytes: &[u8]) -> String {
        todo!()
    }

    fn compare_address_wasmv1(
        &self,
        left: &str,
        right: &str,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::ComparisonResult> {
        todo!()
    }

    fn compare_native_amount_wasmv1(
        &self,
        left: &massa_proto_rs::massa::model::v1::NativeAmount,
        right: &massa_proto_rs::massa::model::v1::NativeAmount,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::ComparisonResult> {
        todo!()
    }

    fn compare_native_time_wasmv1(
        &self,
        left: &massa_proto_rs::massa::model::v1::NativeTime,
        right: &massa_proto_rs::massa::model::v1::NativeTime,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::ComparisonResult> {
        todo!()
    }

    fn compare_pub_key_wasmv1(
        &self,
        left: &str,
        right: &str,
    ) -> massa_sc_runtime::Result<massa_proto_rs::massa::model::v1::ComparisonResult> {
        todo!()
    }

    fn save_gas_remaining_before_subexecution(&self, gas_used_until: u64) {}
}
