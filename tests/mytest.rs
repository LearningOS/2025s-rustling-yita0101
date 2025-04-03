


use std::thread;
use std::time::Duration;

#[test]
pub fn demo01(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

}