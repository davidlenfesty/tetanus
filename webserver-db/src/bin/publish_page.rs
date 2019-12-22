extern crate webserver_db;
extern crate diesel;

use self::diesel::prelude::*;
use self::webserver_db::*;
use self::models::Page;
use std::env::args;

fn main() {
    use webserver_db::schema::pages::dsl::{pages, published};

    let id = args().nth(1).expect("publish_page requires a page id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection().unwrap();

    let page = diesel::update(pages.find(id))
        .set(published.eq(true))
        .get_result::<Page>(&connection)
        .expect(&format!("Unable to find page {}", id));

    println!("Published page {}", page.name);
}