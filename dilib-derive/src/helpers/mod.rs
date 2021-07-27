#[allow(unused)]
pub fn token_stream_to_string(tokens: &proc_macro2::TokenStream) -> String {
    tokens
        .to_string()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[allow(unused)]
pub fn token_stream_to_string_non_whitespace(tokens: &proc_macro2::TokenStream) -> String {
    let mut result = String::new();
    let tokens = tokens
        .to_string()
        .split_ascii_whitespace()
        .map(std::string::String::from)
        .collect::<Vec<_>>();

    let mut index = 0;
    while index < tokens.len() {
        let s = &tokens[index];

        // If is a reference, check for the lifetime
        if s == "&" {
            if let Some(lifetime) = tokens.get(index + 1) {
                // Check if is a `'lifetime` and not a char reference.
                if lifetime.starts_with('\'') && !lifetime.ends_with('\'') {
                    if lifetime == "'static" {
                        result.push_str("&'static ");
                    } else {
                        result.push('&');
                    }

                    index += 2;
                }
            }
        }

        if let Some(s) = tokens.get(index) {
            if !s.trim().is_empty() {
                result.push_str(s);
            }
            index += 1;
        }
    }

    result
}
