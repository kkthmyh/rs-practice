fn main() {
    println!("Hello, world!");
    another_func();
    another_func1(2);
    println!("five() is {}",five());
}

// 无参数
fn another_func() {
    println!("this is another func");
}
// 带参数
fn another_func1(x:i32) {
    println!("this is another func2 {}",x);
}

// 带返回值
fn five() -> i32{
    5
}