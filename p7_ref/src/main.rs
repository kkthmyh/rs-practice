fn main() {
    // &表示允许引用某些值而不获取所有权
    let s1 = String::from("hello");
    let l = cal_len(&s1);
    println!("s1 is {},len is {}", s1, l);

    // 创建可变引用
    let mut s3 = String::from("hello");
    let len = cal_len2(&mut s3);
    println!("s3 is {},len is {}", s1, len);

    // 不可以将一个可变引用同时赋值给超过一个变量，好处是在编译期间就可以防止数据竞争
    let mut str1 = String::from("Rust");
    let a = &mut str1;
    let b = &mut str1;
    // 这里会报错
    // error[E0499]: cannot borrow `str1` as mutable more than once at a time
    //     --> src/main.rs:15:13
    //     |
    //     14 |     let a = &mut str1;
    // |             --------- first mutable borrow occurs here
    // 15 |     let b = &mut str1;
    // |             ^^^^^^^^^ second mutable borrow occurs here
    // 16 |     println!("a is {},b is {}", a, b)
    //     |                                 - first borrow later used here
    println!("a is {},b is {}", a, b);



    // 不可以同时拥有一个可变引用和一个不可变引用
    // 多个不可变引用是可以的
    let mut str1 = String::from("hello");
    let r1  = &str1;
    let r2 = &str1;
    // 这里编译直接报错
    let r3 = &mut str1;


    // 悬垂指针
    // 这里编译也会直接出错，因为s在方法执行完就已经回收了，不允许指向一个空内存片段
    let r = dangle();




}

fn cal_len(s: &String) -> usize {
    s.len()
}

fn cal_len2(s: &mut String) -> usize {
    s.push_str("world");
    s.len()
}

fn dangle() -> &String {
    let s = String::from("rust");
    &s
}
