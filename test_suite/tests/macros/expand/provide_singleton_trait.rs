use dilib::provide;
use std::fmt::Display;

#[provide(bind="Display")]
fn get_singleton_trait() -> String {
    "hola".to_string()
}