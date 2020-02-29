#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{JsonValue};

#[catch(404)]
fn not_found() -> JsonValue {
  json!({
    "status": "error",
    "reason": "Resource was not found."
  })
}

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

fn main() {
  rocket::ignite().mount("/", routes![index])
    .register(catchers![not_found])
    .launch();
}
