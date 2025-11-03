use massa_rust_web3::{BUILDNET_URL, OperationId, MassaRpcClient, MassaJsonRpc};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let op_id = std::env::args().nth(1).unwrap();
    println!("op_id: {}", op_id);

    let op_ids =
        vec![OperationId::from_str(op_id.as_str()).expect("Cannot create op_id from first arg")];

    let client = MassaRpcClient::new(BUILDNET_URL);
    let result = client.get_operations(op_ids).await;

    if let Some(log_file) = std::env::args().nth(2) {
        let log_content = format!("{:#?}", result);
        std::fs::write(log_file.as_str(), log_content).expect("Cannot write to log file");
    } else {
        println!("get_operations result: {:?}", result);
    }
}
