// 枚举
enum IpAddrKind{
    V4,
    V6,
}

// 枚举作为struct的字段类型
struct IpAddr{
    kind : IpAddrKind,
    address: String,
}

// 枚举字段可以设置类型
enum IpAddrKind2{
    V4(u8,u8,u8,u8),
    V6(String),
}


fn main() {

    let four  = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let ip1 = IpAddrKind2::V4(127,0,0,1);
    let ip2 = IpAddrKind2::V6(String::from("ipv6"));
}


fn route(kind: IpAddrKind) {
}