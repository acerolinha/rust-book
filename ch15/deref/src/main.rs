use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implementing Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, *y);

    let m = MyBox::new(String::from("Rust"));
    // deref coercion
    hello(&m);
    // hello(&(*m)[..]); // equivalent to above
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
