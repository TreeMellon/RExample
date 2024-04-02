// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];

//     let v: Vec<u8> = Vec::from(arr);
//     is_vec(v);

//     let v: Vec<u8> = vec![1, 2, 3];
//     is_vec(v);

//     let v = vec![1, 2, 3];
//     is_vec(v.clone());

//     let mut v1 = Vec::new();
//     is_vec(v1.clone());

//     for i in &v {
//         v1.push(*i);
//     }

//     assert_eq!(v, v1);

//     println!("success");
// }

// fn is_vec(v: Vec<u8>) {}

// fn main() {
//     let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);

//     let mut v2: Vec<i32> = Vec::new();
//     v2.extend(&v1);

//     assert_eq!(v1, v2);
//     println!("success");
// }

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let v1: Vec<i32> = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);

    let s: String = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s: String = "hello".to_string();
    let v2: Vec<u8> = s.into_bytes();
    assert_eq!(v1, v2);

    let s: &str = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);

    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("success");
}
