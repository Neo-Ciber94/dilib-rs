#[cfg(nightly)]
#[test]
fn main() {
    macrotest::expand("tests/expand/*.rs");

    // We are currently running on windows, so this tests break in linux
    #[cfg(target_os = "windows")]
    macrotest::expand("tests/macros/expand/*.rs");
}
