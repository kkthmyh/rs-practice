use std::{fmt::Display, str};

// struct的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 生命周期省略
// 规则1应用于输入生命周期
// 规则2、3应用于输出生命周期
// 如果编译器应用完此3个规则后，仍然有无法确定的生命周期引用就会报错，需要我们显式标注
// 这些规则适用于fn定义及impl块
// 1. 每个引用类型的输入参数都有自己的生命周期
// 2. 如果只有一个输入生命周期参数，那么该生命周期会被赋给所有输出的生命周期
// 3. 如果有多个输入生命周期参数，但其中一个是&self或者是&mut self，那么self的生命周期会被赋给所有的输出生命周期参数

// 例子1
// fn first_word(s: &str) -> &str{}   根据规则1得到如下
// fn first_word<'a>(s: &’a str) -> &str{}   根据规则2得到如下
// fn first_word<'a>(s: &’a str) -> &‘a str{}   根据规则2得到如下

// 例子2
// fn longest(x: &str,y: &str) -> &str{} 根据规则1得到如下
// fn longest<'a,'b>(x: &'a str,y: &'b str) -> &str{} 规则2、3都不适用，编译器无法计算返回值的生命周期，所以需要显示标注

// 例子3
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }

//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         self.part
//     }
// }

// 静态生命周期
// ‘static是一个比较特殊的生命周期，表示的是整个程序的持续时间
// - 例如所有的字符串字面值都拥有‘static生命周期
// let s: &'static str = "i have a static lifetime";

// 泛型参数类型、trait bound 、生命周期

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not found a '.'");
    // 这里由于first_sentence变量的生命周期是整个代码块的，所以可以作为结构体的part参数
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a, T>(x: &'a str, y: &'a str, t: T) -> &'a str
where
    T: Display,
{
    println!("t is {}", t);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
