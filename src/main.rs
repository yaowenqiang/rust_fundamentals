#![allow(unused_variables)]

use std::fmt::format;

fn main() {
    let unused_variables :u32 = 0;
    let location : [f32;2] = [ 0.0, 0.0];
    let location2 : [f32;20] = [ 0.0; 20];
    let location3 : (&str, f64, f64) = ( "POLE", 0.1,0.2);
    println!("Hello, world!");
    println!("location name: {}, latitude:{}, longitude: {} ", location3.0, location3.1,location3.2);
    let (name, latitude, longitude) = location3;
    println!("location name: {}, latitude:{}, longitude: {} ", name, latitude,longitude);

    let person_name_slice   = "Donald Mallard";
    let person_name_string  = person_name_slice.to_string();
    let person_name_string  = String::from(person_name_string);
    let person_name_string  = String::from("Donald Mallard");
    let person_name_slice1 = person_name_string.as_str();

    let duck = "Duck";
    let airlines = "AirLines";
    let aireline_name = [duck, " ", airlines].concat();
    let aireline_name2 = format!("{} {}", duck, aireline_name);
    println!("{}", aireline_name);
    println!("{}", aireline_name2);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + " every time";
    println!("{}", slogan);

    // variables

    let my_inferred_variable = 1_i8;
    let _warning_variable = 0;

    // casting variable data type

    let float_thirty_two: f32 = 17.2;
    let unsigned_eight : u8 = 5;
    let cast_unsigned_eight = unsigned_eight as f32;
    let result = float_thirty_two / cast_unsigned_eight;
    println!("{}",result);

    let number: u8 = 65;
    let letter: char = number as char;
    print!("{}", letter);





}
