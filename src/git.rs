use std::process::Command;
use std::io::Error;
use repository::Repository;

pub fn init (name: &str) -> Result<Repository, Error> {
    Command::new("git")
            .arg("init")
            .arg(name)
            .output()
            .map(|_| Repository {
                path: name
            })
}
