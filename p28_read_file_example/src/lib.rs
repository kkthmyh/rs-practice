// 返回了一个实现了error trait的类型

use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for content in contents.lines() {
        if content.contains(query){
            res.push(content);
        }
    }
   res
}

#[cfg(test)]
mod tests {

    use super::*;
    fn one_result() {
        let query = "duct";
        let contents = "/
Rust:
safe , fast , productive.
Pick three.";
        assert_eq!(vec!["safe , fast , productive."], search(query, contents));
    }
}
