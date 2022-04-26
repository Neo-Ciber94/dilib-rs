use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=README.md");

    let readme = fs::read_to_string("README.md").expect("Failed to read README.md");
    let output = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("README-lib.md");
    fs::write(output, readme).expect("Failed to write README-lib.md");
}
