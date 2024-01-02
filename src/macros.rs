#[macro_export]
macro_rules! free {
    ($s:expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            libc::free($s as *mut std::ffi::c_void)
        }
    };
}

#[macro_export]
macro_rules! const_string_ptr {
    ($s:expr) => {
        std::ffi::CString::from_vec_unchecked($s.as_bytes().to_vec())
            .as_c_str()
            .as_ptr()
            .cast()
    };
}

#[macro_export]
macro_rules! string_from_ptr {
    ($s:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { std::ffi::CStr::from_ptr($s) }.to_str()
    }};
}

#[macro_export]
macro_rules! load_atomic {
    ($s:expr) => {
        $s.load(std::sync::atomic::Ordering::Relaxed)
    };
}

#[macro_export]
macro_rules! store_atomic {
    ($s:expr, $v:expr) => {
        $s.store($v, std::sync::atomic::Ordering::Relaxed)
    };
}

#[macro_export]
macro_rules! clear_password {
    () => {
        crate::wm::PASSWORD.lock().unwrap().truncate(0)
    };
}
