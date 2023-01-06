use std::cell::RefCell;
use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;

#[derive(Debug)]
enum List{
    Cons(i32, Rc<List>),
    Nil,
}

enum List2{
    Cons(Rc<RefCell<i32>>, Rc<List2>),
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

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}
fn main() {
    let b = Box::new(1);
    println!("b = {}", b);
    // let list = Cons(1, Rc::new(Cons(2, Box::new(Cons(3, Rc::new(Nil))))));
    // println!("list = {:#?}", list);

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
    hello(&m); // equal to the bottom line
    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("my other stuff"),
    };


    println!("CustomSmartPointers created.");
    // c.drop();
    println!("{}", c.data);
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
    // println!("{}", c.data);

    let list_a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&list_a));
    let list_b = Cons(3, Rc::clone(&list_a));
    println!("count after creating b = {}", Rc::strong_count(&list_a));
    {
        let list_c = Cons(4, Rc::clone(&list_a));
        println!("count after creating c = {}", Rc::strong_count(&list_a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&list_a));

    let value = Rc::new(RefCell::new(5));
    let list2_a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let list2_b = Cons(Rc::new(RefCell::new(3), Rc::clone(&list2_a)));
    let list2_c = Cons(Rc::new(RefCell::new(4), Rc::clone(&list2_a)));
    *value.borrow_mut() += 10;
    println!("list2_a after = {:?}", list2_a);
    println!("list2_b after = {:?}", list2_b);
    println!("list2_c after = {:?}", list2_c);

}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
