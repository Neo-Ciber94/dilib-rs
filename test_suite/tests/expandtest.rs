#[cfg(nightly)]
#[test]
fn main() {
    macrotest::expand("tests/expand/*.rs");
    macrotest::expand("tests/macros/expand/*.rs");
}
