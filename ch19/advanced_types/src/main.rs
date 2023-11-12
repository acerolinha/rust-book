fn main() {
    // type alias
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    println!();

    // using type alias to reduce repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(_f: Thunk) {
        // --snip--
    }

    // fn returns_long_type() -> Thunk {
    // --snip--
    // }

    // the never type
    fn _bar() -> ! {
        panic!("This function never returns!");
    }
}
