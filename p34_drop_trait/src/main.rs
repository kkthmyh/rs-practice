// 实现drop trait，可以让我们自定义当值离开作用域时发生的动作

struct TestDrop {
    name: String,
}

impl Drop for TestDrop {
    fn drop(&mut self) {
        println!("{}", &self.name);
    }
}

fn main() {
    let a = TestDrop {
        name: "rust".to_string(),
    };
    let b = TestDrop {
        name: "cpp".to_string(),
    };
    println!("Hello, world!");
}
