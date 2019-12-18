extern crate webserver_db;
extern crate diesel;

use self::diesel::prelude::*;
use self::webserver_db::*;
use std::env::args;

fn main() {
    use webserver_db::schema::pages::dsl::*;

    let target = args().nth(1).expect("Expected a target!");
    let pattern  = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(pages.filter(name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}