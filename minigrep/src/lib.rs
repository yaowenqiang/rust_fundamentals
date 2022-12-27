#![feature(core_panic)]
extern crate core;
use core::panicking::panic;
use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub file_path: String,
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // println!("In file {}", file_path);
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config{query, file_path}
    }
   pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config{query, file_path})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.";
        assert_eq!(vec!["safe, fast, productive"],search(query, contents));
    }
}