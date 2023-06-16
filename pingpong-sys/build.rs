use std::{path::PathBuf, env};

extern crate cc;
extern crate bindgen;


fn main() {
    cc::Build::new()
        .file("src/pingpong.c")
        .compile("pingpong");

    println!("cargo:rerun-if-changed=src/pingpong.c");
    bindgen::Builder::default().header("src/pingpong.h")
        .clang_arg("-Ipingpong")
        //.override_abi(abi, arg)
        .layout_tests(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}