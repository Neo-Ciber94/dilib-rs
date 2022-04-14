use std::env::var_os;
use std::process::Command;

fn main() {
    // Checks whether we are on nightly and emit it to the compiler
    if is_nightly() {
        println!("cargo:rustc-cfg=nightly")
    }
}

fn is_nightly() -> bool {
    if let Some(version) = rust_version() {
        version.contains("nightly") || version.contains("dev")
    } else {
        false
    }
}

fn rust_version() -> Option<String> {
    let rustc = var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    String::from_utf8(output.stdout).ok()
}
