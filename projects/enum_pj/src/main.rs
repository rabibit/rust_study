#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAdd {
    kind: IpAddKind,
    address: String,
}

fn main() {
    let home = IpAdd {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAdd {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };
    
    println!("Hello, world!");
    println!("Hello, world! my home address is {:#?}", home);
    println!("Hello, world! loop back address is {:#?}", loopback);
}
