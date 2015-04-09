extern crate rustc_serialize;
use rustc_serialize::json;

//TODO: Why can't we use Decodable/Encodable on &str
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct FileData {
    pub name: String,
    pub content: String
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct FileSet {
    pub files: Vec<FileData>
}

impl FileSet {
    pub fn from_json (json: &str) -> FileSet {
        let file_set:FileSet = json::decode(json).ok().expect("invalid data provided");
        file_set
    }

    pub fn to_json (&self) -> String {
        format!("{}", json::as_json(&self))
    }

    pub fn to_pretty_json (&self) -> String {
        format!("{}", json::as_pretty_json(&self))
    }
}
