fn main() {
    // 这种写法编译会报错，因为s1的值指向了s2，原来指向s1的引用被打断
    // let s1 = String::from("hello");
    // |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // 4 |     let s2 = s1;
    // |              -- value moved here
    // 5 |     println!("s1 is {}",s1);
    // |                         ^^ value borrowed here after move

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 is {}", s1);

    // 采用clone可以复制Heap上的数据
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 is {}", s3);
}
