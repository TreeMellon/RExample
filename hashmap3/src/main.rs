// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
//     map.insert(1, 2);
//     map.insert(3, 4);

//     assert!(map.capacity() >= 100);

//     map.shrink_to(50);
//     assert!(map.capacity() >= 50);

//     map.shrink_to_fit();
//     assert!(map.capacity() >= 2);
//     println!("success");
// }

use std::collections::HashMap;
fn main() {
    let v1: i32 = 10;
    let mut m1: HashMap<i32, i32> = HashMap::new();

    m1.insert(v1, v1);
    println!("v1 is still usable after insert {}", v1);

    let v2: String = "hello".to_string();
    let mut m2: HashMap<&str, i32> = HashMap::new();
    m2.insert(&v2, v1);

    assert_eq!(&v2, "hello");

    println!("success");
}
