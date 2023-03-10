// Rust中不存在null这种概念，取而代之的是option<T>,它是一个枚举结构，其中包含两个
// 变体，Some(T) 和 None

// match 匹配必须穷举起所有可能性，可以用_通配符代替

enum Coin {
    BTC,
    ETH,
    LTC,
    DOGE,
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;

    let a = 8;
    let b: Option<i32> = Some(9);
    // 这里编译报错
    // error[E0277]: cannot add `Option<i32>` to `{integer}`
    // --> src/main.rs:13:15
    //     |
    //     13 |     let c = a + b;
    // |               ^ no implementation for `{integer} + Option<i32>`
    // |
    let c = a + b;
}


fn value(coin: Coin) -> u8 {
    match coin {
        Coin::BTC => 1,
        Coin::ETH => 2,
        Coin::LTC => 3,
        Coin::DOGE => 4,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

