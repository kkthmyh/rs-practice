// if let 处理只关心一种匹配而忽略其他匹配的情况

fn main() {

    let num = Some(0);
    // 假设当前我们只处理Some(3)的情况,可以使用if let代替
    match num {
        Some(3) => println!("3333"),
        _ =>println!("others"),
    }

    if let Some(3) = v{
        println!("3333");
    }else {
        println!("others");
    }

}
