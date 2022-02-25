use diesel::Insertable;
use crate::schema::snippets;

#[derive(Insertable)]
#[table_name = "snippets"]
pub struct Snippet<'a> {
    pub link: &'a str,
    pub code: &'a str
}
