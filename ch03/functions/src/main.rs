// function with return value
fn five() -> i32 {
    5 // implictly returns the last expression
}

fn plus_one(x: i32) -> i32 {
    x + 1 // adding a colon to the end of this line will generate an error
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of y is: {y}");
}

// statements and expressions
// fn main() {
//     let y = 6; // this is a statement
//                // the next line would generate an error as statements don't return a value
//                // let x = (let y = 6);

//     // the following block is an expression
//     let y = {
//         let x = 3;
//         x + 1 // note that this doesn't end with a ';', so it returns a value
//               // adding a colon to the end of an expression turns it into a statement
//     };

//     println!("The value of y is: {y}");
// }

// function with multiple parameters
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// function with single parameter
// fn main() {
//     another_function(5); // 5 is an argument
// }

// fn another_function(x: i32) { // x is a parameter
//     println!("The value of x is: {x}");
// }

// function without parameters
// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }
