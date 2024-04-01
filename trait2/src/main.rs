// use std::ops;

// struct Foo;
// struct Bar;

// #[derive(Debug, PartialEq)]
// struct FooBar;

// #[derive(Debug, PartialEq)]
// struct BarFoo;

// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }

// fn main() {
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
//     // println!("Hello, world!");
// }

trait Summary {
    fn summarize(&self) -> String;
}
#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn main() {
    let post: Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is aweseom".to_string(),
    };

    let weibo: Weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

fn summary<T: Summary>(a: &T) {
    let output: String = a.summarize();

    println!("{}", output);
}
