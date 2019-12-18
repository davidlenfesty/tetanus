extern crate webserver_db;
extern crate diesel;

use self::webserver_db::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // drop newline
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_page(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

// Let's be honest, this is never going to be used on Windows
const EOF: &'static str = "CTRL+D";