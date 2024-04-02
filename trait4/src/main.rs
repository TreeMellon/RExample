// fn main() {
//     assert_eq!(sum(1, 2), 3);
//     println!("{:?}", sum(4.2, 10.2));
// }

// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is x = {:?}", self.y);
//         }
//     }
// }

// #[derive(Debug, PartialOrd, PartialEq)]
// struct Unit(i32);

// fn main() {
//     // let pair: Pair<Unit> = Pair {
//     //     x: Unit(1),
//     //     y: Unit(3),
//     // };

//     let pair: Pair<Unit> = Pair::new(Unit(1), Unit(2));

//     pair.cmp_display();
// }

trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn main() {
    let duck: Duck = Duck;
    duck.swim();

    let bird: Box<dyn Bird> = hatch_a_bird(2);

    assert_eq!(bird.quack(), "duck duck");

    let bird: Box<dyn Bird> = hatch_a_bird(1);

    assert_eq!(bird.quack(), "swan swan");

    println!("success");
}

fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}
