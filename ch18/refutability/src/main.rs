fn main() {
    // irrefutable pattern
    #[allow(unused_variables)]
    let x = 5;

    // refutable pattern;
    if let Some(x) = Some(5) {
        println!("{}", x);
    }

    // won't compile because let requires
    // an irrefutable pattern
    // let Some(x) = Some(5);

    // will give a warn because
    // the pattern will always match
    #[allow(irrefutable_let_patterns)]
    if let x = 5 {
        println!("{}", x);
    }
}
