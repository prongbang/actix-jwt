use crate::api::user::repository::{Repository};

pub trait UseCase {
    fn get_user_me(&self) -> String;
}

pub struct UserUseCase {
    repository: Box<dyn Repository>,
}

impl UserUseCase {
    pub fn new(repository: Box<dyn Repository>) -> Box<Self> {
        Box::new(UserUseCase { repository })
    }
}

impl UseCase for UserUseCase {
    fn get_user_me(&self) -> String {
        self.repository.get_user_me()
    }
}