use diesel::Queryable;

#[derive(Queryable)]
pub struct Snippet {
    pub id: u64,
    pub link: String,
    pub code: String
}
