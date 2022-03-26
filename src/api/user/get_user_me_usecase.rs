use crate::api::user::repository::Repository;
use crate::core::usecase::UseCase;

pub struct GetUserMeUseCase {
    pub repository: Box<dyn Repository>,
}

impl GetUserMeUseCase {
    pub fn new(repository: Box<dyn Repository>) -> Box<dyn UseCase<String, String>> {
        Box::new(GetUserMeUseCase { repository })
    }
}

impl UseCase<String, String> for GetUserMeUseCase {
    fn execute(&self, _param: String) -> String {
        self.repository.get_user_me()
    }
}

pub async fn execute(repository: &Box<dyn Repository>) -> String {
    return repository.get_user_me();
}