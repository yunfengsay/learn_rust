
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = Ipaddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

fn main() {
    println!("loopback is {:?}", loopback);
}
