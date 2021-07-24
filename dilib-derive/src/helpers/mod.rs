
pub fn token_stream_to_string(tokens: proc_macro2::TokenStream) -> String {
    tokens
        .to_string()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join("")
}
