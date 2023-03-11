// String 和 &str

fn main() {

    // String类型不支持按照索引位置访问 如下
    // let s12 = String::from("rust");
    // let s13 = &s12[0];

    // 内部表示
    // String是对Vec<u8>的包装
    let s14 = String::from("你好！");
    for i in s14.bytes() {
        println!("{}" , i);
    }
    // unicode标量
    for i in s14.chars() {
        println!("{}" , i);
    }

    // 切割String
    let s15 = String::from("大家好，我叫张三！");
    let s16 = &s15[0..6];
    println!("s16 is {}" , s16);


    


    // 创建1个新的字符串
    let s1 = String::new();
    let s2 = "rust".to_string();
    let s3 = String::from("rust");

    // 更新String
    // push_str()
    let mut s4 = String::from("hello ");
    s4.push_str("world");
    println!("s4 is {}",s4);

    // push()
    let mut s5 = String::from("hello ");
    s5.push('!');


    // +号拼接 注意拼接之后s6会丢失所有权
    // 可以看作内部调用了 fn add(self,s:&str) -> String{} rust会降&String自动转换成&str
    let s6 = String::from("hello");
    let s7 = String::from(" World");
    let s8 = s6 + &s7;


    // format！
    let s9 = String::from("hello");
    let s10 = String::from(" World");

    let s11  = format!("{}-{}",s9,s10);
    println!("s11 is {}",s11);



    
     
}
