use git::{};
use repository::{FileData};

mod git;
mod repository;

fn main() {
    let repository = git::init("foo").unwrap();

    repository.add_files(vec!(
        FileData { name: "foo.txt", content: "foobar" },
        FileData { name: "bar.txt", content: "bar" },
    ));

    repository.commit_all();
    println!("Look into the foo directory");
}
