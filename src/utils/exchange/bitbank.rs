use reqwest::{Client};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub buy:String,
}

#[derive(Debug, Deserialize)]
pub struct TickerResponse {
    pub success: i8,
    pub data: Data,
}
pub async fn get_btc_price() -> TickerResponse {
    let client = Client::new();

    //get btc price from bitbank API (public)
    let public_bitbank_api_base = "https://public.bitbank.cc";
    let path = "/btc_jpy/ticker";
    let response = client
        .get(public_bitbank_api_base.to_string() + path)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    let ticker_res:TickerResponse = serde_json::from_str(&body).unwrap();

    ticker_res
}

#[cfg(test)]
mod tests {
    use super::get_btc_price;

    #[tokio::test]
    async fn check_btc_price(){
        let ticker_res = get_btc_price().await;
        assert_eq!(ticker_res.success, 1);
        dbg!(ticker_res);
        
        
    }
}