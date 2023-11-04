// using generics with methods
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// with generic types that are different from
// its struct’s definition
struct PointD<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointD<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointD<X2, Y2>) -> PointD<X1, Y2> {
        PointD {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointD { x: 5, y: 10.4 };
    let p2 = PointD { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// using generics with enums
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// using generic with structs
// #[allow(dead_code)]
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// #[allow(dead_code)]
// #[derive(Debug)]
// struct PointD<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     println!("{integer:#?}");
//     println!("{float:#?}");
//     println!("");

//     // All types must be of the same type T
//     // let wont_work = Point { x: 5, y: 4.0 };
//     let both_integer = PointD { x: 5, y: 10 };
//     let both_float = PointD { x: 1.0, y: 4.0 };
//     let integer_and_float = PointD { x: 5, y: 4.0 };
//     println!("{both_integer:#?}");
//     println!("{both_float:#?}");
//     println!("{integer_and_float:#?}");
//     println!("");
// }

// combining functions into a single
// function with generics

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     // with duplicate definition
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");

//     // with generics definition
//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }
