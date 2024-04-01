struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let arrays: [Array<i32, 3>; 3] = [
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
        Array { data: [4, 1, 2] },
    ];

    let arrays: [Array<f64, 2>; 3] = [
        Array { data: [1.0, 2.0] },
        Array { data: [2.0, 3.0] },
        Array { data: [4.0, 1.0] },
    ];

    println!("Hello, world!");
}
