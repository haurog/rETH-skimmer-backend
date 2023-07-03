#[macro_use]
extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

// add our routes and services modules
mod routes;
mod services;

use routes::date::date_plus_month;
use routes::date::get_current_date;
use routes::reth::get_reth_exchange_rates;


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome to the api!"
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {

    let rocket = rocket::build().mount(
        "/",
        routes![
            say_hello,
            get_current_date,
            date_plus_month,
            get_reth_exchange_rates
        ]
    ).attach(CORS);

    Ok(rocket.into())
}
