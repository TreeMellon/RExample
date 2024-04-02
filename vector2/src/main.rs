// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i))
//     }

//     for i in 0..5 {
//         match v.get(i) {
//             Some(e) => v[i] = e + 1,
//             None => v.push(i + 2),
//         }
//     }

//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("success");
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     let slice1: &[i32] = &v[..];

//     let slice2: &[i32] = &v[0..v.len()];

//     assert_eq!(slice1, slice2);

//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);

//     let slice3: &[i32] = &v[0..3];

//     assert_eq!(slice3, &[1, 2, 3]);

//     println!("success");
// }

// #[derive(Debug, PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let v: Vec<IpAddr> = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];

//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success");
// }

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ip6:{:?}", self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
