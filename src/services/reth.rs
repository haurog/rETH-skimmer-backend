// chrono is a time library for Rust
use chrono::Datelike;
use reqwest;

use rocket::serde::json::{Value};
// import our Date object from the routes/date module
use crate::routes::reth::Date;

pub async fn get_reth_exchange_rates() -> Date {
    let current_utc = chrono::Utc::now();
    let year = current_utc.year();
    let month = current_utc.month();
    let day = current_utc.day();
    let current_date = Date {
        day,
        month,
        year
    };

    let result = reqwest::get("https://rocketscan.io/api/mainnet/reth/ratios").await.unwrap().text().await;
    println!("{:?}", result);


    return current_date;


    // let data = rocket::serde::json::from_str::<Value>(r#"
    // {
    //     "name": "John Doe",
    //     "age": 43,
    //     "phones": [
    //         "+44 4743233445",
    //         "+44 7645345677"
    //     ]
    // }"#);

    // data
}