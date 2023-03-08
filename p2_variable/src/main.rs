fn main() {
    println!("Hello, world!");
    // 定义不可变的变量
    let x = 5;
    println!("num is {}", x);
    // shadowing 变量覆盖
    let x = x + 1;
    println!("num is {}", x);
    // 定义常量
    const NUM: i32 = 10;

    let spaces = "   ";
    let spaces = spaces.len();

}
