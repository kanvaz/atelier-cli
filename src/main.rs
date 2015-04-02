extern crate clap;

use clap::{App, Arg};
//doesn't compile today, try tomorrow ;)
//use uuid::Uuid;
use git::{};
use repository::{FileData};

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
            .help("e.g. { \"files\": { \"style.css\": \"button: { color: red; }\", \"app.js\": \"alert('foo');\" } }")
            .takes_value(true))
        .get_matches();

    let repository = git::init("foo").unwrap();

    repository.add_files(vec!(
        FileData { name: "foo.txt", content: "foobar" },
        FileData { name: "bar.txt", content: "bar" },
    ));

    repository.commit_all();
    println!("Look into the foo directory");
}
