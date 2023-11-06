// const list with box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    #[allow(unused_variables)]
    let list = Cons(
        1, 
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );
}

// const list without box
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// fn main() {
//     // this program will not compile because the
//     // recursive type List has infinite size
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// box intro
// fn main() {
//     // using a box
//     let b = Box::new(5); // alocated on the heap
//     println!("b = {b}");
// }
