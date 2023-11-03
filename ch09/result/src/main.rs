use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // this call returns a Result<T, E>
    let greeting_file_result = File::open("hello.txt");

    // handling Result with match
    #[allow(unused_variables)]
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {error:?}");
        }
    };

    // matching on different errors
    let greeting_file_result = File::open("hello.txt");
    #[allow(unused_variables)]
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // with closures
    #[allow(unused_variables)]
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|erorr| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening te file: {error:?}");
        }
    });

    // panic on error shortcuts
    // unwrap
    // let greeting_file = File::open("hello2.txt").unwrap();

    // expect <- preferred in production-quality code
    // let greeting_file =
    // File::open("hello2.txt").expect("hello2.txt should be included in this project");

    println!(
        "{}",
        read_username_from_file().expect("Failed to read username")
    );

    println!(
        "{}",
        read_username_from_file_short().expect("Failed to read username")
    );

    println!(
        "{}",
        read_username_from_file_shorter().expect("Failed to read username")
    );

    println!(
        "{}",
        read_username_from_file_short_af().expect("Failed to read username")
    );

    println!(
        "{}",
        read_username_from_file_tiny().expect("Failed to read username")
    );

    let username = read_username_from_file_tiny().expect("Failed to read username");
    println!(
        "{}",
        last_char_of_first_line(username.as_str()).expect("Last char not found")
    );
}

// error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// or shorter
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")
        .expect("hello.txt should be included in this project")
        .read_to_string(&mut username)
        .expect("Failed to read hello.txt");
    Ok(username)
}

// or even shorter
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// or even more shorter
fn read_username_from_file_short_af() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// or even more shorter than before
fn read_username_from_file_tiny() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? also works with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
