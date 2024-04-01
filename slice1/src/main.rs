// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];

//     let s2: &str = "hello, world";

//     println!("Success!");
// }

// fn main() {
//     let arr: [char; 3] = ['선', '종', '철'];
//     let slice: &[char] = &arr[0..2];

//     assert!(std::mem::size_of_val(&slice) == 16);

//     println!("Success!");
// }

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     let slice: &[i32] = &arr[1..4];

//     assert_eq!(slice, &[2, 3, 4]);

//     println!("Success!");
// }

// fn main() {
//     let s: String = String::from("hello");
//     let slice1: &str = &s[0..2];

//     let slice2 = &s[0..2];

//     assert_eq!(slice1, slice2);

//     println!("Success!");
// }

fn main() {
    let mut s: String = String::from("hello world");

    let word: &str = first_word(&s);

    // s.clear();

    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    &s[..1]
}
