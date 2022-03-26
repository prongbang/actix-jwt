use std::sync::Arc;
use crate::api::user::datasource::DataSource;

pub trait Repository {
    fn get_user_me(&self) -> String;
}

pub struct UserRepository {
    datasource: Arc<dyn DataSource>,
}

impl UserRepository {
    pub fn new(datasource: Arc<dyn DataSource>) -> Arc<Self> {
        Arc::new(UserRepository { datasource })
    }
}

impl Repository for UserRepository {
    fn get_user_me(&self) -> String {
        self.datasource.get_user_me()
    }
}