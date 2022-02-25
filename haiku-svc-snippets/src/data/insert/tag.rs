use diesel::Insertable;
use crate::schema::tags;

#[derive(Insertable)]
#[table_name = "tags"]
pub struct Tag<'a> {
    pub name: &'a str
}
