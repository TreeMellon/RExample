// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }

// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck .. oh sorry, the swam is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan ");
//     }
// }

// fn main() {
//     let birds: [&dyn Bird; 2] = [&Duck, &Swan];

//     for bird in birds {
//         bird.quack();
//     }
//     // println!("Hello, world!");
// }

// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", self)
//     }
// }

// fn main() {
//     let x: f64 = 1.1f64;
//     let y: u8 = 8u8;

//     draw_with_box(Box::new(x));

//     draw_with_ref(&y);

//     println!("success");
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw_with_ref(x: &dyn Draw) {
//     x.draw();
// }

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn static_dispatch<T: Foo>(a: T) {
    a.method();
}

fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}

fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("success");
}
