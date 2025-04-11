


use std::thread;
use std::time::Duration;

#[test]
pub fn demo01(){
    let x = 123;
    let r1 = &x;               // 引用
    let r2 = &x as *const i32; // 原始指针

    println!("引用地址: {:p}", r1);
    println!("指针地址: {:p}", r2);
}