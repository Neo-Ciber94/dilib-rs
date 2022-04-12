use dilib::provide;

#[provide(name="es_greet")]
fn get_greet() -> String {
    "Hola Mundo!".to_string()
}