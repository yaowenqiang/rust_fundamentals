use crate::List::{Cons, Nil};
use std::ops::Deref;
#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn main() {
    let b = Box::new(1);
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:#?}", list);

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let a = 5;
    let b = Box::new(x);
    assert_eq!(5, a);
    assert_eq!(5, *b);

    let c = 5;
    let d = MyBox::new(c);
    assert_eq!(5, c);
    assert_eq!(5, *d);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(nane: &str) {
    println!("Hello, {name}!");
}
