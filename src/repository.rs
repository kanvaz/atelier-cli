use std::process::Command;
use std::borrow::ToOwned;
use std::path::Path;

pub struct Repository<'a> {
    pub path: &'a str
}

impl<'a> Repository<'a> {

    pub fn commit_all (&self) -> String {
        let output = Command::new("git")
                .current_dir(Path::new(self.path))
                //FIXME: don't rely on custom alias
                .arg("ca")
                .arg("-m \"foo\"")
                .output()
                .unwrap_or_else(|e| panic!("Failed to run git init with error: {}",e));
        let buf = String::from_utf8_lossy(&output.stdout);

        buf.trim_matches('\n').to_owned()
    }
}
