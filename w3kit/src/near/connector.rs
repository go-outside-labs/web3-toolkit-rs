// near/connectors.rs
// author: steinkirch

use std::env;


////////////////////////////
// Public functions
////////////////////////////

pub async fn near_connect_ws() {

    let url = &env::var("NEAR_URL_WS").expect("⛔️ No NEAR_URL_WS on .env file");
    println!("⛔️ near_connect_ws() not implemented yet");

}

pub async fn near_connect_http() {

    let url = &env::var("NEAR_URL_HTTP").expect("⛔️ No NEAR_URL_HTTP on .env file");
    println!("⛔️ near_connect_http() not implemented yet");

}

pub async fn near_get_account(account_address: &str) {

    println!("✅ retrieving balances...");
    println!("⛔️ near_get_account() not implemented yet");

}
