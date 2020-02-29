use rocket_contrib::databases::diesel;

pub mod canonical_feeds;

#[database("postgres_db")]
pub struct Conn(diesel::PgConnection);