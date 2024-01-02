use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref PASSWORD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}
