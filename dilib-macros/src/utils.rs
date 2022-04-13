use quote::ToTokens;

pub fn format_tokens<T>(tokens: &T) -> String
where
    T: ToTokens,
{
    tokens
        .to_token_stream()
        .into_iter()
        .flat_map(|t| t.to_string().chars().collect::<Vec<char>>())
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}
