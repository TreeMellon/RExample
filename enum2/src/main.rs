// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0),
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// }

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let five: Option<i32> = Option::Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    if let Some(n) = six {
        println! {"{}",n};

        println!("Success!");
    } else {
        panic!("NEVER LET THIS RUN!");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
