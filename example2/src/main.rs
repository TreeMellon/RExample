fn main() {
    println!("Hello, world!");

    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success");
            panic!("We have no value for 'false', but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line !");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {}
        _ => {}
    }

    never_return_fn()
}

fn never_return_fn() -> ! {
    //  panic!()
    unimplemented!()
}
