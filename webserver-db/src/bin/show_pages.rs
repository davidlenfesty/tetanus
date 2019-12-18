extern crate webserver_db;
extern crate diesel;

use self::webserver_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use webserver_db::schema::pages::dsl::*;

    let connection = establish_connection();
    let results = pages.filter(published.eq(true))
        .limit(5)
        .load::<Page>(&connection)
        .expect("Error loading pages!");

    println!("Displaying {} pages", results.len());

    for page in results {
        println!("{}", page.name);
        println!("-------\n");
        println!("{}", page.body);
    }
}