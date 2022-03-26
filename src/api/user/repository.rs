use crate::api::user::datasource::DataSource;

pub trait Repository {
    fn get_user_me(&self) -> String;
}

pub struct UserRepository {
    datasource: Box<dyn DataSource>,
}

impl UserRepository {
    pub fn new(datasource: Box<dyn DataSource>) -> Box<Self> {
        Box::new(UserRepository { datasource })
    }
}

impl Repository for UserRepository {
    fn get_user_me(&self) -> String {
        self.datasource.get_user_me()
    }
}