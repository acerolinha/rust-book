use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!();

    // type annotation in closure
    #[allow(unused_variables)]
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // closure syntax
    // fn add_one_v1    (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1} ;
    // let add_one_v4 = |x|               x + 1  ;

    // compiler will infer one concrete type for closure
    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // // The next line would throw an error because
    // // the closure is expecting a String
    // let n = example_closure(5);

    // references and ownership
    // immutable reference
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    println!();

    // mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // The next line would throw an error because
    // list is already borrowed as mutable
    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
    println!();

    // moving ownership
    let list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);
    thread::spawn(move || {
        println!("From thread: {:?}", list);
    })
    .join()
    .unwrap();
}
