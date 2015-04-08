use std::process::Command;
use std::borrow::ToOwned;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use file_set::FileData;

#[derive(Debug)]
pub struct Repository {
    pub path: String
}

impl Repository {

    pub fn commit_all (&self) -> String {
        let output = Command::new("git")
                .current_dir(Path::new(&self.path))
                //FIXME: don't rely on custom alias
                .arg("ca")
                .arg("-m \"foo\"")
                .output()
                .unwrap_or_else(|e| panic!("Failed to run git init with error: {}",e));
        let buf = String::from_utf8_lossy(&output.stdout);

        buf.trim_matches('\n').to_owned()
    }

    pub fn add_files (&self, files: Vec<FileData>) {
        files.iter().all(|file_data| {
            let path = Path::new(&self.path).join(&file_data.name);
            //FIXME: what's the best approach for error handling here?
            let mut file = File::create(&path).ok().unwrap();
            file.write(file_data.content.as_bytes()).ok();
            true
        });
    }

    pub fn add_files_and_commit (&self, files: Vec<FileData>) {
        self.add_files(files);
        self.commit_all();
    }
}
