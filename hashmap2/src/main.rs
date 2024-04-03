// use std::collections::HashMap;

// fn main() {
//     let mut player_stats = HashMap::new();

//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     player_stats
//         .entry("health")
//         .or_insert_with(random_stat_buff);

//     let health: &mut u8 = player_stats.entry("health").or_insert(50);

//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);
// }

// fn random_stat_buff() -> u8 {
//     42
// }

use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings: HashMap<Viking, i32> = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} {}", viking, health);
    }
}
