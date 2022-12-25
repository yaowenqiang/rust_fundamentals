#![feature(core_panic)]
extern crate core;
use core::panicking::panic;
use std::{env, fs, process};
use std::error::Error;


struct Config {
    query: String,
     file_path: String,
}
fn main() {
    let args :Vec<String> = env::args().collect();
    // let (query, file_path) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for: {}", config.query);
    println!("In file : {}", config.file_path);
    // dbg!(args);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // println!("In file {}", file_path);
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
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
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{query, file_path})
    }
}