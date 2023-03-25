// 关联类型与范型的trait
// 范型
// 每次实现trait时需要标注类型
// 可以为同一类型多次实现某个trait

// 关联类型
// 无需标注类型
// 无法为单个类型多次实现某个trait

struct Counter {}
// 范型trait
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// 实现u32的trait
impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}

// 实现String的trait
impl Iterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        todo!()
    }
}

// 关联类型的trait
pub trait Iterator2 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator2 for Counter {
    // 这里需要指明具体类型
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// 完全限定语法，如何调用同名方法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("pilot fly")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard fly")
    }
}

impl Human{
    fn fly(&self) {
        println!("human fly")
    }
}


fn main() {

    let person = Human;
    person.fly();
    // 调用同名方法
    Pilot::fly(&person);
    Wizard::fly(&person);

 }
