use diesel::Queryable;

#[derive(Queryable)]
pub struct Language {
    pub id: u64,
    pub name: String
}
