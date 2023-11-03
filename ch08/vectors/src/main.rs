fn main() {
    // Creating a vector
    #[allow(unused_variables)]
    let v: Vec<i32> = Vec::new();

    // Creating a vector with initial values
    #[allow(unused_variables)]
    let v = vec![1, 2, 3];

    // Updating a vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading elements of a vector
    let v = vec![1, 2, 3, 4, 5];

    // by indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // by get
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Only the next line would cause a panic
    // let does_not_exist = &v[100];
    // The next line returns None and don't panic
    #[allow(unused_variables)]
    let does_not_exist = v.get(100);

    #[allow(unused_mut)]
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // Can't push to v because it may would reallocate
    // and the reference would be invalid
    // v.push(6);

    println!("The first element is: {first}");

    // iterating
    // through immutable reference
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    println!();

    // through mutable reference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // using enums to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:#?}");

    // dropping a vector

    {
        #[allow(unused_variables)]
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
      // so are its elements
}
