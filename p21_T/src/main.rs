// 泛型

// struct中的泛型
struct Point<T> {
    x: T,
    y: T,
}
struct Point2<T, U> {
    x: T,
    y: U,
}

// 枚举中的泛型
enum Option<T> {
    Some(T),
    None,
}

// impl中的泛型
impl <T> Point<T> {
    
    fn x(&self) -> &T {
        &self.x
    }
        
}


fn main() {
    println!("Hello, world!");
}
