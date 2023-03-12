use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};
fn main() {
    //  使用match来处理异常
    // let f = File::open("hello.txt");
    // let file = match f {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("error creating file:{:?}", e),
    //         },
    //         _ => panic!("error open the file"),
    //     },
    // };

    // 使用unwarp_or_else优雅处理
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("error creating file:{:?}", error);
    //         })
    //     } else {
    //         panic!("error open the file");
    //     }
    // });

    //  使用unwrap 约等于 使用match表达式
    // let f = File::open("hello.txt").unwrap();

    // 使用expect 好处是可以自定义错误信息
    // let f = File::open("hello.txt").expect("can not open the file");

    // 错误的传递 使用？
    // 我们来看一个读取文件的例子
    let re = read_from_file();
}

fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
