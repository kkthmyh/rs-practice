// 引用计数指针Rc
// 使用场景
// 1、需要在Heap上分配数据，这些数据被程序的多个部分读取（只读），但在编译时无法确定在哪个部分最后使用完这些数据
// 2、Rc<T>只能用于单线程场景


// Rc::clone() vs clone()
// Rc::clone()增加引用 不会执行数据的深度拷贝
// 类型的clone() 很多会执行数据的深度拷贝
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::Cons;
use crate::List::Nil;

fn main() {
    
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(4, Rc::clone(&a));
    let c = Cons(8, Rc::clone(&a));    
}
