// other slices
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// string slices
// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{s}");
//     println!("{hello} {world}");

//     // if starting from index 0, leftside can be omitted
//     let slice = &s[0..2];
//     println!("{slice}");
//     let slice = &s[..2];
//     println!("{slice}");

//     // if slice include the last byte, rightside can be omitted
//     let len = s.len();

//     let slice = &s[3..len];
//     println!("{slice}");
//     let slice = &s[3..];
//     println!("{slice}");

//     // if both indexes are omitted, slice is whole string
//     let slice = &s[0..len];
//     println!("{slice}");
//     let slice = &s[..];
//     println!("{slice}");

//     let first_word = first_word(&s);
//     let first_word_literal = first_word_literal(&s);

//     println!("{}", first_word);
//     println!("{}", first_word_literal);

//     // The next line would throw an error because we have
//     // a mutable and a immutable reference would coexist
//     // s.clear();
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn first_word_literal(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }
