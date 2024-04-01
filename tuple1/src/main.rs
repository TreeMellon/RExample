// fn main() {
//     let _t0: (u8, i16) = (0, -1);

//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));

//     let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from("hello"));
//     println!("Hello, world!");
// }

// fn main() {
//     let tup: (i32, f64, &str) = (1, 6.4, "hello");

//     let (x, z, y) = tup;

//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);

//     println!("success");
// }

// fn main() {
//     let (x, y) = sum_multiply((3, 2));

//     assert_eq!(x, 5);
//     assert_eq!(y, 6);

//     println!("Success");
// }

// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }
