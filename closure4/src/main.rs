fn main() {
    let s = String::new();

    let update_string = move || println!("{}", s);

    exec(update_string);
    println!("Hello, world!");
}

fn exec<F: FnOnce()>(f: F) {
    f()
}
