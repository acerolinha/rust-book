// using method syntax
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // a method with same name as a field is allowed
    fn width(&self) -> bool {
        self.width > 0
    }

    // The following would also work
    // fn can_fit(&self, other: &Self) -> bool {
    fn can_fit(&self, other: &Rectangle) -> bool {
        other.area() < self.area()
    }
}

// structs can have multiple impl blocks
impl Rectangle {
    fn can_fit_in_place(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    // associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // call of associated function that is a method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a non zero width: it is {}.", rect1.width);
    }

    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 fit rect2? {}", rect1.can_fit(&rect2));
    println!("Can rect1 fit rect3? {}", rect1.can_fit(&rect3));

    println!(
        "Can rect1 fit rect2 without rotating? {}",
        rect1.can_fit_in_place(&rect2)
    );
    println!(
        "Can rect1 fit rect3 without rotating? {}",
        rect1.can_fit_in_place(&rect3)
    );

    // call of associated function that is not a method
    let square = Rectangle::square(64);
    println!("Area of the square: {} square pixels.", square.area());
}

// using structs
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     // The next line would not work because Rectangle does
//     // not implement Display trait.
//     // println!("{rect1}");

//     println!("{rect1:?}");

//     println!("{rect1:#?}");

//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// using tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// using single variables
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
