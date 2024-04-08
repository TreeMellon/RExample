// fn main() {
//     let v: &str = "hello";
//     need_static(v);

//     println!("success");
// }

// fn need_static(r: &'static str) {
//     assert_eq!(r, "hello");
// }

// static NUM: i32 = 18;

// fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
//     &NUM
// }

// fn main() {
//     {
//         let lifetime_num: i32 = 9;
//         let coerced_static: &i32 = coerce_static(&lifetime_num);

//         println!("coerced_static: {}", coerced_static);
//     }

//     println!("NUM {} stays accessible", NUM);
// }

use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is : {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    const i: i32 = 5;
    print_it(i);

    print_it(&i);

    print_it1(&i);

    print_it2(&i);
}
