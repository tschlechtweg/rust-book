// enumeration: IpAddrKind
// two variants: V4, V6
enum IpAddr {
    // variants can have embedded value
    // structs, primitives, enums, ...
    V4(u8, u8, u8, u8),
    V6(String),
}

// enumeration
// all grouped under one type
enum Message {
    Quit,
    // can also name the fields, similar to structs
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move{x: _, y: _} => println!("Move"),
            Message::Write(_) => println!("Write"),
            Message::ChangeColor(_, _, _) => println!("ChangeColor"),
        }
    }
}

fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    // variant name becomes instance creator
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let testmsg = Message::Move{x: 32, y: 64};
    testmsg.call();
}
