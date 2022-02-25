use diesel::Insertable;
use crate::schema::languages;

#[derive(Insertable)]
#[table_name = "languages"]
pub struct Language<'a> {
    pub name: &'a str
}
