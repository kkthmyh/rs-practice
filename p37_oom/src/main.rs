// 如何防止循环引用
// 依靠开发者来保证 不能依靠rust编译器
// 重新组织数据结构，一些引用来表达所有权 一些引用不表达所有权

// 将Rc<T> 换成 Weak<T>
// Rc::clone为Rc<T>实例的strong_count加1，Rc<T>实例只有在strong_count 为0的时候才会被清理
// Rc<T> 实例通过调用Rc::downgrade方法可以创建值的Weak Refence ，每次调用时weak_count +1
// weak_count 不为零不影响Rc<T>实例的清理
use crate::Node::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Node {
    Cons(i32, RefCell<Rc<Node>>),
    Nil,
}

impl Node {
    fn tail(&self) -> Option<&RefCell<Rc<Node>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a inital rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // 这里其实创建了一个循环引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // 这里会导致栈溢出
    // println!("a next item = {:?}", a.tail());
}
