use super::schema::pages;

#[derive(Insertable)]
#[table_name="pages"]
pub struct NewPage<'a> {
    pub parent_id: i32,
    pub name: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, PartialEq)]
pub struct Page {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub body: String,
    pub isfolder: bool,
    pub published: bool,
}