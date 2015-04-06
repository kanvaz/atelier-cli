extern crate clap;
extern crate rustc_serialize;
use clap::{App, Arg};
//doesn't compile today, try tomorrow ;)
//use uuid::Uuid;
use git::{};
use repository::{FileData, FileSet};
use rustc_serialize::json;

mod git;
mod repository;

fn main() {

    // Pull version from Cargo.toml
    let version = format!("{}.{}.{}{}",
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR"),
                          env!("CARGO_PKG_VERSION_PATCH"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let matches = App::new("clog")
        .version(&version[..])
        .arg(Arg::new("init")
            .short("i")
            .long("init")
            .help("if present, data should create new repository"))
        .arg(Arg::new("data")
            .short("d")
            .long("data")
            .help("e.g. \"{ \"files\": [{ \"name\":\"style.css\", \"content\": \"button: { color: red; }\", \"app.js\": \"alert('foo');\" }] }")
            .takes_value(true))
        .get_matches();

    let repository = git::init("foo").unwrap();
    let test_data = "{ \"files\": [{ \"name\":\"style.css\", \"content\": \"button: { color: red; }\", \"app.js\": \"alert('foo');\" }] }";

    let file_set = matches.value_of("data").map(|data| {
        let file_set:FileSet = json::decode(data).ok().expect("invalid data provided");
        file_set
    });



    println!("{:?}", file_set);

    //let file_set:FileSet = json::decode(test_data).unwrap();

    match file_set {
        Some(file_set) => repository.add_files(file_set.files),
        _ => ()
    }

    //repository.add_files(file_set.files);
    //
    // repository.commit_all();
    println!("Look into the foo directory");
}
