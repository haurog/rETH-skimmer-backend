#[macro_use]
extern crate rocket;

// add our routes and services modules
mod routes;
mod services;

use routes::date::get_current_date;
// use routes::date::date_plus_month;

#[get("/")]
fn say_hello() -> &'static str {
    "Hello, welcome to the api!"
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![say_hello, get_current_date]);

    Ok(rocket.into())
}
