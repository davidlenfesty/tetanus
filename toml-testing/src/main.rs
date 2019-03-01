extern crate toml;

use std::fs::File;
use std::io::{BufRead, BufReader};
use toml::Value;

fn main() {
    //let config_file = File::open("config.toml").unwrap();
    //let config_file = BufReader::new(&config_file);

    let table = std::fs::read_to_string("config.toml").unwrap();

    let table = table.parse::<Value>().unwrap();

    println!("{:?}", table);

    println!("{}", table["registers"]);

    println!("Description is: {}", table.get("registers").unwrap().get("DDRA").unwrap().get("description").unwrap());

    //for line in config_file.lines() {
    //    let line = line.unwrap();
    //    println!("{:?}", line);

    //    let value = line.parse::<Value>().unwrap();

    //    println!("{:?}", value);
    //}
}