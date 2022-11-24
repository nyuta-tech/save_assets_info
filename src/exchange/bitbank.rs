use crate::crypto;
use chrono::Utc;
use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use std::env;
use std::str;

#[derive(Debug, Deserialize)]
pub struct TickerData {
    pub buy: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerResponse {
    pub success: i8,
    pub data: TickerData,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub asset: String,
    pub onhand_amount: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetData {
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct AssetResponse {
    pub success: i8,
    pub data: AssetData,
}

const PUBLIC_API: &str = "https://public.bitbank.cc";
const PRIVATE_API: &str = "https://api.bitbank.cc";

pub async fn get_btc_price() -> TickerResponse {
    let client = Client::new();

    //get btc price from bitbank API (public)
    let path = "/btc_jpy/ticker";
    let response = client
        .get(PUBLIC_API.to_string() + path)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    let ticker_res: TickerResponse = serde_json::from_str(&body).unwrap();

    ticker_res
}

pub async fn get_eth_price() -> TickerResponse {
    let client = Client::new();

    //get btc price from bitbank API (public)
    let path = "/eth_jpy/ticker";
    let response = client
        .get(PUBLIC_API.to_string() + path)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    let ticker_res: TickerResponse = serde_json::from_str(&body).unwrap();

    ticker_res
}
pub async fn get_assets_info() -> AssetResponse {
    let client = Client::new();
    let api_key: String = env::var("BITBANK_API_KEY").unwrap();
    let path = "/v1/user/assets";
    let timestamp = Utc::now().timestamp() * 1000;

    //msg => timestamp + path
    let msg = timestamp.to_string() + path;
    let headers = create_header(api_key, timestamp.to_string(), msg);
    dbg!(PRIVATE_API.to_string() + path);
    let response = client
        .get(PRIVATE_API.to_string() + path)
        .headers(headers)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();

    //desirialize json to struct
    let assets_response: AssetResponse = serde_json::from_str(&body).unwrap();

    assets_response
}

pub fn create_header(key: String, nonce: String, msg: String) -> HeaderMap {
    let api_secret: String = env::var("BITBANK_API_SECRET").unwrap();
    let signature = crypto::sign_hmac(&api_secret, &msg);
    let mut headers = HeaderMap::new();
    headers.insert("ACCESS-KEY", key.parse().unwrap());
    headers.insert("ACCESS-NONCE", nonce.to_string().parse().unwrap());
    headers.insert(
        "ACCESS-SIGNATURE",
        signature.to_lowercase().parse().unwrap(),
    );
    headers
}

#[cfg(test)]
mod tests {
    use super::{get_assets_info, get_btc_price};
    use dotenv::dotenv;

    #[tokio::test]
    async fn check_btc_price() {
        let ticker_res = get_btc_price().await;
        assert_eq!(ticker_res.success, 1);
        assert!(ticker_res.data.buy != "");
        dbg!(ticker_res);
    }

    #[tokio::test]
    async fn check_self_assets() {
        dotenv().ok();
        let assets_res = get_assets_info().await;
        assert_eq!(assets_res.success, 1);
        dbg!(assets_res);
    }
}
