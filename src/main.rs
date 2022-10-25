#![allow(unused_variables)]
fn main() {
    let unused_variables :u32 = 0;
    let location : [f32;2] = [ 0.0, 0.0];
    let location2 : [f32;20] = [ 0.0; 20];
    let location3 : (&str, f64, f64) = ( "POLE", 0.1,0.2);
    println!("Hello, world!");
    println!("location name: {}, latitude:{}, longitude: {} ", location3.0, location3.1,location3.2);
    let (name, latitude, longitude) = location3;
    println!("location name: {}, latitude:{}, longitude: {} ", name, latitude,longitude);
}
