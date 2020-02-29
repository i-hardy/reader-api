use crate::models::{CanonicalFeed};
use crate::schema::canonical_feeds;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn find_one(conn: &PgConnection, feed_id: i32) -> Option<CanonicalFeed> {
    let feed = canonical_feeds::table
        .filter(canonical_feeds::id.eq(feed_id))
        .first::<CanonicalFeed>(conn)
        .map_err(|err| eprintln!("canonical_feeds::find_one: {}", err))
        .ok()?;
    Some(feed)
}
