// defining a macro
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// defining a custom derive macro
pub trait HelloMacro {
    fn hello_macro();
}

use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// attribute-like macros
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
//     // snip
// }

// #[route(GET, "/")]
// fn index() {
//     // snip
// }

// function-like macros
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     // snip
// }

// let sql = sql!(SELECT * FROM posts WHERE id=1);

fn main() {
    Pancakes::hello_macro();
}
