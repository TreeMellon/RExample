use std::mem::size_of_val;

fn main() {
    let c1: char = 'a';

    println! {"{}", size_of_val(&c1)};

    let c2: char = '중';

    println! {"{}", size_of_val(&c2)};

    println!("Hello, world!");

    let f: bool = false;

    let t: bool = true && false;
    if t {
        println!("Success!");
    }

    assert_eq!(t, f);

    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
