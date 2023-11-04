// defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// implemeting trait on type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// implementing default behavior
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// impl Summary for NewsArticle {}

// calling other methods in the same trait
// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("(Read more from {}...)", self.summarize_author())
//     }
// }

// impl Summary for Tweet {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// trait as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// receiving two parameters that implements trait
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     // do something
// }

// ensuring same type (T) via trait bound syntax
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     // do something
// }

// trait bounds with the + syntax
// use std::fmt::Display;

// pub fn notify(item: &(impl Summary + Display)) {
//     // do something
// }

// with generics
// pub fn notify<T: Summary + Display>(item: &T) {
//     // do something
// }

// trait bounds with where cluases
// instead of writing this
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     // do something
// }

// prefer writing this
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     // do something
// }

// returning types that implement trait
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people."),
//         reply: false,
//         retweet: false,
//     }
// }

// conditionally implementing methods on
// a generic depending on trait bounds

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     // only implements cmp_dipsplay if Pair<T>
//     // implements Display and PartialOrd
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// conditionally implement a trait for
// any type that implements another trait
// impl<T: Display> ToString for T {
//     // do something
// }
