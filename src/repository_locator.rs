use uuid::Uuid;
use repository::Repository;
use git::{self};

pub enum RepositoryState<'a> {
    NonExisting,
    Existing(&'a str)
}

static REPO_PREFIX :&'static str = "TEMP_REP_";

pub fn get_repository_handle (id: RepositoryState) -> Repository {
    match id {
        RepositoryState::NonExisting => {
            git::init(&generate_path(&Uuid::new_v4().to_simple_string())).unwrap()
        },
        RepositoryState::Existing(id) => {
            Repository {
                path: generate_path(id)
            }
        }
    }
}

pub fn generate_path (id: &str) -> String {
    String::new() + REPO_PREFIX + id
}
