// shadowing
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// variables and constants
// const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");

//     // The next line would generate an error if 'x' was immutable.
//     x = 6;
//     println!("The value of x is: {x}");

//     println!("Three hour in seconds: {THREE_HOUR_IN_SECONDS}");
// }
