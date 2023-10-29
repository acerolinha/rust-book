// the option enum
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y = Some(5);

    // The next line would trhow an error
    // because y can possibly be not defined
    // let sum = x + y;

    // clear warnings
    println!("{some_number:#?}");
    println!("{some_char:#?}");
    println!("{absent_number:#?}");
    println!("{x:#?}");
    println!("{y:#?}");
}

// enums with associated values
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// // each enum variant can have different types
// enum IpAddr2 {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// // like structs, enums can also have methods
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {}
// }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
//     let ip2 = IpAddr2::V4(8, 8, 8, 8);

//     let m = Message::Write(String::from("Hello"));
//     m.call();
// }

// defiing an enum
// enum IpAddrKind {
//     V4,
//     V6,
// }

// fn main() {
//     // instatiating an enum
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(four);
//     route(six);
// }

// // defining a function that takes any IpAddrKind
// fn route(_: IpAddrKind) {}
