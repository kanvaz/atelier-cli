extern crate clap;
extern crate rustc_serialize;
extern crate uuid;

use clap::{App, Arg};
use uuid::Uuid;
use git::{};
use file_set:: { FileSet, FileData };
use repository::{ Repository };
use rustc_serialize::json;

mod git;
mod repository;
mod file_set;

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
            .mutually_excludes("update-set")
            .help("if present, data should create new repository"))
        .arg(Arg::new("update-set")
            .short("u")
            .long("update-set")
            .mutually_excludes("init")
            .takes_value(true)
            .help("updates existing set eg --update-set=\"4711\""))
        .arg(Arg::new("data")
            .short("d")
            .long("data")
            .help("e.g. \"{ \"files\": [{ \"name\":\"style.css\", \"content\": \"button: { color: red; }\", \"app.js\": \"alert('foo');\" }] }")
            .takes_value(true))
        .get_matches();

    let repository = if matches.is_present("init") {
        get_repository_handle(None)
    } else if matches.is_present("update-set") {
        get_repository_handle(matches.value_of("update-set"))
    } else {
        panic!("Either --init or --update-set must be used")
    };

    let test_data = "{ \"files\": [{ \"name\":\"style.css\", \"content\": \"button: { color: red; }\", \"app.js\": \"alert('foo');\" }] }";

    //FIXME: Gives me a sad face to break the chaining
    let file_set = matches.value_of("data")
            .map(FileSet::from_json)
            .expect("--data is mandatory");

    repository.add_files_and_commit(file_set.files);

    println!("Look into the {:0} directory", repository.path);
}
fn get_repository_handle (id: Option<&str>) -> Repository {
    match id {
        None => {
            //why doesn't this yield lifetime issues?
            git::init(&Repository::generate_path(&Uuid::new_v4().to_simple_string())).unwrap()
        },
        Some(id) => {
            Repository {
                path: Repository::generate_path(id)
            }
        }
    }
}
