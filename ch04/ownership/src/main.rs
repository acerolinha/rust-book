// cloning avoids move
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

// trying to use a variable after being moved
// fn main() {
//     let first = String::from("Ferris");
//     let full = add_suffix(first);
//     println!("{full}");
//     // The next line would cause an error because first was already freed.
//     // println!("{full}, originally {first}");
// }

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// moving the ownership
// fn main() {
//     let a = Box::new([0; 1_000_000]); // until this line a is the owner of the box
//     let b = a; // the ownership of the box is moved from a to b
// }

// box owner managing deallocation
// fn main() {
//     let a = 4; // 0. allocates 4 on the stack
//     make_and_drop(); // 2. deallocate 5 of the heap and it's pointer of the stack
// }

// fn make_and_drop() {
//     let b = Box::new(5); // 1. allocates 5 on the heap and the pointer to it on the stack
// }

// boxes in the haep
// fn main() {
//     let a = Box::new([0; 1_000_000]); // 0. stack: a = |-> h_0 | haep: h_0 = [0,0,0,0,...,0]
//     let b = a; // 1. stack: b = |-> h_0 | haep: h_0 = [0,0,0,0,...,0]
//     // Manual memory management is not allowed.
//     // free(b);
// }

// variables in the stack
// fn main() {
//     let n = 5; // 1. stack: n = 5
//     let y = plus_one(n); // 3. stack: n = 5; y = 6
//     println!("The value of y is: {y}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1 // 2. stack: n = 5; x = 5
// }
