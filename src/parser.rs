use std::collections::VecDeque;

#[derive(Debug)]
pub enum LatentExpression {
    Int(i32),
    Float(f64),
    Text(String),
    List(Vec<LatentExpression>),
}

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

pub fn read_from(tokens: &mut VecDeque<String>) -> LatentExpression {
    // List shouldn't be null
    if tokens.len() == 0 {
        panic!("Unexpected EOF while reading");
    }

    let token = tokens.pop_front().unwrap();

    if token == "(" {
        let mut list = vec![];
        while tokens.len() > 0 && tokens[0] != ")" {
            list.push(read_from(tokens))
        }
        return LatentExpression::List(list);
    }

    // Invalid parenthesis
    if token == ")" {
        panic!("Unexpected parenthesis");
    }

    if let Ok(value_int) = token.parse::<i32>() {
        return LatentExpression::Int(value_int);
    }

    if let Ok(value_float) = token.parse::<f64>() {
        return LatentExpression::Float(value_float);
    }

    LatentExpression::Text(token.to_string())
}
