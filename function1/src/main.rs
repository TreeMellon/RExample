// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(self: &mut Self) {
//         println!("the current state is {}", self.color);
//     }

//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn new() -> Self {
//         Self {
//             color: String::from("red"),
//         }
//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light: TrafficLight = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
//     println!("success");
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// impl Rectangle {}

// fn main() {
//     println!("sucesss");
// }
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            Self::Yellow => "yellow",
            Self::Green => "green",
            Self::Red => "red",
        }
    }
}
fn main() {
    let c: TrafficLightColor = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");
}
