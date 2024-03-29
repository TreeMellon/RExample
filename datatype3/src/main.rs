use std::ops::{Range, RangeInclusive};

fn main() {
    println!("Hello, world!");

    let mut sum: i32 = 0;

    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");

    assert!(1u32 + 2u32 == 3u32);

    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1i8);
    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
