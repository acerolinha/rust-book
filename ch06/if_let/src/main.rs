// handling values that match one pattern
// while ignoring the rest
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // this code is bolerplate
    }

    // using if let syntax
    // this is equivalent to the above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }

    // we can access the _ pattern with else
    // this is equivalent to the above
    let config_max: Option<u8> = None;

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    } else {
        println!("The maximum is not configured")
    }
}
