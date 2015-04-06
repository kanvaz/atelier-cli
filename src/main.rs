extern crate clap;
extern crate rustc_serialize;
extern crate uuid;

use clap::{App, Arg};
use uuid::Uuid;
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

    //FIXME: can we move this?
    let repository_name = String::new() + "temp_rep" + &Uuid::new_v4().to_simple_string();

    let repository = if matches.is_present("init") {
        git::init(&repository_name).unwrap()
    } else {
        panic!("missing --init argument");
    };

    let test_data = "{ \"files\": [{ \"name\":\"style.css\", \"content\": \"button: { color: red; }\", \"app.js\": \"alert('foo');\" }] }";

    let file_set = matches.value_of("data").map(|data| {
        let file_set:FileSet = json::decode(data).ok().expect("invalid data provided");
        file_set
    });

    println!("{:?}", file_set);


    match file_set {
        Some(file_set) => { repository.add_files(file_set.files); repository.commit_all(); },
        _ => ()
    }

    println!("Look into the {:0} directory", repository_name);
}
