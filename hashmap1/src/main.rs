// use std::collections::HashMap;

// fn main() {
//     let mut scores: HashMap<&str, i32> = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);

//     let score: Option<&i32> = scores.get("Sunface");
//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
//         let score = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score);
//     }
// }

// use std::collections::HashMap;

// fn main() {
//     let teams: [(&str, i32); 3] = [("A Team", 100), ("American Team", 10), ("France Team", 50)];

//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }

//     let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();

//     assert_eq!(teams_map1, teams_map2);
// }
