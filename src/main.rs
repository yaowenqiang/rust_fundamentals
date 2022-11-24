#![allow(unused_variables)]
#![feature(exclusive_range_pattern)]
#![feature(core_panic)]

extern crate core;

use core::panicking::panic;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, format, write};
use std::fs::File;
use std::io::{ErrorKind, Read, Error};
use std::iter::Sum;
use std::str::ParseBoolError;

enum NavigationAids {
    // NOB = 7,
    // // VOR = 6,
    // VOR,
    // VORDME,
    // FIX {name: String, latitude: f32, longitude: f32}
    NOB(u16),
    // VOR = 6,
    VOR(String, f32),
    VORDME(String, f32),
    FIX{name: String, latitude: f32, longitude: f32}
}

struct Waypoint {
    name: String,
    latitude: f32,
    longitude: f32,
}

struct Segment {
    start: Waypoint,
    end: Waypoint
}

struct Boeing {
    required_crew: u8,
    range: u16
}

struct Airbus {
    required_crew: u8,
    range: u16
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 280 > distance) {
            true
        } else {
            false
        }
    }
}

trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self {
            start,
            end
        }
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T,U> {
    x: T,
    y: U,
}

impl <T, U> Point2<T,U> {
    fn mixup<T1,U1>(self, other: Point2<T1,U1>) -> Point2<T,U1> {
        Point2{
            x: self.x,
            y: other.y
        }
    }
}

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None
}

pub trait Summary {
    fn summarize(&self) -> String {
        // String::from("(Read more...)")
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}",self.username)
    }
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
        // format!("{}, by {}, ({})",self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T, U>(t : &T, u :U) -> String
where
    T: Display + Clone,
    U: Clone + Debug + Display,
{
    format!("Breaking news! {}, {}", t, u)
}

pub fn return_summarizable() -> impl Summary {
    NewsArticle{
        headline:String::from("Today WC begin."),
        location:String::from("Qatar"),
        author:String::from("wbb"),
        content:String::from("FIFA WC 2022")
    }
}

// this function  won't work, must return the same type
// pub fn return_multitype_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle{
//             headline:String::from("Today WC begin."),
//             location:String::from("Qatar"),
//             author:String::from("wbb"),
//             content:String::from("FIFA WC 2022")
//         }
//     } else {
//         Tweet{
//             username: String::from("jack"),
//             content: String::from("very good content")
//         }
//     }
// }

fn main() {
    let unused_variables: u32 = 0;
    let location: [f32; 2] = [0.0, 0.0];
    let location2: [f32; 20] = [0.0; 20];
    let location3: (&str, f64, f64) = ("POLE", 0.1, 0.2);
    println!("Hello, world!");
    // println!("location name: {}, latitude:{}, longitude: {} ", location3.0, location3.1, location3.2);
    let (name, latitude, longitude) = location3;
    println!("location name: {}, latitude:{}, longitude: {} ", name, latitude, longitude);

    let person_name_slice = "Donald Mallard";
    let person_name_string = person_name_slice.to_string();
    let person_name_string = String::from(person_name_string);
    let person_name_string = String::from("Donald Mallard");
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
    let unsigned_eight: u8 = 5;
    let cast_unsigned_eight = unsigned_eight as f32;
    let result = float_thirty_two / cast_unsigned_eight;
    println!("{}", result);

    let number: u8 = 65;
    let letter: char = number as char;
    println!("{}", letter);
    let number2: u128 = 65;
    // let letter2 : char = number2 as char;
    // let letter2 : char = char::from_u32(number2);
    // println!("{}", letter2);

    // Mutability

    let mut changeable_variable = 65;

    // scope

    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        // shadowing
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);

    // operators

    let modules = 18 % 7;
    println!("{}", modules);

    let squared = i32::pow(8, 2);
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);

    println!("Integer: {}", squared);
    println!("float to int: {}", float_integer);
    println!("float to float: {}", float_float);

    let order_ops = 8 + 4 * 2 - (12 / 3 + 7) + 4;
    println!("{}", order_ops);


    //bitwise operators

    let bitwise_and = 86 & 27;
    println!("bitwise and: {}", bitwise_and);

    let bitwise_or = 86 | 27;
    println!("bitwise or: {}", bitwise_or);

    let bitwise_xor = 86 ^ 27;
    println!("bitwise xor: {}", bitwise_xor);

    let left_shift = 86 << 1;
    println!("left shift: {}", left_shift);

    let right_shift = 86 >> 1;
    println!("right shift: {}", right_shift);


    // if statement

    let word = "Duck";
    if word == "Duck" {
        println!("Quack");
    } else if word == "Dog" {
        println!("Bark");
    } else {
        println!("ALL quiet out here.");
    }

    // enum

    // println!("NOR: \t{}", NavigationAids::NOB);
    println!("NOR: \t{}", NavigationAids::NOB as u8);
    println!("VOR: \t{}", NavigationAids::VOR as u8);
    println!("VORDME: \t{}", NavigationAids::VORDME as u8);


    let ndb_val = NavigationAids::NOB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_tarry = NavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -85.91367
    };

    print_nav_aid(&ndb_val);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_tarry);


    let ndb_freq:u16 = 384;
    match ndb_freq {
        // range match
        200..500 => {
            println!("NDB frequency is valid")
        }
        _ => {
            println!("NDB frequency is invalid")
        }
    }

    let ndb_freq:u16 = 384;
    match ndb_freq {
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB frequency is valid")
        }
        _ => {
            println!("NDB frequency is invalid")
        }
    }

    // Options

    let phrase:String = String::from("Duck Airlines");
    let letter = phrase.chars().nth(5);

    let value = match letter {
        Some(character)  => character.to_string(),
        None => String::from("No value")
    };
    println!("{}", value);

    // let
    // OuterAttribute* let PatternNoTopAlt(: Type)?(=Expression)?;
    // https://doc.rust-lang.org/reference/statements.html#let-statements
    // https://doc.rust-lang.org/reference/expressions/if-expr.html
    let animal = "Duck";
    if let animal = "Duck" {
        print!("Quack");
    }

    if  animal == "Cat" {
        print!("Quack");
    }


    let dish = ("Ham", "Eggs");
    if let ("Bacon", b) = dish {
        println!("Bacon is served with {}", b);
    } else {
        // This block is evaluated instead.
        println!("No bacon will be served");
    }

// this body will execute
    if let ("Ham", b) = dish {
        println!("Ham is served with {}", b);
    }

    if let (c, d) = dish {
        println!("{}", format!("c:{}, d:{}", c,d));
    }

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }

    // loop

    let mut counter = 0;
    loop {
        counter += 1;
        println!("{}", counter);
        if counter == 5 {
            continue;
        }
        if counter == 10 {
            break;
        }
    }

    for index in 1 .. 10 {
        println!("{}", index);
    }

    for index in 1 ..=10 {
        println!("{}", index);
    }

    // https://www.merriam-webster.com/dictionary/iterate

    let arrs = [10,20,30];
    for i in arrs.iter() {
        println!("{}", i);
    }

    // ownership
    let mut original = String::from("original value");
    println!("\noriginal: \n\"{}\"\n", original);
    {

        // let mut next = original;
        let  next = &mut original;
        println!("{}", next); // works ,?
        next.push('n');
        *next = String::from("next value"); // * means dereference
        println!("{}", next);
    }
    println!("{}", original);

    // lifetimes

    let outer_scope;
    {
        let inner_scope = 5;
        outer_scope = &inner_scope;
    }
    // println!("{}", outer_scope);

    // let returned_ref = return_bad_ref();
    // println!("{}", returned_ref);

    let referenced_int = 6;
    let returned_value = return_one_parameter(&referenced_int);
    println!("{}", returned_value);

    let value_one = 24;
    let value_two = 77;
    let value = explicit_lifetime(&value_one, &value_two);
    println!("{}", value);

    let greater = return_greater(10, 5);
    println!("{}", greater);

    let mut original = String::from("original value");
    println!("\n outer scope original: \n\"{}\"", original);
    {
        print_original(&original);
        change_original(&mut original);
        println!("inner scope original: \t \"{}\"", original);
    }
    // closure
    let name = "Duck AirLines";
    let write_message = |slogan: String| -> String {
        println!("Hey, This is the closure.");
        println!("{}, {}", name, slogan);
        format!("{}, {}", name, slogan)
    };

    let phrase = write_message(String::from("We hit the ground every time."));
    println!("{}", phrase);


    //exceptions
    //export RUST_BACKTRACE=full
    // panic!("Sorry ,i panicked.");
    // panic_vector();

    let filename = "customer.json";
    match  File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => {
            // println!("{:#?}", error);
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create(filename) {
                        Ok(file) => {
                            println!("file created!");
                        }
                        Err(error) => {
                            println!("{:#?}", error);
                        }
                    }
                }
                _ => {
                    println!("{:#?}", error);
                }
            }
        }
    }

    let file_data = read_file(filename);
    match file_data {
        Ok(data) => {
            println!("ok: {}", data);
        }
        Err(_) => {}
    }


    // data structures

    let kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.1075,
        longitude: -81.85111
    };

    let kslc = Waypoint{
        name: "KSLC".to_string(),
        ..kcle
    };


    let kcle_kslc = Segment::new(kcle,kslc);

    // collections

    let mut flight :Vec<&str> = Vec::new();
    let vec_macro = vec![1,2,3];
    flight.push("new flight");

    for f in flight.iter() {
        println!("{}", f);
    }

    println!("{}", flight[0]);
    // let f = flight.get(0);
    let f = flight.get(1);
    match f {
        Some(str) => println!("{}", str),
        _ => println!("empty")
    }

    if let Some(flight_value) = flight.get(0) {
        println!("{}", flight_value);
    }

    flight.insert(1,"a c919 flight");
    flight.remove(1);


    // VecDequeue (Vector Double Ended Queue)
    // can add and remove from the front or the back
    // cannot sort elements of a VecDequeue

    let mut flights:VecDeque<&str> = VecDeque::new();
    flights.push_front("front flight");
    flights.push_back("back flight");
    flights.push_front("front flight again");
    flights.push_front("front flight again and again");
    flights.push_back("back flight again");
    flights.push_back("back flight again and again");

    for f in flights.iter() {
        println!("{}", f);
    }
    println!("{}", flights.len());

    println!("{}", flights.contains(&"again"));
    flights.clear();


    // iter_mut


    // Sequence vs Map Collections

    // sequence collections | Map Collections
    // Store entries in a list sequentially | Store entries as key value pairs
    // Has a single generic data type for entries | Has two generic data types, One for the keys and one for the value

    // Hash Map and Btree Map

    // Default Rust collections do not have key collision checking

    let mut flight_maps  = HashMap::new();
    flight_maps.insert("DA918", ("11:12","Dorando"));
    flight_maps.insert("DA418", ("12:05","Salt Lake City"));
    let flight_number = "DA918";
    let option = flight_maps.get(flight_number);
    let (time, destination) = option.unwrap();
    println!("{} - {}", time, destination);


    if !flight_maps.contains_key(flight_number) {
        flight_maps.insert(flight_number, ("12:00", "Puerto Pico"));
    } else {
        println!("Flight {} is already entered!", flight_number);
    }

    // Sets

   let mut flights_hash_set = HashSet::new();
   flights_hash_set.insert(("DA90", "11:12", "Drlando"));
   flights_hash_set.insert(("DA428", "12:15", "London"));
    flights_hash_set.insert(("DA90", "11:12", "Drlando"));
    // set does not have orders

    for f in flights_hash_set.iter() {
        println!("{:?}", f);
    }

    // generics
    let number_list = vec![1,2,3];
    println!("{}", largest(&number_list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let tuple = Point { x: (1,2), y: (3,4) };
    println!("{:#?}", integer);
    println!("{:#?}", float);
    println!("{:#?}", tuple);
    println!("{:#?}", tuple.x());
    let integer2 = Point2 { x: 5, y: 10.1 };
    let float2 = Point2 { x: 1.0, y: 4 };
    let tuple2 = Point2 { x: (1,2), y: 4.5 };
    println!("{:#?}", integer2);
    println!("{:#?}", float2);
    println!("{:#?}", tuple2);

    let p1 = Point2{x: 5, y: 10.4};
    let p2 = Point2{x: "Hello", y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer_i32 = Option_i32::Some(3);
    let float_f64 = Option_f64::Some(5.0);
    println!("{:#?}", integer_i32);
    println!("{:#?}", float_f64);


    //traits

    let news = NewsArticle{
        headline:String::from("Today WC begin."),
        location:String::from("Qatar"),
        author:String::from("wbb"),
        content:String::from("FIFA WC 2022")
    };
    println!("{}", news.summarize());
    notify(&news);
    notify1(&news);

}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (&self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

fn largest<T :std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}

fn panic_vector() {
    let vector = vec![1,2,3,4,5];
    println!("{}", vector[10]);
}

fn print_original(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
}
fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("next value");
    println!("fn change_original next: \t\"{}\"", next);
    // println!("fn change_original original: \t\"{}\"", original);
}
fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 < p2 {
        p1
    } else {
        p2
    }
}

// fn lifetime_syntax<'a, 'b>(p1: &'a i32, p2: &i32, p3: &'b f64) {
//
// }
// functions
fn return_greater(first: u8, second: u8) -> u8 {
    if first > second {
        first
    } else {
        second
    }
}



fn return_one_parameter(value: &i32) -> &i32 {
    value
}

// fn return_bad_ref() -> &i32 {
//     let value = 5;
//     &value
// }
fn print_nav_aid(navaid: &NavigationAids) {
    match navaid {
        NavigationAids::NOB(khz) => {
            println!("NOB frequency is {} kilohertz", khz);
        }
        NavigationAids::VOR(id, freq) => {
            println!("VOR identifier is  {} and it's frequency is {} kilohertz", id, freq);
        }
        NavigationAids::VORDME(id, freq) => {
            println!("VORDME identifier is  {} and it's frequency is {} kilohertz", id, freq);
        }
        NavigationAids::FIX{name, latitude, longitude} => {
            println!("FIX {} is at {} latitude and {} longitude", name, latitude, longitude);
        }
    }
}
