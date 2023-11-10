fn main() {
    // in match arms
    let x = Some(5);

    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // in if let
    if let Some(i) = x {
        Some(i + 1);
    } else if let None = x {
        ()
    }

    // in while let
    while let Some(i) = x {
        Some(i + 1);
    }

    // in for
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // in let
    #[allow(unused_variables)]
    let (x, y, z) = (1, 2, 3);

    // in function parameters
    fn _foo(_x: i32) { // _x is a pattern
                       // --snip--
    }

    fn _print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
}
