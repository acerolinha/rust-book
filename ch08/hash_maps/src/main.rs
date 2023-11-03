// import hashmap
use std::collections::HashMap;

fn main() {
    // creating a hash map
    let mut scores = HashMap::new();

    // inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name}: {score}");
    println!();

    // iterating
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!();

    // update
    // by overwriting
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:#?}");

    // only inserting if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:#?}");

    // based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:#?}");
}
