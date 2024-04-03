// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(value: i32) -> Self {
//         Self { value }
//     }
// }

// fn main() {
//     let num: Number = Number::from(30);

//     assert_eq!(num.value, 30);

//     let num: Number = 30.into();
//     assert_eq!(num.value, 30);

//     println!("success");
// }

use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(e: num::ParseIntError) -> Self {
        CliError::ParseError(e)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let contents: String = fs::read_to_string(&file_name)?;

    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("sucess");
}
