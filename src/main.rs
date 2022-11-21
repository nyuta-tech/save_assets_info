use reqwest::{Client, header::HeaderMap};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;

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
    println!("{}", body);
}
