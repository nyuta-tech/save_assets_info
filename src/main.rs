use chrono::Utc;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use sha2::Sha256;
use std::env;
use std::str;

#[derive(Debug, Deserialize)]
struct Address {
    address1: String,
    address2: String,
    address3: String,
    prefcode: String,
    zipcode: String,
}

#[derive(Debug, Deserialize)]
struct ZipCloudResponse {
    status: u32,
    results: Vec<Address>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = "Token ".to_string() + &env::var("INFLUX_TOKEN").unwrap();
    let client = Client::new();
    let base_url = "http://localhost:8086";

    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, token.parse().unwrap());
    let response = client
        .get(base_url.to_string() + "/api/v2/buckets")
        .headers(headers)
        //.header(reqwest::header::AUTHORIZATION, token)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();

    //get assets info from bitbank API
    let path = "/v1/user/assets";
    let utc_now = Utc::now();
    let timestamp = utc_now.timestamp() * 1000;
    let api_key = env::var("BITBANK_API_KEY").unwrap();
    let api_secret = env::var("BITBANK_API_SECRET").unwrap();
    let msg = timestamp.to_string() + path;
    dbg!(&msg);

    headers = HeaderMap::new();

    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    // Create HMAC-SHA256 instance which implements `Mac` trait
    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).expect("test");
    mac.update(msg.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let signature = format!("{:X}", code_bytes);

    dbg!(&signature);
    dbg!(&api_key);
    dbg!(timestamp);

    let bitbank_api_base = "https://api.bitbank.cc/v1";
    headers.insert("ACCESS-KEY", api_key.parse().unwrap());
    headers.insert("ACCESS-NONCE", timestamp.to_string().parse().unwrap());
    headers.insert("ACCESS-SIGNATURE", signature.to_lowercase().parse().unwrap());
    dbg!(&headers);
    let response = client
        .get("https://api.bitbank.cc/v1/user/assets")
        .headers(headers)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    println!("{}", body);
}
