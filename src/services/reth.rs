use reqwest;

use rocket::serde::json::Value;
// import our Date object from the routes/date module

pub async fn get_reth_exchange_rates() -> Value {
    let result = reqwest::get("https://rocketscan.io/api/mainnet/reth/ratios")
        .await
        .unwrap()
        .text()
        .await;
    // println!("{:?}", result);

    let data = rocket::serde::json::from_str::<Value>(result.unwrap().as_str());

    data.unwrap()
}


pub async fn get_reth_exchange_rates_dev() -> Value {
    let result = reqwest::get("https://rocketscan.io/api/mainnet/reth/ratios")
        .await
        .unwrap()
        .text()
        .await;
    // println!("{:?}", result);

    let data = rocket::serde::json::from_str::<Value>(result.unwrap().as_str());

    data.unwrap()
}
