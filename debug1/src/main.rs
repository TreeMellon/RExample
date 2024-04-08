// fn main() {
//     print!("hello world, ");
//     println!("I am");
//     print!("Sunface");
// }

// #[derive(Debug)]
// struct Structure(i32);

// fn main() {
//     println!("{} months in a year", 12);
//     println!("Now {:?}  will print!", Structure(3));
// }

use std::fmt;

struct Structure(i32);

struct Deep(Structure);

impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}

fn main() {
    println!("Now {:?} will print!", Deep(Structure(7)));
}
