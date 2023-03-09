fn main() {
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{},{}",hello,world);

    // 字符串字面值就是切片
    let a = "rust";

    //定义函数时使用字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能
    // fn find_first(s: &str) -> &str {}
}
