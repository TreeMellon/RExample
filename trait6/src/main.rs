// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self {
//         42
//     }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self {
//         self.clone()
//     }
// }

// fn my_function<T: MyTrait>(x: T) -> T {
//     x.f()
// }

// fn main() {
//     my_function(13_u32);
//     my_function(String::from("abc"));

//     println!("Hello, world!");
// }

// trait MyTrait {
//     fn f(&self) -> Box<dyn MyTrait>;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Box<dyn MyTrait> {
//         Box::new(42)
//     }
// }

// impl MyTrait for String {
//     fn f(&self) -> Box<dyn MyTrait> {
//         Box::new(self.clone())
//     }
// }

// fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
//     x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("success");
// }

// fn main() {
//     let s: String = String::from("hello, 종철");
//     let slice1: &str = &s[..1];
//     assert_eq!(slice1, "h");

//     let slice2 = &s[7..10];
//     assert_eq!(slice2, "종");

//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '종')
//         }
//     }
// }
fn main() {
    let mut s: String = String::new();

    let v: Vec<u8> = vec![104, 101, 108, 108, 111];

    let s1: String = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("success");
}
