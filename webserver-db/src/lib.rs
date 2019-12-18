#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Page, NewPage};

pub fn establish_connection() ->PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_page<'a>(conn: &PgConnection, name: &'a str, body: &'a str) -> Page {
    use schema::pages;

    let new_page = NewPage {
        name: name,
        body: body,
    };

    diesel::insert_into(pages::table)
        .values(&new_page)
        .get_result(conn)
        .expect("Error saving new post")
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
        let proper_post = Post;
    }
}
