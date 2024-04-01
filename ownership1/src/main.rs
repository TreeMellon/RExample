// fn main() {
//     println!("Hello, world!");

//     let s = String::from("hello");

//     takes_ownership(s);

//     let x = 5;

//     makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn main() {
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn main() {
//     let x: String = String::from("hello, world");
//     let y: String = x.clone();
//     println!("{},{}", x, y);
// }

// fn main() {
//     let s1: String = String::from("hello, world");
//     let s2: String = take_ownership(s1);

//     println!("{}", s2);
// }

// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    let _s = s.as_bytes();
    s
}
