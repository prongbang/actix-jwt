use std::sync::Arc;

pub trait DataSource {
    fn get_user_me(&self) -> String;
}

pub struct UserDataSource {}

impl UserDataSource {
    pub fn new() -> Arc<dyn DataSource> {
        Arc::new(UserDataSource {})
    }
}

impl DataSource for UserDataSource {
    fn get_user_me(&self) -> String {
        "Devไปวันๆ".to_string()
    }
}