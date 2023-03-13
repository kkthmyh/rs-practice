// 我们来做一个grep函数用于在文件中搜索文字内容
use std::env;
use std::process;
use p28_read_file_example::Config;
fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = p28_read_file_example::run(config) {
        println!("Application error :{}",e);
        process::exit(1);
    }
}
