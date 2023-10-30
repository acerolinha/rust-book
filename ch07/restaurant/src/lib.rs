mod front_of_house {
    // both mod and fn should be public to be accessible
    pub mod hosting {
        pub fn add_to_waitlist() {}

        #[allow(dead_code)]
        fn seat_at_table() {}
    }

    mod serving {
        #[allow(dead_code)]
        fn take_order() {}

        #[allow(dead_code)]
        fn serve_order() {}

        #[allow(dead_code)]
        fn take_payment() {}
    }
}

// using super to refer to parent module
#[allow(dead_code)]
fn deliver_order() {}

mod back_of_house {
    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    #[allow(dead_code)]
    fn cook_order() {}

    // pub with structs
    pub struct Breakfast {
        pub toast: String,
        #[allow(dead_code)]
        season_fruit: String, // this field is private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }

    // making an enum public makes all its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// using use keyword to create a shortcut to a path
use crate::front_of_house::hosting;

// creating a shortcut directly to the function
// makes it unclear to know where is the source of it
use crate::front_of_house::hosting::add_to_waitlist;

// on the other hand, to bring structs, enums and other
// full path is preferred
use std::collections::HashMap; // this is an external package

// bringing two paths with the same item name
use std::fmt;
use std::io;

// parent is specified for disambiguation
#[allow(dead_code)]
fn fun1(_: fmt::Result) {}
#[allow(dead_code)]
fn fun2(_: io::Result<()>) {}

// providing new names with as keyword

use std::io::Result as IoResult;

#[allow(dead_code)]
fn fun3(_: IoResult<()>) {}

// re-exporting names with pub use
pub use crate::back_of_house::Appetizer;

// nested paths
#[allow(unused_imports)]
use std::{cmp::Ordering, i16};

// using self
#[allow(unused_imports)]
use std::array::{self, IntoIter};

// glob operator
#[allow(unused_imports)]
use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // by use keyword shortcut
    hosting::add_to_waitlist();

    // by unclear use keyword shortcut
    add_to_waitlist();

    // full path shortcut for struct
    let mut map = HashMap::new();
    map.insert(1, 2);

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // The next line works because toast field is public
    meal.toast = String::from("Wheat");

    // The next line won't compile because seasonal_fruit field is private
    // meal.seasonal_fruit = String::from("blueberries");

    #[allow(unused_variables)]
    let order1 = back_of_house::Appetizer::Soup;
    #[allow(unused_variables)]
    let order2 = back_of_house::Appetizer::Salad;
}
