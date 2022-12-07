
use dotenv::dotenv;

use ethers_core::{
    abi::Abi,
    types::{Address}, //, H256
};
use ethers_contract::Contract;
use ethers_providers::{Provider, Http};
//use ethers_signers::Wallet;
use std::convert::TryFrom;


#[tokio::main]
async fn main() {

    println!("You are now using haris!");
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let binance_res = reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT").await.expect("Error!");
    let binance_body = binance_res.text().await.expect("Error!");
    println!("Result : {}", binance_body);

    // WEB3 CRATE ******************
    // We are using QuickNode API on Matic main network to get a faster result
    let transport = web3::transports::Http::new("https://skilled-young-choice.matic.discover.quiknode.pro/62ff487298631c3d207ced949a588db4acc9aa9f/").expect("No transport!");
    let web3 = web3::Web3::new(transport);
    let current_block = web3.eth().block_number().await.unwrap();
    println!("The current block is : {:#?}", current_block);

    //let accounts = web3.eth().accounts().await.unwrap();

    // let tx = TransactionRequest {
    //     from: accounts[0],
    //     to: Some(accounts[1]),
    //     gas: None,
    //     gas_price: None,
    //     value: Some(U256::from(10000)),
    //     data: None,
    //     nonce: None,
    //     condition: None
    // };

    // let tx_hash = web3.eth().send_transaction(tx).await.unwrap();

    //let balance = web3.eth().balance(accounts[1], None).await.unwrap();

    //println!("Balance before: {}", balance);

    println!("Balance before: {}", get_private_key());

    // let result = web3::contract::Contract::call(&self, "realtimeBalanceOf",
    // "ISuperfluidToken token, address account, uint256 /*time*/",
    // "from".parse::<Address>(),
    // "options").await.expect("Error");


// ETHERS CRATE *******

// this is a fake address used just for this example
let address = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".parse::<Address>().expect("Error!");

let envabi = std::env::var("abi").expect("abi must be set.");
println!("abi: {}", envabi);

// (ugly way to write the ABI inline, you can otherwise read it from a file)
let abi: Abi = serde_json::from_str(envabi.as_str()).expect("Error!");

// connect to the network
let client = Provider::<Http>::try_from("https://skilled-young-choice.matic.discover.quiknode.pro/62ff487298631c3d207ced949a588db4acc9aa9f/").unwrap();

// create the contract object at the address
let contract = Contract::new(address, abi, client);

// Calling constant methods is done by calling `call()` on the method builder.
// (if the function takes no arguments, then you must use `()` as the argument)
let init_value: String = contract
    .method::<_, String>("getValue", ()).expect("Error!")
    .call()
    .await.expect("Error!");

// // Non-constant methods are executed via the `send()` call on the method builder.
// let call = contract
//     .method::<_, H256>("setValue", "hi".to_owned()).expect("Error!");
// let pending_tx = call.send().await.expect("Error!");

// // `await`ing on the pending transaction resolves to a transaction receipt
// let receipt = pending_tx.confirmations(6).await.expect("Error!");
println!("Call: {}", init_value);


}

fn get_private_key() -> String{

    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let pk = std::env::var("PRIVATE_KEY").expect("abi must be set.");
    println!("pk: {}", pk);

    return pk

    //let private_key = hex::decode(pk).unwrap();

    // use ethereum_types::{H160,H256,U256};
    // return H256(to_array(private_key.as_slice()));
}
