use rocket::serde::json::{Json};
use rocket::serde::json::{Value};

// import services module
use crate::services;

// create get-current-date route under /date and call get_current_date service which will return a Date object
// route returns a Date object converted to JSON
#[get("/rETH")]
pub async fn get_reth_exchange_rates() -> Json<Value> {
    Json(services::reth::get_reth_exchange_rates().await)
}