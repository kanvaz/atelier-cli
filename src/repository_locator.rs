use uuid::Uuid;
use repository::Repository;
use git::{self};

pub fn get_repository_handle (id: Option<&str>) -> Repository {
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
