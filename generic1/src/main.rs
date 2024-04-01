// struct A;
// struct S(A);
// struct SGen<T>(T);

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     reg_fn(S(A));
//     gen_spec_t(SGen(A));
//     gen_spec_i32(SGen(7));

//     generic::<char>(SGen('A'));

//     generic(SGen('Z'));

//     println!("success");
// }

// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("success");
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer: Point<i32> = Point { x: 5, y: 10 };
//     let float: Point<f64> = Point { x: 1.0, y: 4.0 };

//     println!("success");
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let p: Point<i32, String> = Point {
//         x: 5,
//         y: "hello".to_string(),
//     };

//     println!("success");
// }

// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// fn main() {
//     let x: Val<f64> = Val { val: 3.0 };
//     let y: Val<String> = Val {
//         val: "hello".to_string(),
//     };

//     println!("{},{}", x.value(), y.value());
// }
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1: Point<i32, i32> = Point { x: 5, y: 10 };
    let p2: Point<&str, char> = Point {
        x: "Hello",
        y: '한',
    };

    let p3: Point<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '한');

    println!("success");
}
