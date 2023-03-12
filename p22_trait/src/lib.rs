use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}
// 定义trait
pub trait Summary {
    fn summarize(&self) -> String;
}
// 实现trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

// 使用trait作为参数
// 1、使用impl
pub fn notify1(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// 2、trait bound
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
// 3、使用where简化实现多个trait的情况
// 简化前
pub fn notify3<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Breaking news! {}", a.summarize())
}

// 简化后
pub fn notify4<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}

// 使用trait bound有条件的实现方法
// 在使用泛型类型参数的impl块上使用trait bound，我们可以有条件的为实现了特定trait的类型来实现方法

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }  
}
// 只有当T实现了display和partialord的trait，才可以使用cmp_display方法
 impl <T:Display+PartialOrd> Pair<T> {
     fn cmp_display(&self) {

     }
 }



// trait的覆盖实现 下面是标准库的代码截取
// 此代码的意思是，为所有现实了display trait的T类型，实现ToString trait，例如u8
//  impl<T: fmt::Display> ToString for T {
//  }
