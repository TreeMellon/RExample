// fn main() {
//     let x: i32 = 5;
//     let y: &i32 = &x;

//     assert_eq!(5, *y);

//     println!("Success!");
// }

// fn main() {
//     let mut s: String = String::from("hello, ");

//     borrow_object(&s);

//     println!("Success!");
// }

// fn borrow_object(s: &String) {}

// fn main() {
//     let mut s: String = String::from("hello, ");

//     let p: &mut String = &mut s;

//     p.push_str("world");

//     println!("Success! {}", p);
// }

// fn main() {
//     let c: char = '중';

//     let r1: &char = &c;

//     let ref r2 = c;

//     assert_eq!(*r1, *r2);

//     assert_eq!(get_addr(r1), get_addr(r2));

//     println!("Success");
// }

// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// fn main() {
//     let mut s: String = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);

//     println!("s: {} ", s);

//     println!("Success!");
// }

// fn main() {
//     // let s: Box<str> = "hello, world".into();
//     // greetings(&s)
//     let s: &str = "hello, world";
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}", s)
// }

// fn main() {
//     let s: &str = "hello, world";
//     //  greetings(s.to_string());
//     greetings(s.to_owned());
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }

fn main() {
    let s1: String = String::from("hi,종철");
    let h: &str = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[3..9];
    assert_eq!(h1, "종철");

    println!("Success!");
}
