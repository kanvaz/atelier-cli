use git::{};
use repository::{};

mod git;
mod repository;

fn main() {
    let r = git::init("foo").unwrap();
    r.commit_all();
    println!("Hello, world!");
}
