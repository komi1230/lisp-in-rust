use std::collections::VecDeque;

pub fn tokenize(s: &str) -> VecDeque<String> {
    // Spread each tokens
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");

    // Split string to each tokens
    let tokens: VecDeque<String> = spreaded
        .trim()
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();

    tokens
}
