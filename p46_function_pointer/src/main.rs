// 函数指针

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, args: i32) -> i32 {
    f(args) + f(args)
}
fn main() {
    let res = do_twice(add_one, 10);
    println!("res {}", res);
}

// 返回闭包
// 这里编译不过，因为编译器无法确定返回的闭包所需要的空间大小
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
