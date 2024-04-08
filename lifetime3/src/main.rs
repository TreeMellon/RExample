// struct Owner(i32);

// impl Owner {
//     fn add_one<'a>(&'a mut self) {
//         self.0 += 1;
//     }
//     fn print<'a>(&'a self) {
//         println!("print: {}", self.0);
//     }
// }

// fn main() {
//     let mut owner = Owner(18);

//     owner.add_one();
//     owner.print();
//     println!("Hello, world!");
// }

// struct ImportantExcerpt {
//     part: &'static str,
// }

// impl ImportantExcerpt {
//     fn level<'a>(&'a self) -> i32 {
//         3
//     }
// }

// fn main() {}

fn input(x: &i32) {
    println!("'annotated_input': {}", x);
}

fn pass<'a>(x: &i32) -> &i32 {
    x
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }
    fn print(&self) {
        println!("'print': {}", self.0);
    }
}

struct Person {
    age: u8,
    name: &'static str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}
