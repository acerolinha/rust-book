// [Loops]
// looping through collections
// with for
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // iterating over a range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

// with while
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// while
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");
//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// loop labels
// fn main() {
//     let mut count = 0;
//     // Defining the loop label.
//     'counting_up: loop { // must start with single quote (')
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 // Breaks the inner loop.
//                 break;
//             }
//             if count == 2 {
//                 // Breaks the labeled loop.
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// returning value from loop
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             // Returning a value out of the loop with 'break'.
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// loop
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// [If Expressions]
// if in a let statement
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     // The next line would generate an error of incompatible types.
//     // let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// multiple conditions with else if
// fn main() {
//     let number = 6;

//     // This could be refactored to a 'match' to improve readability.
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 3");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// handling if else
// fn main() {
//     let number = 7;

//     // The following block won't work as conditions must be a 'bool'.
//     // if number {

//     // }

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
