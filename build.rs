extern crate bindgen;
extern crate cc;

use std::{env, path};

fn main() {
    cc::Build::new()
        .file("build.c")
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .warnings(false)
        .compile("stb_image");

    //println!("cargo:rustc-link-lib=stb_image");

    // gen bindings
    bindgen::builder()
        .header("stb/stb_image.h")

        .whitelist_function("stbi_(.*)")
        .whitelist_type("stbi_(.*)")

        .layout_tests(false)
        .rustfmt_bindings(true)

        .generate()
        .expect("Error generating bindings!!")
        
        // write to disk
        .write_to_file({
            let mut path = path::PathBuf::from(
                env::var_os("OUT_DIR").unwrap().to_str().unwrap()
            );

            path.push("stb_image.rs");
            path
        })
        .expect("Error writing bindings!!");
}
