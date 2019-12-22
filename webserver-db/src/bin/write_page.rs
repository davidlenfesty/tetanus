extern crate webserver_db;
extern crate diesel;

use self::webserver_db::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection().unwrap();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // drop newline

    println!("Is this the root page? [y/N]");
    let mut is_root = String::new();
    stdin().read_line(&mut is_root).unwrap();
    let is_root = &is_root[..(is_root.len() - 1)]; // drop newline
    let is_root = is_root.to_ascii_uppercase();
    let is_root = match is_root.as_ref() {
        "Y" => true,
        _   => false
    };

    let mut parent_id = -1;
    if !is_root {
        println!("Okay, please enter the parent page of this ID:");
        let mut parent_id_str = String::new();
        stdin().read_line(&mut parent_id_str).unwrap();
        let parent_id_str = &parent_id_str[..(parent_id_str.len() - 1)];
        parent_id = parent_id_str.parse::<i32>().expect("Not a valid integer!");
    } else {
    }

    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_page(&connection, parent_id, title, &body).unwrap();
    println!("\nSaved draft {} with id {}", title, post.id);
}

// Let's be honest, this is never going to be used on Windows
const EOF: &'static str = "CTRL+D";