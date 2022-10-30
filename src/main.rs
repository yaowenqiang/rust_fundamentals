#![allow(unused_variables)]

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

fn main() {
    let unused_variables: u32 = 0;
    let location: [f32; 2] = [0.0, 0.0];
    let location2: [f32; 20] = [0.0; 20];
    let location3: (&str, f64, f64) = ("POLE", 0.1, 0.2);
    println!("Hello, world!");
    println!("location name: {}, latitude:{}, longitude: {} ", location3.0, location3.1, location3.2);
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


    let ndb_val = NavigationAids::NDB(385);
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

}

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
        NavigationAids::FIX(name, latitude, longitude) => {
            println!("FIX {} is at {} latitude and {} longitude", name, latitude, longitude);
        }
    }
}
