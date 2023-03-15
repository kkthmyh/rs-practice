// Deref Trait
// 实现deref trait可以让我们自定义解引用运算符的行为
// 通过实现deref 智能指针可以像常规引用来处理

use std::ops::Deref;

// 实现自定义的Box指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 10;
    let y = MyBox::new(x);

    assert_eq!(10, *y);

    let a = "rust";
    hello(a);
    // 实现了deref trait 所以会
    // &b    &Box<String>
    //       deref &String
    //       deref &str
    let b = Box::new("cpp");
    hello(&b);
}

fn hello(name: &str) {
    println!("hello,{}", name);
}
