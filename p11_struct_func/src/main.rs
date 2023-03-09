#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    // 这是方法，self作为第一个参数
    fn area(&self) -> u32 {
        self.width * self.length
    }
    // 这是关联函数，self不作为第一个参数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        length: 50,
    };
    println!("area is {}", rect.area());

    // 调用关联函数创建一个边长为10的正方形
    let s = Rectangle::square(10);
}

