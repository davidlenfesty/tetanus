#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use std::sync::Mutex;
//use rocket::{Rocket, State};
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use super::schema::people;

#[derive(Queryable, AsChangeSet, Serialize, DeSerialize)]
#[table_name = "people"]
struct Person {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
    pub profession: String,
    pub salary: u32,
}

#[get("/list")]
fn list_all(conn: State<Mutex<Connection>>) -> String {
    // TODO figure out error handling here
    let mut out_string = String::from("<!DOCTYPE html><body>
        <h1>List of all data<h1>");

    out_string
}

fn main() {
    //let conn = Connection::open("db.sqlite").expect("Open failed");
    let conn = Connection::open_in_memory().expect("Open failed");

    init_database(&conn);

    //rocket::ignite().mount("/hello", routes![hello]).launch();
    rocket::ignite()
        .mount("/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![list_all])
        .manage(Mutex::new(conn))
        .launch();
}
