// lifetime annotation snyatx
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

use std::fmt::Display;

// in function signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// in struct definitions
#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// in method definitions
impl<'a> ImportantExcerpt<'a> {
    #[allow(dead_code)]
    fn level(&self) -> i32 {
        3
    }
}

// generics, traits and lifetimes all together
#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // works
    let result = longest("abc", "1234");
    println!("{result}");

    // works
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // won't work
    // for this to to be valid, string2 would
    // need to be valid until the end of the outer scope
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{i:#?}");

    // static lifetime
    #[allow(unused_variables)]
    let s: &'static str = "I have a static lifetime.";
}
