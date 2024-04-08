// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// #[derive(Debug)]
// struct NamedBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x: i32 = 18;
//     let y: i32 = 15;

//     let single: Borrowed = Borrowed(&x);
//     let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y };
//     let reference: Either = Either::Ref(&x);
//     let number: Either = Either::Num(y);

//     println!("x is borrwed in {:?}", single);
//     println!("x and y are borrwed in {:?}", double);
//     println!("x is borrwed in {:?}", reference);
//     println!("y is not borrwed in {:?}", number);
// }

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

fn main() {
    let var_a: u32 = 35;
    let example: Example;

    let var_b: NoCopyType = NoCopyType {};

    example = Example {
        a: &var_a,
        b: &var_b,
    };
    println!("(success {:?}", example);
}
