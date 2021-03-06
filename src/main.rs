extern crate clap;
extern crate atelier;

use clap::{App, Arg};
use atelier::file_set::{ FileSet };
use atelier::repository_locator::{ self, RepositoryState };

fn main() {
    let version = format!("{}.{}.{}{}",
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR"),
                          env!("CARGO_PKG_VERSION_PATCH"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let matches = App::new("clog")
        .version(&version[..])
        .arg(Arg::with_name("init")
            .short("i")
            .long("init")
            .conflicts_with("repository-id")
            .help("if present, data should create new repository"))
        .arg(Arg::with_name("repository-id")
            .short("r")
            .long("repository-id")
            .conflicts_with("init")
            .takes_value(true)
            .help(r#"updates existing repository e.g. --repository-id="4711""#))
        .arg(Arg::with_name("data")
            .short("d")
            .long("data")
            //atom doesn_t seem to handle raw strings pretty well
            //call with --data='{ "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }'
            .help(r#"e.g. { "files": [{ "name":"style.css", "content": "button: { color: red; }"}] }"#)
            .takes_value(true))
        .arg(Arg::with_name("pretty")
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
                repository.add_files_and_commit(file_set.files, "SAVEPOINT");
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
