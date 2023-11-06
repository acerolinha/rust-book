struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    #[allow(unused_variables)]
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // this is not valid
    // c.drop();

    // force dropping with std::mem::drop
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}
