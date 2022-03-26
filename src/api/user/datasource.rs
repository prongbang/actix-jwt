pub trait DataSource {
    fn get_user_me(&self) -> String;
}

pub struct UserDataSource {}

impl UserDataSource {
    pub fn new() -> Box<Self> {
        Box::new(UserDataSource {})
    }
}

impl DataSource for UserDataSource {
    fn get_user_me(&self) -> String {
        "John Wick".to_string()
    }
}