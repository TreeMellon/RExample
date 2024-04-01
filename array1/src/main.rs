// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];

//     assert!(arr.len() == 5);
//     println!("Hello, world!");
// }

// fn main() {
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];

//     assert!(std::mem::size_of_val(&arr) == 12);
//     println!("Hello, world!");
// }

// fn main() {
//     //  let list: [i32; 100] = __;
//     let list: [i32; 100] = [1; 100];

//     assert!(list[0] == 1);
//     assert!(list.len() == 100);

//     println!("Success!");
// }

// fn main() {
//     let _arr: [i32; 3] = [1, 2, 3];

//     println!("Success!");
// }

fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];

    let name0 = names.get(0).unwrap();

    let _name1 = &names[1];

    println!("Success!");
}
