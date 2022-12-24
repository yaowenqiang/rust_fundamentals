#![feature(core_panic)]
extern crate core;
use core::panicking::panic;
use std::{env, fs};


struct Config {
    query: String,
     file_path: String,
}
fn main() {
    let args :Vec<String> = env::args().collect();
    // let (query, file_path) = parse_config(&args);
    // let config = parse_config(&args);
    let config = Config::new(&args);
    println!("Searching for: {}", config.query);
    println!("In file : {}", config.file_path);
    // dbg!(args);

    // println!("In file {}", file_path);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");
    println!("With text:\n{contents}");
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config{query, file_path}
    }
}