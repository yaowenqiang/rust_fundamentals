use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        // for i in 1..10 {
        //     println!("hi number {} from the spawned thread!", i);
        //     thread::sleep(Duration::from_millis(1));
        // }
        println!("Here's a vector: {:?}", v);
    });
    // for i in 1..10 {
    //     println!("hi, number {} from the main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // drop(v);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    handle.join().unwrap();
}
