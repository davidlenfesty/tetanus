#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rusqlite;

use std::sync::Mutex;
use rocket::{Rocket, State};
use rocket_contrib::serve::StaticFiles;
use rusqlite::{Connection, NO_PARAMS};
use rusqlite::types::ToSql;

fn init_database(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS data (
        id INTEGER PRIMARY KEY,
        data TEXT NOT NULL
        )", NO_PARAMS)
        .expect("create entries table");

    //conn.execute("INSERT INTO data (id, data) VALUES (?1, ?2)",
    //    &[&0, &"Rocketeer" as &ToSql])
    //    .expect("insert failed");
}

#[get("/list")]
fn list_all(conn: State<Mutex<Connection>>) -> String {
    conn.lock()
        .expect("DB connection lock")
        .query_row("SELECT data FROM data", NO_PARAMS,
            |row| {row.get(0)})
        .expect("Wrong string")
}

fn main() {
    let conn = Connection::open("db.sqlite").expect("Open failed");

    init_database(&conn);

    //rocket::ignite().mount("/hello", routes![hello]).launch();
    rocket::ignite()
        .mount("/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![list_all])
        .manage(Mutex::new(conn))
        .launch();
}
