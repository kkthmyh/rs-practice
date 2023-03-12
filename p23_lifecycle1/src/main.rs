// 生命周期
// 主要作用是防止悬垂引用

fn main() {
    // 这个例子简单的说明了生命周期
    // {
    //     let x;                  // - - - - - - + - - - 'a start
    //     {                             //             |
    //         let y = 10;          // - + - 'b s  |
    //         x = &y;                   //   |         |
    //     }                             // - + - 'b e  |
    //     println!("x is {}", x);       //             |
    // }                                 // - - - - - - + - - - 'a end

    // 可以看到 y的生命周期是短于x的生命周期的，现在将x指向一个已经消失的内存区域显然是不合理的，所以编译出错

    // 函数的生命周期
    let s1 = String::from("rust");
    let s2 = String::from("cpp");

    let result = longest(&s1, &s2);
    println!("longest is {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
