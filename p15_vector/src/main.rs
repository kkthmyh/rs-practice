// Vector
fn main() {
    // 创建
    let v1: Vec<i32> = Vec::new();
    let v2: Vec<u32> = vec![1, 2, 3];

    // 添加元素
    let mut v = Vec::new();
    v.push(1);

    // 读取元素
    let v2 = vec![1, 2, 3, 4, 5, 6];
    let a = &v2[0];
    println!("a is {}", a);

    match v2.get(2) {
        None => print!("is None"),
        Some(i) => println!("is {}", i),
    }

    // 所有权和借用规则
    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];
    // 这里编译会出错
    // v3.push(6);
    println!("The first element is {}", first);

    // 遍历
    for item in v3.iter() {
        println!("element is {}", item)
    }

    for item in &v3 {
        println!("element is {}", item)
    }

    // 遍历改变元素值
    for item in &mut v3 {
        *item += 1;
    }

    println!("vec is {:?}", v3)
}
