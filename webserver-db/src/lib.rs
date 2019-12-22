#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use snafu::Snafu;

pub mod schema;
pub mod models;

use self::models::{Page, NewPage};

// Custom error type using snafu
// I may want to implement this myself, just to learn
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to connect to database"))]
    DbConnection,

    #[snafu(display("Unable to write to database"))]
    DbWrite,

    #[snafu(display("Unable to read from database"))]
    DbRead,
}

// TODO should be a way to type this so we only need to use one thing
type Result<T, E = Error> = std::result::Result<T, E>;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(_) => Err(Error::DbConnection),
    }
}

pub fn create_page<'a>(conn: &PgConnection, parent_id: i32, name: &'a str, body: &'a str) -> Result<Page> {
    use schema::pages;

    let new_page = NewPage {
        parent_id: parent_id,
        name: name,
        body: body,
    };

    // I think we can replace this match if we properly
    // implement the From trait on our errors
    // maybe snafu does this for us? TODO
    match diesel::insert_into(pages::table) 
        .values(&new_page)
        .get_result(conn) {
            Ok(page) => Ok(page),
            Err(_) => Err(Error::DbWrite),
    }
}

// change arg names
pub fn find_page<'a>(conn: &PgConnection, parent: i32, desc: &'a str) -> Result<Vec<Page>> {
    use schema::pages::dsl::*;

    match pages.filter(parent_id.eq(parent))
        .filter(name.eq(desc))
        .load::<Page>(conn) {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::DbRead),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn page_is_created() {
        // Need to connect to a new db or something
        let conn = establish_connection();
        let proper_post = Page;
    }
}
