use std::process::Command;
use std::borrow::ToOwned;
use std::path::Path;
use std::fs::{self, File};
use std::io::{Write};
use file_set::FileData;
use readext::ReadExt;

#[derive(Debug)]
pub struct Repository {
    pub path: String
}


static BLACKLIST: [&'static str; 2] = [".git", ".DS_Store"];

impl Repository {


    pub fn stage_all (&self) -> String {
        let output = Command::new("git")
                .current_dir(Path::new(&self.path))
                .arg("add")
                .arg("-A")
                .output()
                .unwrap_or_else(|e| panic!("Failed to run git add -A with error: {}",e));
        let buf = String::from_utf8_lossy(&output.stdout);

        buf.trim_matches('\n').to_owned()
    }

    pub fn commit_all (&self, message: &str) -> String {
        let output = Command::new("git")
                .current_dir(Path::new(&self.path))
                .arg("commit")
                .arg(format!("-m {0}", message))
                .output()
                .unwrap_or_else(|e| panic!("Failed to run git commit -m with error: {}",e));
        let buf = String::from_utf8_lossy(&output.stdout);

        buf.trim_matches('\n').to_owned()
    }

    pub fn add_files (&self, files: Vec<FileData>) {
        files.iter().all(|file_data| {
            let path = Path::new(&self.path).join(&file_data.name);
            File::create(&path)
                .and_then(|mut file| {
                    file.write(file_data.content.as_bytes())
                }).ok();

            true
        });
    }

    pub fn add_files_and_commit (&self, files: Vec<FileData>, message: &str) {
        self.add_files(files);
        self.stage_all();
        self.commit_all(message);
    }

    pub fn read_all_files(&self) -> Vec<FileData> {
        fs::read_dir(Path::new(&self.path))
            .map(|iter| {
                iter
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .map(|path| {
                    FileData {
                        name: path.as_path().file_name().map_or("", |f| f.to_str().unwrap_or("")).to_string(),
                        content: File::open(path.as_path()).and_then(|mut f| f.read_into_string()).unwrap_or(String::new())
                    }
                })
                .filter(|file_data| file_data.name.len() > 0 && !BLACKLIST.iter().any(|b| *b == file_data.name))
                .collect()
            })
            .unwrap_or(vec!())
    }
}
