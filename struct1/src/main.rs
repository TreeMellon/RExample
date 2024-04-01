// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let v: Point = Point(0, 127, 255);
//     check_color(v);
//     println!("Hello, world!");
// }

// fn check_color(p: Point) {
//     let Point(x, y, z) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(z, 255);
// }

// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let age = 18;
//     let mut p = Person {
//         name: String::from("sunface"),
//         age,
//     };

//     p.age = 30;

//     p.name = String::from("sunfei");

//     println!("Success");
// }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let u1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
    println!("Success");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}
