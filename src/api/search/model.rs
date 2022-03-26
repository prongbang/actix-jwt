pub trait Search {
    fn search(&self, query: String) -> String;
}

pub struct SearchClient {}

impl SearchClient {
    pub fn new() -> Self {
        SearchClient {}
    }
}

impl Search for SearchClient {
    fn search(&self, query: String) -> String {
        format!("Search: {}", query).to_string()
    }
}
