use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn add_one_v1 (x: u32) -> u32 {x + 1}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",
             user_pref2, giveaway2
    );

    let expensive_closure = | num : u32 | -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let add_one_v2 = | x : u32| ->u32 {x + 1};
    println!("add_one_v2: {}", add_one_v2(2));
    let add_one_v3 = |x :i32|   {x +2};
    let add_one_v4 = |x: i32|   x + 1;

    let example_closure = |x| x;
    let n = example_closure(5);
    // let s = example_closure(String::from("hello"));

   let list = vec![1,2,3] ;
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();

    println!("After calling closure: {:?}", list);

    let mut list2 = vec![1,2,3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(4);
    borrows_mutably();
    println!("After calling  closure: {:?}", list2);

    let list3 = vec![1,2,3];
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();


    let mut rec_list = [
        Rectangle{width: 10, height: 1},
        Rectangle{width: 3, height: 5},
        Rectangle{width: 7, height: 12},
    ];

    rec_list.sort_by_key(|r| r.width);
    println!("{:#?}", rec_list);

    // let mut sort_operations = vec![];
    let mut num_sort_operations = 0;
    let value = String::from("by key called");
    rec_list.sort_by_key(|r| {
        // sort_operations.push(value);
        num_sort_operations+= 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations.", rec_list);

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1 = vec![1,2,3];
    let v2 :Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total :i32 = v1_iter .sum();
    assert_eq!(total, 6);
}
