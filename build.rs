extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=xkbcommon");
    println!("cargo:rustc-link-lib=xkbcommon-x11");
    println!("cargo:rustc-link-lib=xkbregistry");
    println!("cargo:rustc-link-lib=cairo");
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=xcb-randr");
    println!("cargo:rustc-link-lib=xcb-image");
    println!("cargo:rustc-link-lib=xcb-xrm");
    println!("cargo:rustc-link-lib=xcb-xkb");
    println!("cargo:rustc-link-lib=xcb-render");
    println!("cargo:rustc-link-lib=xcb-shm");
    println!("cargo:rustc-link-lib=xcb-util");
    println!("cargo:rustc-link-lib=pam");

    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("wrapper.h")
        .ctypes_prefix("libc")
        .allowlist_item("XCB.*")
        .allowlist_item("XKB.*")
        .allowlist_item("CAIRO.*")
        .allowlist_item("PAM.*")
        .allowlist_item("xcb.*")
        .allowlist_item("xkb.*")
        .allowlist_item("cairo.*")
        .allowlist_item("pam.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
