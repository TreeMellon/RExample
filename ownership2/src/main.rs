// fn main() {
//     println!("Hello, world!");
//     let s: String = String::from("hello, world");

//     print_str(s.clone());
//     println!("{}", s);
// }

// fn print_str(s: String) {
//     println!("{}", s)
// }

// fn main() {
//     let x: () = (1,2,(), "hello".to)
//     let y = x.clone();
//     println!("{},{}",x,y)
// }

// fn main() {
//     let x: (i32, i32, (), &str) = (1, 2, (), "hello");
//     let y: (i32, i32, (), &str) = x;
//     println!("{:?}, {:?}", x, y);
// }

// fn main() {
//     let s: String = String::from("hello, ");

//     let mut s1 = s;
//     s1.push_str("world");
//     println!("Success!");
// }

// fn main() {
//     let x: Box<i32> = Box::new(5);

//     let mut y: Box<i32> = Box::new(1);

//     *y = 4;

//     assert_eq!(*x, 5);

//     println!("Success!");
// }

// fn main() {
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     };

//     let person: Person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     let Person { name, ref age } = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);
// }

fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", t.1, s1, s2);
}
