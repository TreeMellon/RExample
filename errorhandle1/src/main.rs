// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("success");

//         panic!()
//     }
// }

// fn main() {
//     drink("lemonade");

//     println!("Exercise failed!");
// }

fn main() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v: Vec<i32> = vec![1, 2, 3];
    let ele: i32 = v[2];

    let ele: &i32 = v.get(1).unwrap();

    let v = production_rate_per_hour(2);

    divide(15, 1);

    println!("success");
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u16) -> f64 {
    let cph: u16 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        1..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minutes(speed: u16) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
