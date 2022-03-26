use crate::api::user::datasource::UserDataSource;
use crate::api::user::repository::UserRepository;
use crate::api::user::usecase::{UserUseCase};

pub fn new() -> Box<UserUseCase> {
    UserUseCase::new(
        UserRepository::new(
            UserDataSource::new()
        )
    )
}