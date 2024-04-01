// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }

// struct Teacher {}

// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("hi, I'm your new teacher")
//     }

//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }

// fn main() {
//     let s: Student = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Hello, world!");
// }

// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);

// fn main() {
//     let _one_second: Seconds = Seconds(1);

//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = (_one_second == _one_second);
//     let _this_is_true = (_one_second > _one_second);

//     let foot: Inches = Inches(12);
//     println!("One foot equals {:?} ", foot);

//     let meter: Centimeters = Centimeters(100.0);

//     let cmp: &str = if foot.to_centimeters() < meter {
//         "smaller"
//     } else {
//         "bigger"
//     };

//     println!("One foot is {} than one meter", cmp);
// }

use std::ops;

fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("success");
}
