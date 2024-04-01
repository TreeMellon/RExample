// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale: u32 = 2;
//     let rect1: Rectangle = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);

//     println!("{:?}", rect1);
// }

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name: String = f.name.clone();

    println!("{}, {}, {:?}", _name, f.data, f);
}
