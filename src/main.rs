use dotenv::dotenv;
use reqwest::{header::HeaderMap, Client};
use std::env;
mod crypto;
mod exchange;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client = Client::new();

    //get btc price from bitbank API (public)
    let ticker_response = exchange::bitbank::get_btc_price().await;
    let btc_price = ticker_response.data.buy.parse::<f32>().unwrap();
    let ticker_response = exchange::bitbank::get_eth_price().await;
    let eth_price = ticker_response.data.buy.parse::<f32>().unwrap();

    //get assets info from bitbank API
    let assets_response = exchange::bitbank::get_assets_info().await;
    let assets = assets_response.data.assets;

    //insert assets_info into influxdb.

    let token = "Token ".to_string() + &env::var("INFLUX_TOKEN").unwrap();
    let base_url = "http://influxdb:8086";

    let mut all_jpy_value: f32 = 0.0;
    let mut all_btc_value: f32 = 0.0;
    let mut all_eth_value: f32 = 0.0;

    let mut msg: String;
    for info in assets.iter() {
        let measurement = "assets_info";
        let exchange = "bitbank";
        let currency = &*info.asset;

        let amount = &info.onhand_amount.parse::<f32>().unwrap();

        let jpy_value: f32;
        let btc_value: f32;
        let eth_value: f32;

        match currency {
            "btc" => {
                btc_value = *amount;
                jpy_value = *amount * btc_price;
                eth_value = jpy_value / eth_price;
            }
            "jpy" => {
                jpy_value = *amount;
                btc_value = *amount / btc_price;
                eth_value = *amount / eth_price;
            }
            "eth" => {
                eth_value = *amount;
                jpy_value = *amount * eth_price;
                btc_value = jpy_value / btc_price;
            }
            _ => continue,
        }

        msg = format!(
            "{0},exchange={1},currency={2} jpy_value={3},btc_value={4},eth_value={5} ",
            measurement, exchange, currency, jpy_value, btc_value, eth_value
        );
        all_btc_value += btc_value;
        all_jpy_value += jpy_value;
        all_eth_value += eth_value;

        client
            .post(base_url.to_string() + "/api/v2/write?org=BOOLION&bucket=CryptoBucket")
            .body(msg)
            .headers(create_headers(&token))
            .send()
            .await
            .unwrap();
    }
    msg = format!(
        "all_assets jpy_value={},btc_value={},eth_value={} ",
        all_jpy_value, all_btc_value, all_eth_value
    );
    let response = client
        .post(base_url.to_string() + "/api/v2/write?org=BOOLION&bucket=CryptoBucket")
        .body(msg)
        .headers(create_headers(&token))
        .send()
        .await
        .unwrap();
    dbg!(&response);
    let body = response.text().await.unwrap();
    dbg!(body);
}

fn create_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, token.parse().unwrap());
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
    headers
}
