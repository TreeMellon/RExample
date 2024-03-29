#![deny(clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut values: HashMap<&str, &str> = HashMap::new();
    values.insert("foo", "bar");
    println!("{:?}", values);

    if values.contains_key("name") {
        println!("name exists");
    } else {
        println!("No name");
    }
}
