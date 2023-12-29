use crate::types::{AuthState, UnlockState};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref AUTH_STATE: Arc<Mutex<Option<AuthState>>> = Arc::new(Mutex::new(Some(AuthState::Idle)));
    pub static ref UNLOCK_STATE: Arc<Mutex<Option<UnlockState>>> = Arc::new(Mutex::new(Some(UnlockState::None)));
    pub static ref PASSWORD: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}
