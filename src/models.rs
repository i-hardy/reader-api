#[derive(Serialize, Queryable, Debug)]
pub struct CanonicalFeed {
  pub id: i32,
  pub name: String,
  pub url: String,
}
