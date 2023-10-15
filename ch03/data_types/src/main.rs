// [Compound Types]
// array type
fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
    let c = [3; 2]; // would generate [3, 3];

    // accessing array elements
    let first = a[0]; // index starts at 0
    let second = a[1];
    // the next line would cause the program to panic with 'index out of bounds' error
    // let third = a[10];
}

// tuple type
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1); // with optional type annotation

//     // using pattern matching to destrucutre a tuple
//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");

//     // directly accessing elements of a tuple
//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;

//     // () // this is a tuple called unit, it represents an empty value or return type
// }

// [Scalar Types]
// characater type
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// boolean type
// fn main() {
//     let t = true;
//     let f: bool = false; // with explicit type annotation
// }

// numeric operations
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
// }

// floating point types
// fn main() {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
// }
