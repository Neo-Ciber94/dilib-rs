use std::{fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=README.md");
    let file_path = fs::canonicalize("../README.md").unwrap();
    let readme = fs::read_to_string(file_path.clone()).unwrap();
    let output = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("README-lib.md");
    fs::write(output, &readme).unwrap();
}