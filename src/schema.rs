table! {
    canonical_feeds (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
    }
}

table! {
    feeds (id) {
        id -> Int4,
        name -> Varchar,
        website -> Varchar,
        last_updated -> Timestamptz,
        canonical_feed -> Int4,
    }
}

joinable!(feeds -> canonical_feeds (canonical_feed));

allow_tables_to_appear_in_same_query!(
    canonical_feeds,
    feeds,
);
