use std::collections::HashMap;

fn main() {
    // calculating median and mode
    // vec1: median = 4, mode = 3
    let vec1 = vec![1, 2, 3, 3, 4, 5, 6, 7, 9];
    // vec2: median = 5, mode = any
    let vec2 = vec![1, 2, 3, 4, 5, 6, 7, 8];

    println!("{:#?}", median_and_mode(&vec1));
    println!("{:#?}", median_and_mode(&vec2));

    // convert to pig latin
    let s1 = String::from("first");
    let s2 = String::from("apple");
    let s3 = String::from("this is a phrase");

    println!("{}", convert_to_pig_latin(s1));
    println!("{}", convert_to_pig_latin(s2));
    println!("{}", convert_to_pig_latin(s3));

    // add employee names to a department
    let mut departments: HashMap<&str, Vec<&str>> = HashMap::new();
    let eng_dept = departments.entry("Engineering").or_insert(vec![]);
    eng_dept.push("Sally");
    eng_dept.push("Bryan");
    let sales_dept = departments.entry("Sales").or_insert(vec![]);
    sales_dept.push("Amir");
    sales_dept.push("Joanne");
    sales_dept.push("Edgar");

    println!("{departments:#?}");
}

fn median_and_mode(vec: &Vec<i32>) -> (i32, i32) {
    let median = vec[vec.len() / 2];
    let mut map = HashMap::new();

    for &i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = (0, -1);

    for (k, v) in map {
        if v > mode.1 {
            mode = (k, v);
        }
    }

    (median, mode.0)
}

fn convert_to_pig_latin(string: String) -> String {
    let mut result = String::new();
    for word in string.split_whitespace() {
        result.push_str(" ");
        let start = word.chars().next().unwrap();
        match start {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
                result.push_str(word);
                result += "-hay"
            }
            _ => {
                result.push_str(&word[1..]);
                result += format!("-{start}ay").as_str();
            }
        };
    }
    result[1..].to_string()
}
