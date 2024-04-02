// fn main() {
//     assert_eq!(sum(1, 2), 3);
//     println!("{:?}", sum(4.2, 10.2));
// }

// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

s

impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x, self.y);
        }
    }
}