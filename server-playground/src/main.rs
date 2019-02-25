#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello {}, age {}", name, age)
}

fn main() {
    //rocket::ignite().mount("/hello", routes![hello]).launch();
    rocket::ignite()
        .mount("/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/hello", routes![hello])
        .launch();

}
