use std::{fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=README.md");
    let readme = fs::read_to_string("../README.md").unwrap();
    let output = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("README-lib.md");
    fs::write(output, readme).unwrap();
}