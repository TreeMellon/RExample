// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let x: &str = "long";
//     let y: &str = "longer";
//     println!("{}", longest(x, y));
// }

// fn invalid_output<'a>(s: &'a str) -> &'a str {
//     s
// }

// fn main() {
//     let s: String = String::from("foo");

//     let x = invalid_output(&s);

//     println!("{}", x);
// }

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow() {
    let _x: i32 = 12;

    let y: &i32 = &_x;
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}
