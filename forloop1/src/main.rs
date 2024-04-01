// fn main() {
//     let names: [String; 2] = [String::from("liming"), String::from("joy")];

//     for name in names {
//         println!("{}", &name);
//     }

//     println!("{:?}", names);

//     let numbers: [i32; 3] = [1, 2, 3];

//     for n in numbers {
//         println!("{}", n);
//     }

//     println!("{:?}", numbers);
// }

// fn main() {
//     let a: [i32; 4] = [4, 3, 2, 1];

//     for (i, v) in a.iter().enumerate() {
//         println!("The {}th element is {}", i + 1, v);
//     }
// }

// fn main() {
//     let mut n: i32 = 1;

//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("fizz");
//         } else {
//             println!("{}", n);
//         }

//         n += 1;
//     }
// }

// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n != 66 {
//             n += 1;
//             continue;
//         }

//         break;
//     }

//     assert_eq!(n, 66);
//     println!("success");
// }

// fn main() {
//     let mut counter: i32 = 0;

//     let result: i32 = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     assert_eq!(result, 20);
//     println!("success");
// }

fn main() {
    let mut count = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }

    assert!(count == 30);
    println!("success!");
}
