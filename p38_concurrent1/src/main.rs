// Rust 标准库仅提供1:1模型的线程
// 通过thread::spawn函数可以创建线程
// rust中如果主线程执行结束，那么其他线程将会被终止，可以通过使用join handle来等待所有线程执行完成

// Move闭包
// move闭包通常和thread::spawn函数一起使用，它允许你使用其他线程的数据，即创建线程时，将值的所有权移交给其他线程
use std::{thread, time::Duration};
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn thread print number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread print number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    test_move();
}

fn test_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here is a vector:{:?}", v);
    });
    handle.join().unwrap();
}
