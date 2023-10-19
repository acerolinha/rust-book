// rules of references
// 1. at any given time, you can have either
// one mutable reference or any number
// of immutable references

// 2. references must always be valid

// dangling references
fn main() {
    // let reference_to_nothing = dangle();
    let something = no_dangle();
    println!("{something}");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// } // s is dropped here, so it's reference would poind to invalid memory

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// we can, altough, have multiple immutable references simultaneously
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem

//     println!("{r1} and {r2}");
//     // r1 and r2 scopes end here

//     let r3 = &mut s; // no problem

//     println!("{r3}");
// }

// we can't have mutable and immutable references simultaneously
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG problem
//     // The above error occurs because immutable references
//     // doesn't expect to have it's value suddenly changed

//     println!("{r1}, {r2}, {r3}");
// }

// we can have two mutable references, but not simultaneously
// fn main() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//     }

//     let r2 = &mut s;
// }

// you can't have more than one mutable reference to a value
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;

//     // The next line won't work because we cannot more than one mutable reference
//     let r2 = &mut s;

//     println!("{r1}, {r2}");
// }

// modifying data through a mutable reference
// fn main() {
//     let mut s = String::from("Hello");

//     change(&mut s);

//     println!("{s}");
// }

// fn change(some_string: &mut String) {
//     // We can mutate because we have a mutable reference
//     some_string.push_str(", world!");
// }

// trying to modify a borrowed object
// fn main() {
//     let s = String::from("Hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     // The next line won't work because references are immutabel by default.
//     // We can set it to be mutable with &mut
//     some_string.push_str(", world!");
// }

// borrowing instead of giving onwership
// fn main() {
//     let s1 = String::from("hello"); // s1 comes into scope
//     let len = calculate_length(&s1); // instead of moving s1, we pass a reference to it
//     println!("The length of '{s1}' is {len}."); // s1 remains in the scope
// }

// fn calculate_length(s: &String) -> usize { // s is a refecenre to a String
//     s.len()
// } // s goes out of scope, but it's not dropped because it's not owned by the function
