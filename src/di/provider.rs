use std::sync::Arc;
use crate::api::user;
use crate::api::user::datasource::UserDataSource;
use crate::api::user::repository::UserRepository;

pub struct Container {
    pub user_repo: Arc<dyn user::repository::Repository>,
}

pub fn inject() -> Container {
    let user_ds = UserDataSource::new();
    let user_repo = UserRepository::new(user_ds);

    Container {
        user_repo: user_repo.clone(),
    }
}