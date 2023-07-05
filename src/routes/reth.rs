use rocket::serde::json::{Json};
use rocket::serde::json::{Value};

// import services module
use crate::services;


#[get("/rETH")]
pub async fn get_reth_exchange_rates() -> Json<Value> {
    Json(services::reth::get_reth_exchange_rates().await)
}

#[get("/rETHDevelop")]
pub async fn get_reth_exchange_rates_dev() -> Json<Value> {
    Json(services::reth::get_reth_exchange_rates_dev().await)
}