// chrono is a time library for Rust
use chrono::Datelike;

// import our Date object from the routes/date module
use crate::routes::reth::Date;

pub fn get_reth_exchange_rates() -> Date {
    let current_utc = chrono::Utc::now();
    let year = current_utc.year();
    let month = current_utc.month();
    let day = current_utc.day();
    let current_date = Date {
        day,
        month,
        year
    };
    current_date
}