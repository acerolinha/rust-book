// returning values
fn main() {
    let s1 = gives_ownership(); // 'gives_ownership' moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved to 'takes_and_gives_back' and is returned back to s3
    // The next line won't work because s2 was moved in the above line.
    // println!("{s2}");
    println!("{s1} {s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is moved to it's caller
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to it's caller
}

// ownership and functions
// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s is moved and cannot be used after this line

//     // The next line won't work because s was moved
//     // println!("{s}");

//     let x = 5; // x comes into scope

//     makes_copy(x); // x is cloned and can be used after this line
//     // this happens because i32 implements Copy trait

//     println!("{x}");
// }

// fn takes_ownership(some_string: String) { // some_string comes into scope

//     println!("{some_string}");
// } // some_string is dropped

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // some_integer is dropped

// cloning data
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // clone deeply copy, not only stack, but also heap data

//     println!("s1 = {s1}, s2 = {s2}");

//     // types that are stored in the stack don't need to be cloned
//     let x = 5;
//     let y = x; // this deeply copy x, since it's size is known

//     println!("x = {x}, y = {y}");
// }

// moving data
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1; // s1 was moved into s2

//     // The next line won't work because s1 was moved
//     // println!("{s1}, world!");
// }

// memory and allocation
// fn main() {
//     {
//         let s = String::from("Hello"); // s is valid from this point forward

//         // do stuff with s
//     } // this scope is now over, and s is no longer valid
// }

// the String type
// fn main() {
//     // Creates a String from a string literal.
//     let s = String::from("hello");
//     println! {"{s}"};

//     // String can be mutated
//     let mut s1 = String::from("Hello");
//     s1.push_str(", world!"); // push_str() appendas a literal to a String
//     println! {"{s1}"};
// }

// variable scope
// fn main() {
//     // s is not valid here, since it's not yet declared
//     {
//         let s = "hello"; // s is valid from this point forward

//         // do stuff with s
//     } // this scope is now over, and s is no longer valid
// }
