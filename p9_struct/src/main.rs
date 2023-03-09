// 普通struct
#[derive(Debug)]
struct User{
    username: String,
    email:String,
    sign_in_count:u64,
    active:bool,
}

// tuple struct
#[derive(Debug)]
struct Point(i32,i32,i32);



fn main() {
    println!("Hello, world!");

    let user1 = User{
        username: "sayram".to_string(),
        email: "sayram@123.com".to_string(),
        sign_in_count: 0,
        active: false,
    };

    println!("user is {:?}",user1);

    let p1 = Point(1,2,3);
    println!("point is {:?}",p1);

}
