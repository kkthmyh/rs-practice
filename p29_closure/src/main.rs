// 闭包的定义
// let add1 = |x: u32| -> u32{x+1}
// let add2 = |x| {x+1};
// let add3 = |x|  x+1;

//  闭包从所在环境中捕获值的方式
// 1、取得所有权： FnOnce
// 2、可变借用： FnMute
// 3、不可变借用： Fn

// Move关键字
// 在参数列表前使用Move关键字，可以强制闭包取得环境值的所有权
fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("rust"));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z = x;
    //  这里x的所有权已经被move到闭包里面了，所以编译报错
    println!("can not use x here :{:?}",x);
}
