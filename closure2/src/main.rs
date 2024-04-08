// fn main() {
//     let movable: Box<i32> = Box::new(3);

//     let consume = || {
//         println!("'movable': {:?}", movable);
//         take(&movable);
//     };

//     consume();
//     consume();
//     println!("Hello, world!");
// }

// fn take<T>(_v: &T) {}

// fn main() {
//     let movable = Box::new(3);

//     let consume = move || {
//         println!("'movable': {:?}", movable);
//     };

//     consume();
//     consume();
// }

// fn main() {
//     let example_closure = |x: String| -> String { x };

//     let s: String = example_closure(String::from("hello"));

//     let n = example_closure(5.to_string());
// }

// fn fn_once<F>(func: F)
// where
//     F: Fn(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x: Vec<i32> = vec![1, 2, 3];
//     fn_once(|z| z == x.len())
// }

fn main() {
    let mut s: String = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}
