use massa_rust_web3::{BUILDNET_URL,
                      get_status,
                      // get_status_reqwest
};

#[tokio::main]
async fn main() {

    let node_status = get_status(BUILDNET_URL).await.unwrap();
    println!("{}", "#".repeat(20));
    println!("Node status: {}", node_status);
    println!("{}", "#".repeat(35));

    /*
    let node_status = get_status_reqwest(BUILDNET_URL).await.unwrap();
    println!("{}", "#".repeat(20));
    println!("Node status: {}", node_status);
    */
}