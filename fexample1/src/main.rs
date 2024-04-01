fn main() {
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    let v: i32 = {
        let mut h: i32 = 1;
        h += 2;
        h
    };

    assert_eq!(v, 3);
    println!("Success");

    let s = sum(1, 2);
    assert_eq!(s, 3);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
