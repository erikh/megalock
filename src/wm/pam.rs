use crate::{
    bindings::{
        pam_authenticate, pam_conv, pam_end, pam_handle_t, pam_message, pam_response, pam_set_item,
        pam_start, pam_strerror, PAM_CONV_ERR, PAM_PROMPT_ECHO_OFF, PAM_PROMPT_ECHO_ON,
        PAM_SUCCESS, PAM_USER, PAM_XDISPLAY,
    },
    const_string_ptr,
    utils::{get_display, get_username},
    wm::statics::PASSWORD,
};
use anyhow::{anyhow, Result};
use tracing::{debug, trace};

unsafe extern "C" fn conv_callback(
    num_msg: i32,
    msg: *mut *const pam_message,
    resp: *mut *mut pam_response,
    _appdata_ptr: *mut libc::c_void,
) -> i32 {
    trace!("PAM callback started");

    if num_msg == 0 {
        return PAM_CONV_ERR.try_into().unwrap();
    }

    let mut responses = Vec::new();

    let msg: Vec<*const pam_message> = Vec::from_raw_parts(
        msg,
        num_msg.try_into().unwrap(),
        num_msg.try_into().unwrap(),
    );

    for m in &msg {
        let style: u32 = unsafe { (**m).msg_style }.try_into().unwrap();

        if style != PAM_PROMPT_ECHO_ON && style != PAM_PROMPT_ECHO_OFF {
            responses.push(pam_response {
                resp_retcode: 0,
                resp: std::ptr::null_mut(),
            });
            continue;
        }

        trace!("Setting PAM password to {}", PASSWORD.lock().unwrap());

        let password = PASSWORD.lock().unwrap().as_bytes().to_vec();
        let s = std::ffi::CString::from_vec_unchecked(password);
        let response = pam_response {
            resp_retcode: 0,
            resp: s.as_c_str().as_ptr().cast_mut(),
        };
        responses.push(response);
        std::mem::forget(s);
    }

    std::mem::forget(msg);

    if !responses.is_empty() {
        *resp = responses.as_slice().as_ptr().cast_mut();
        std::mem::forget(responses);
    }

    trace!("PAM callback finished");

    PAM_SUCCESS.try_into().unwrap()
}

pub fn authenticate_password(name: &str) -> Result<()> {
    let conv = pam_conv {
        conv: Some(conv_callback),
        appdata_ptr: std::ptr::null_mut(),
    };

    let mut pam_handle: *mut pam_handle_t = std::ptr::null_mut();

    debug!("Authentication start for '{}'", get_username()?);

    let res = unsafe {
        pam_start(
            const_string_ptr!(name),
            std::ptr::null(),
            &conv,
            &mut pam_handle,
        )
    };
    if res != PAM_SUCCESS.try_into().unwrap() {
        return Err(anyhow!("PAM: {}", unsafe {
            std::ffi::CStr::from_ptr(pam_strerror(pam_handle, res)).to_str()
        }?));
    }

    if pam_handle.is_null() {
        return Err(anyhow!("PAM could not be initialized"));
    }

    let res = unsafe {
        pam_set_item(
            pam_handle,
            PAM_USER.try_into().unwrap(),
            const_string_ptr!(get_username()?),
        )
    };
    if res != PAM_SUCCESS.try_into().unwrap() {
        return Err(anyhow!("PAM: {}", unsafe {
            std::ffi::CStr::from_ptr(pam_strerror(pam_handle, res)).to_str()
        }?));
    }

    let res = unsafe {
        pam_set_item(
            pam_handle,
            PAM_XDISPLAY.try_into().unwrap(),
            const_string_ptr!(get_display()),
        )
    };
    if res != PAM_SUCCESS.try_into().unwrap() {
        return Err(anyhow!("PAM: {}", unsafe {
            std::ffi::CStr::from_ptr(pam_strerror(pam_handle, res)).to_str()
        }?));
    }

    trace!("authentication starting");
    let res = unsafe { pam_authenticate(pam_handle, 0) };
    trace!("authentication finished");

    let authenticated = res == PAM_SUCCESS.try_into().unwrap();

    if authenticated {
        debug!("Authentication successful.");
    } else {
        debug!("Authentication Denied: {} [{}]", res, unsafe {
            std::ffi::CStr::from_ptr(pam_strerror(pam_handle, res)).to_str()
        }?)
    }

    let res = unsafe { pam_end(pam_handle, res) };
    if res != PAM_SUCCESS.try_into().unwrap() {
        return Err(anyhow!("PAM: {}", res));
    }

    if !authenticated {
        return Err(anyhow!("Authentication Denied"));
    }

    trace!("authentication done");

    Ok(())
}
