use uuid::Uuid;
use repository::Repository;
use git::{self};

pub enum RepositoryState<'a> {
    NonExisting,
    Existing(&'a str)
}

pub fn get_repository_handle (id: RepositoryState) -> Repository {
    match id {
        RepositoryState::NonExisting => {
            //why doesn't this yield lifetime issues?
            git::init(&Repository::generate_path(&Uuid::new_v4().to_simple_string())).unwrap()
        },
        RepositoryState::Existing(id) => {
            Repository {
                path: Repository::generate_path(id)
            }
        }
    }
}
