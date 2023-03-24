use std::string;

fn main() {
    let x = 2;
    // 匹配字面值
    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("anything"),
    }

    // 匹配变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("matched ,y = {:?}", y),
        _ => println!("nothing"),
    }
    println!("at the end x = {:?}, y = {:?}", x, y);

    // 多重模式｜
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }

    // 范围模式
    let x = 5;
    match x {
        1..=5 => println!("one to five"),
        _ => println!("nothing"),
    }

    // 解构值

    let point = Point { x: 30, y: 0 };
    match point {
        Point { x, y: 0 } => println!("y is 0"),
        Point { x: 0, y } => println!("x is 0"),
        _ => println!("nothing"),
    }

    // 使用.. 忽略不需要的值
    let numbes = (1, 2, 3, 4, 5);
    match numbes {
        (x, .., y) => {
            println!("{},{}", x, y);
        }
        _ => {
            println!("nothing");
        }
    }

    // match守卫 match后还可以进行条件判断
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => {
            println!("yes");
        }
        _ => {
            println!("nothing");
        }
    }

    // @绑定
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_varilable @ 3..=7,
        } => {
            println!("Fount an id in range : {}", id_varilable)
        }
        _ => println!("nothing"),
    }
}

struct Point {
    x: u32,
    y: u32,
}

enum Message {
    Hello { id: i32 },
}
