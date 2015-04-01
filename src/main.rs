use git::{};

mod git;

fn main() {
    let s = git::init("foo");
    println!("Hello, world!");
}
