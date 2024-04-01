// fn main() {
//     let o: Option<i32> = Some(7);

//     if let Some(i) = o {
//         println!("This is a really long string and '{:?}", i);
//         println!("success");
//     }
// }

// enum Foo {
//     Bar(u8),
// }

// fn main() {
//     let a: Foo = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value {}", i);

//         println!("success");
//     }
// }

// fn main() {
//     let age: Option<i32> = Some(30);
//     if let Some(age) = age {
//         assert_eq!(age, 30);
//     }

//     match age {
//         Some(age) => println!("age is a new variable. Its value is {}", age),
//         _ => (),
//     }
// }

// fn main() {}
// fn match_number(n: i32) {
//     match n {
//         1 => println!("One"),
//         2..=5 => println!("match 2 -> 5"),
//         6..=10 => {
//             println!("match 6 -> 10")
//         }
//         _ => {
//             println!("match  -infinate -> 0 or 11 -> +infinite")
//         }
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p: Point = Point { x: 3, y: 30 };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         Point {
//             x: 0..=5,
//             y: y @ (10 | 20 | 30),
//         } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: {} {}", x, y),
//     }
// }

// fn main() {
//     let num: Option<i32> = Some(4);
//     let split: i32 = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
//     println!("success");
// }

fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
    println!("success");
}
