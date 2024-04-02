// fn main() {
//     let mut s: String = String::from("hello, world");

//     let slice1: &str = s.as_str();
//     assert_eq!(slice1, "hello, world");

//     let slice2: &str = &s[..5];
//     assert_eq!(slice2, "hello");

//     let slice3: &mut String = &mut s;
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");

//     println!("success");
// }
// fn main() {
//     let mut s: String = String::new();
//     s.push_str("hello");

//     let v: Vec<u8> = vec![104, 101, 108, 108, 111];

//     let s1: String = String::from_utf8(v).unwrap();

//     assert_eq!(s, s1);

//     println!("success");
// }

fn main() {
    let mut s: String = String::with_capacity(25);
    println!("{}", s.capacity());

    for _ in 0..3 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }
    println!("{}", s);
}
