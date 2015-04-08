extern crate clap;
extern crate rustc_serialize;
extern crate uuid;

use clap::{App, Arg};
use git::{};
use repository_locator::{ RepositoryState };
use file_set:: { FileSet };
mod git;
mod repository;
mod repository_locator;
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
            //atom doesn_t seem to handle raw strings pretty well
            //call with --data='{ "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }'
            .help(r#"e.g. { "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }"#)
            .takes_value(true))
        .get_matches();

    let repository = if matches.is_present("init") {
        repository_locator::get_repository_handle(RepositoryState::NonExisting)
    } else if matches.is_present("update-set") {
        repository_locator::get_repository_handle(RepositoryState::Existing(matches.value_of("update-set").unwrap()))
    } else {
        panic!("Either --init or --update-set must be used")
    };

    matches.value_of("data")
            .map(FileSet::from_json)
            .map(|file_set| { repository.add_files_and_commit(file_set.files) })
            .expect("--data is mandatory");

    println!("Look into the {:0} directory", repository.path);
}
