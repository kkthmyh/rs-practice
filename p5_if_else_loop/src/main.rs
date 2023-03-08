fn main() {
    // if else
    let num = 4;
    if num < 5 {
        println!("small")
    } else {
        println!("big")
    }

    // loop
    let mut i = 0;
    loop {
        println!("loop");
        if i < 10 {
            i += 1;
        }else {
            break;
        }
    }

    // for

    let list = [1,2,2,3,3];
    for item in list.iter() {
        println!("{}",item);
    }
}
