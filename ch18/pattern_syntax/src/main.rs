fn main() {
    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!();

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {x:?}, y = {y}");
    println!();

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!();

    // matching ranges
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    println!();

    // destructuring
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // destructuring with shorthand
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // destructuring and matching literals
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    println!();

    // destructuring enums
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
    println!();

    // destructuring nested structs and enums
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
    println!();

    // destructuring structs and tuples
    #[allow(unused_variables)]
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring values in a pattern
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    foo(3, 4);
    println!();

    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {:?}", setting_value);
    println!();

    // ignoring multiple parts of a tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    println!();

    // ignoring an unused variable by starting its name with _
    let _x = 5; // this does not give a warning
    #[allow(unused_variables)]
    let y = 10; // this does give a warning

    // ignoring remaining parts of a value with ..
    #[allow(dead_code)]
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }
    println!();

    // matching only the first and last values in a tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}"),
    }
    println!();

    // match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    println!();

    // combining multiple patterns with a match guard
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    println!();

    // @ bindings
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
    println!();
}
