use std::sync::atomic::AtomicBool;

lazy_static::lazy_static! {
    pub static ref RETRY_VERIFICATION: AtomicBool = AtomicBool::from(false);
    pub static ref SKIP_REPEATED_EMPTY_PASSWORD: AtomicBool = AtomicBool::from(false);
    pub static ref IGNORE_EMPTY_PASSWORD: AtomicBool = AtomicBool::from(false);
}
