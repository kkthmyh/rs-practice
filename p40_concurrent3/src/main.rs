use std::{
    sync::{Arc, Mutex},
    thread,
};

// 使用共享内存并发 Mutex
// Mutex是互斥锁
// 在同一时刻Mutex只允许一个线程访问某些数据
// 想要访问数据必须首先获取互斥锁lock

// 多线程情况下数据的多重所有权需要使用Arc<T>智能指针
// 使用Arc来进行原子引用计数，可以用在并发场景，类似于Rc指针，但是Rc指针不能用于多线程环境，本质是因为没有实现Mutex的trait

// RefCell<T>/Rc<T> vs Mutex<T>/Arc<T>
// Mutex<T>提供了内部可变性，和Cell家族一样
// 使用RefCell<T>来改变Rc<T>里面的内容
// 使用Mutex<T>来改变Arc<T>里面的内容
// 使用Mutex<T>可能存在死锁风险

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
