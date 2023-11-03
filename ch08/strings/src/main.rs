fn main() {
    // creating a string
    #[allow(unused_mut, unused_variables)]
    let mut s = String::new();

    // with initial data
    let data = "initial contents";

    #[allow(unused_variables)]
    let s = data.to_string();

    // directly on string literal
    #[allow(unused_variables)]
    let s = "initial contents".to_string();

    // or
    #[allow(unused_variables)]
    let s = String::from("initial contents");

    // can handle any UTF-8
    #[allow(unused_variables)]
    let hello = String::from("السلام عليكم");
    #[allow(unused_variables)]
    let hello = String::from("Dobrý den");
    #[allow(unused_variables)]
    let hello = String::from("Hello");
    #[allow(unused_variables)]
    let hello = String::from("שָׁלוֹם");
    #[allow(unused_variables)]
    let hello = String::from("नमस्ते");
    #[allow(unused_variables)]
    let hello = String::from("こんにちは");
    #[allow(unused_variables)]
    let hello = String::from("안녕하세요");
    #[allow(unused_variables)]
    let hello = String::from("你好");
    #[allow(unused_variables)]
    let hello = String::from("Olá");
    #[allow(unused_variables)]
    let hello = String::from("Здравствуйте");
    #[allow(unused_variables)]
    let hello = String::from("Hola");

    // updating a string
    // by pushing a string slice
    let mut s = String::from("foo");
    s.push_str("bar"); // takes a string slice

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // because push_str takes a string slice we can
    // still use s2 after it's call
    println!("s2 is {}", s2);

    // by pushing a single character
    let mut s = String::from("lo");
    s.push('l');

    // with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s2 is coerced to a string slice
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    // multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");

    // with format macro
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");

    // indexing
    #[allow(unused_variables)]
    let s1 = String::from("hello");
    // the next line would throw an error
    // let h = s1[0]; // error: cannot be indexed by integer

    // consider Hindi word नमस्ते
    // it's bytes are
    // [224, 164, 168, 224, 164, 174, 224, 164, 184,
    //  224, 165, 141, 224, 164, 164, 224, 165, 135] = length 18
    // it's chars are
    // ['न', 'म', 'स', '्', 'त', 'े'] = length 6
    // it's grapheme clusters are
    // ['न', 'म', 'स्', 'ते'] = length 4

    // string slicing
    let hello = "Здравствуйте";
    // s contains the first 4 bytes of hello
    let s = &hello[0..4];
    println!("{s}");
    // s would countain only part of the first character bytes
    // let s = &hello[0..1]; // error: byte index 1 is not a char boundary
    // the next line would throw an error
    // println!("{s}");

    // iterating over strings
    // by chars
    for c in "Зд".chars() {
        println!("{c}");
    }

    // by bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // by grapheme clusters
    // only available via exernal crates
}
