// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("s is '{}'", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("r1 is '{} r2 is {}'", r1, r2);

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // }

    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
