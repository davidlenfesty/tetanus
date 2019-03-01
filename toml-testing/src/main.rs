extern crate toml;

use std::fs::File;
use std::io::{BufRead, BufReader};
use toml::Value;

fn main() {
    let config_file = File::open("config.toml").unwrap();
    let config_file = BufReader::new(&config_file);

    for line in config_file.lines() {
        let line = line.unwrap();

        let value = line.parse::<Value>().unwrap();

        println!("{}", value);
    }

    
}