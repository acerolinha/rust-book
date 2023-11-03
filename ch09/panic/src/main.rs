fn main() {
    // calling panic
    // panic!("crash and burn");

    // causing panic from library
    // show backtrace with RUST_BACKTRACE=1
    // or RUST_BACKTRACE=full
    let v = vec![1, 2, 3];
    v[99];
}
