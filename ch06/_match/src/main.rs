// catch-all and _ placeholder
fn main() {
    let dice_roll = 9;

    // defining default action
    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        other => println!("move player {other} spaces"),
    }

    // if you don't need to use the value use _ placeholder
    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        _ => println!("reroll"),
    }

    // if you don't want to perform any action use unit value ()
    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        _ => (),
    }
}

// matching with Option<T>
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// this won't compile because match patterns
// must be exhaustive
// fn plus_two(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 2),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     assert_eq!(Some(6), six);
//     assert_eq!(None, none);
// }

// matching and value binding
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     // Alaska,
//     // ...
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     // defining a match
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         // if you want to run multiple lines of code
//         // use curly brackets
//         Coin::Dime => {
//             println!("Lucky penny!");
//             10
//         }
//         // patterns that binds to values
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
//     println!("{}", value_in_cents(Coin::Penny));
//     println!("{}", value_in_cents(Coin::Nickel));
//     println!("{}", value_in_cents(Coin::Dime));
//     println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
// }
