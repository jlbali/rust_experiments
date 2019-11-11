fn main() {

    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

}


/*
enum IpAddressKind{
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

*/

enum IpAddress{
    V4(String),
    V6(String),
}