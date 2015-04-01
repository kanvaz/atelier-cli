use std::process::Command;
use std::borrow::ToOwned;

pub fn init (name: &str) -> String {
    let output = Command::new("git")
            .arg("init")
            .arg(name)
            .output()
            .unwrap_or_else(|e| panic!("Failed to run git init with error: {}",e));
    let buf = String::from_utf8_lossy(&output.stdout);

    buf.trim_matches('\n').to_owned()
}
