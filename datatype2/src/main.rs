fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("success");

    let x: f64 = 1_000.000_1;
    let y: f32 = 0.12;
    let z: f64 = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
