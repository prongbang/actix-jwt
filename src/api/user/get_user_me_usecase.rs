use std::sync::Arc;
use crate::api::user::repository::Repository;
use crate::core::usecase::UseCase;

pub struct GetUserMeUseCase {
    pub repository: Arc<dyn Repository>,
}

impl GetUserMeUseCase {
    pub fn new(repository: Arc<dyn Repository>) -> Arc<dyn UseCase<String, String>> {
        Arc::new(GetUserMeUseCase { repository })
    }
}

impl UseCase<String, String> for GetUserMeUseCase {
    fn execute(&self, _param: String) -> String {
        self.repository.get_user_me()
    }
}

pub async fn execute(repository: &Arc<dyn Repository>) -> String {
    return repository.get_user_me();
}