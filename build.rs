use std::env;

fn main() {
    //TODO use crate cmake to build and link
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let jigsawlib_dir = format!("{cargo_dir}/jigsaw/build");
    //panic!("Im here {jigsawlib_dir}");

    //https://flames-of-code.netlify.app/blog/rust-and-cmake-cplusplus/
    println!("cargo:rustc-link-lib=dylib=stdc++"); 
    println!("cargo:rustc-link-search={jigsawlib_dir}");
}
