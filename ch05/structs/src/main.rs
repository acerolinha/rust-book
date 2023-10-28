// Defining a user
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Defining a tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
    // Instantiating a struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Instantiating a tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Instantiating a unit-like struct
    // let subject = AlwaysEqual; // equivalent to ()

    // Accessing fields

    // Normal structs
    println!("User");
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!();

    // Tuple structs
    println!("Tuples");
    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);
    println!();

    // Changing value of fields
    let mut user1 = user1;

    user1.email = String::from("anyone@example.com");

    println!("Changing value");
    println!("{}", user1.email);
    println!();

    let user1 = build_user(format!("me@myself.com"), format!("myself"));

    println!("User 1");
    println!("{}", user1.email);
    println!("{}", user1.username);
    println!();

    // struct update syntax
    let user2 = User {
        email: format!("another@email.com"),
        ..user1
    };

    println!("User 2");
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!();

    println!("User 1");
    println!("{}", user1.email);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    // The next line would throw an error because username (String)
    // was borrowed by user2 in the struct update syntax
    // println!("{}", user1.username);
}

// Using field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
