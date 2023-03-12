// &i32 一个引用
// &’a i32 带有显式生命周期的引用
// &‘a mut i32 带有显式生命周期的可变引用
fn main() {
    let s1 = String::from("rust");
    let result;
    {
        let s2 = String::from("cpp");
        result = longest(&s1, &s2);
    }
    println!("result is {}",result);

    // 上述代码会编译失败
    // 因为s2的生命周期短于s1，longest函数返回值可能指向s2，所以编译无法通过

}

// 实际上此函数返回的是s1 和 s2 生命周期最短的那个
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}