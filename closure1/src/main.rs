// fn main() {
//     let x = 1;
//     let closure = |val| val + x;
//     assert_eq!(closure(2), 3);
//     //  println!("Hello, world!");
// }

// fn main() {
//     fn function(i: i32) -> i32 {
//         i + 1
//     }

//     let closure_annotated = |i: i32| -> i32 { i + 1 };
//     let closure_inferred = |i| i + 1;

//     let i = 1;

//     println!("function: {}", function(i));
//     println!("closure_annotaed: {}", closure_annotated(i));
//     println!("closure_inferred: {}", closure_inferred(i));
// }

// fn main() {
//     let color: String = String::from("green");

//     let print = || println!("'color': {}", color);

//     print();
//     print();

//     let _reborrow: &String = &color;

//     println!("{}", color);
// }

fn main() {
    let mut count: i32 = 0;

    let mut inc = move || {
        count += 1;
        println!("'count': {}", count);
    };

    inc();

    let _reborrow: &i32 = &count;

    inc();

    let _count_reborowed = &mut count;

    assert_eq!(count, 0);
}
