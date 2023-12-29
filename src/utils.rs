pub fn get_username() -> Result<String, std::str::Utf8Error> {
    unsafe {
        let pw = libc::getpwuid(libc::getuid());
        std::ffi::CStr::from_ptr((*pw).pw_name)
            .to_str()
            .map(String::from)
    }
}

pub fn get_display() -> String {
    std::env::var("DISPLAY").unwrap_or(":0".to_string())
}

pub(crate) fn get_locale() -> String {
    std::env::var("LC_ALL")
        .or_else(|_| {
            std::env::var("LC_CTYPE")
                .or_else(|_| Ok::<String, ()>(std::env::var("LANG").unwrap_or("C".to_string())))
        })
        .unwrap_or("C".to_string())
        .to_string()
}
