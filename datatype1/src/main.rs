fn main() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10;

    println!("Hello, world!");

    let v: u16 = 38_u8 as u16;

    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");

    // let v1 = 251_u8 + 8_u8;
    // let v2 = i8::checked

    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
