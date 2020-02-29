#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod db;
mod schema;
mod models;

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

pub fn rocket() -> rocket::Rocket {
  rocket::ignite().mount("/", routes![index])
    .register(catchers![not_found])
    .attach(db::Conn::fairing())
}

fn main() {
  rocket().launch();
}

#[cfg(test)]
mod test {
  use super::rocket;
  use rocket::local::Client;
  use rocket::http::Status;

  #[test]
  fn root_path() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
  }

  #[test]
  fn not_found() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/invalid-route").dispatch();
    assert_eq!(response.status(), Status::NotFound);
  }
}
