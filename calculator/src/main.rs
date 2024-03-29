use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.next().unwrap().chars().next().unwrap();
    let second = args.next().unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();

    let result = operate(operator, first_number, second_number);

    println!("{} {} {}", first, operator, second);
    println!("{}", result);
    println!("{}", output(first_number, operator, second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    // if operator == '+' {
    //     return first_number + second_number;
    // } else if operator == '-' {
    //     return first_number - second_number;
    // } else if operator == '/' {
    //     return first_number / second_number;
    // } else if operator == '*' {
    //     return first_number * second_number;
    // } else {
    //     return 0.0;
    // }
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    return format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    );
}

// fn nth(&mut self, n: usize) -> Option<String> {
//     self.inner.next()
//     self.innter.next()
// }
