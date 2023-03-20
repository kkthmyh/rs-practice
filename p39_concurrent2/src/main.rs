// 基于消息传递的并发 channel

// 创建channel 使用mpsc::channel来创建，返回值是生产者和消费者
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (sender, receiver) = mpsc::channel();

    // 多个sender
    let sender2 = mpsc::Sender::clone(&sender);

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     sender.send(val).unwrap();
    //     // channel 发送时会移交数据的所有权，这里打印val的值，编译会报错
    //     // println!("val is {}",val);
    // });

    // 发送多个值
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("rust"),
            String::from("hello"),
            String::from("world"),
        ];

        for item in vals {
            sender.send(item).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2 hello"),
            String::from("2 rust"),
            String::from("2 hello"),
            String::from("2 world"),
        ];

        for item in vals {
            sender2.send(item).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for item in receiver {
        println!("Got : {}", item);
    }
}
