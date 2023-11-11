fn main() {
    // associated types
    trait Iterator {
        type Item; // this is a placeholder type

        fn next(&mut self) -> Option<Self::Item>;
    }

    // operator overloading
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let one_meter_in_mm = Millimeters(1000);
    let two_meters = Meters(2);

    assert_eq!(one_meter_in_mm + two_meters, Millimeters(3000));

    // method disambiguation
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    println!();

    // fully qualified syntax
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby dog is called a {}", Animal::baby_name()); // error
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!();

    // supertraits
    trait OutlinePrint: std::fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point2 {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point2 {}

    impl std::fmt::Display for Point2 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let point = Point2 { x: 1, y: 2 };
    point.outline_print();
    println!();

    // newtype pattern
    struct Wrapper(Vec<String>);

    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
