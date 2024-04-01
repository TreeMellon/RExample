use craft::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
}

fn main() {
    println!("Hello, world!");
}
