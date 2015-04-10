extern crate clap;
extern crate rustc_serialize;
extern crate uuid;
extern crate readext;

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
            .mutually_excludes("repository-id")
            .help("if present, data should create new repository"))
        .arg(Arg::new("repository-id")
            .short("r")
            .long("repository-id")
            .mutually_excludes("init")
            .takes_value(true)
            .help(r#"updates existing repository e.g. --repository-id="4711""#))
        .arg(Arg::new("data")
            .short("d")
            .long("data")
            //atom doesn_t seem to handle raw strings pretty well
            //call with --data='{ "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }'
            .help(r#"e.g. { "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }"#)
            .takes_value(true))
        .arg(Arg::new("pretty")
            .long("pretty")
            .short("p")
            .help("pretty print all output"))
        .get_matches();

    let repository = if matches.is_present("init") {
        repository_locator::get_repository_handle(RepositoryState::NonExisting)
    } else if matches.is_present("repository-id") {
        repository_locator::get_repository_handle(RepositoryState::Existing(matches.value_of("repository-id").unwrap()))
    } else {
        println!("{}", matches.usage());
        println!("use --help for more info");
        std::process::exit(1);
    };

    //if --data was specified
    matches.value_of("data")
            .map(FileSet::from_json)
            .map(|file_set| {
                repository.add_files_and_commit(file_set.files);
                println!("Repository created or updated at ./{:0}", repository.path)
            });

    //if --data wasn't specified we have a read operation
    if !matches.is_present("data") {
        let file_set = FileSet { files: repository.read_all_files() };

        match matches.is_present("pretty") {
            true => println!("{}", file_set.to_pretty_json()),
            false => println!("{}", file_set.to_json())
        }
    }
}
