#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod db;
mod api;
mod schema;
mod models;

use api::rocket;

fn main() {
  rocket().launch();
}
