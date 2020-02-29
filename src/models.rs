#[derive(Serialize, Queryable, Debug)]
pub struct CanonicalFeed {
  pub id: i32,
  pub name: String,
  pub url: String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Feed {
  pub id: i32,
  pub name: String,
  pub website: String,
}
