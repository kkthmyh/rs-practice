// Unsafe超能力
// 使用unsafe来将代码切换到unsafe rust
// unsafe代码块中可以执行四个动作
// 1 解引用原始指针
// 2 调用unsafe 函数或者方法
// 3 访问或者修改可变的静态变量
// 4 实现unsafe trait

// 解引用原始指针
// 原始指针
// 可变的：*mut T
// 不可变的：*const T

// 与引用的区别
// 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用检查
// 无法保证能指向合理位置
// 允许为null
// 不实现任何自动清理

// 放弃保证安全来换取更好的性能或与其他语言接口的能力
fn main() {
    let mut num = 5;
    // 创建原始指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 {}", *r1);
        println!("r2 {}", *r2);
    }

    // 调用unsafe函数或方法
    unsafe {
        test_unsafe();
    }
}

unsafe fn test_unsafe() {
    println!("this is unsafe func")
}

// unsafe trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}
             