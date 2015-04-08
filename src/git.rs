use std::process::Command;
use std::io::Error;
use repository::Repository;
use std::borrow::ToOwned;

pub fn init (name: &str) -> Result<Repository, Error> {
    Command::new("git")
            .arg("init")
            .arg(name)
            .output()
            .map(|_| Repository {
                path: name.to_owned()
            })
}
