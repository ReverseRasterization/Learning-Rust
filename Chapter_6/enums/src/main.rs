/*
enum Option<T> {
    None,
    Some(T),
}
*/

enum IpAddr {
    V4(String),
    V6(String),
    V4_ex(u8, u8, u8, u8),
}

enum Message {
    Quit,
    Move { x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    /* 
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    */

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let home_2 = IpAddr::V4_ex(127,0,0,1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; // You may not use an Option<i32> with a regular <i32> until you have covered Option<i32> to an i32
}

//fn route(ip_kind: IpAddrKind) {}
