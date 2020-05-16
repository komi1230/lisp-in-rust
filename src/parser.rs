enum LispData {
    Int(i32),
    Float(f64),
    Text(String),
}


enum LatentExpression {
    Int(i32),
    Float(f64),
    Text(String),
    List(Vec<LispData>)
}


pub fn tokenize(s: &str) -> Vec<String> {

    // Spread each tokens
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");

    // Split string to each tokens
    let tokens: Vec<String> = spreaded.trim().split(" ").map(|item| item.to_string()).collect();

    return tokens;
}


pub fn atom(token: LispData) -> LispData {

    // Integer
    let value_int = token.parse::<i32>();
    if let Ok(value_int) = value_int {
        return value_int.unwrap();
    }

    // Float
    let value_float = token.parse::<f64>();
    if let Ok(value_float) = value_float {
        return value_float.unwrap();
    }

    // String
    return token;
}


pub fn read_from(tokens: Vec<String>) -> LatentExpression {

    // List shouldn't be null
    if tokens.len() == 0 {
        panic!("Unexpected EOF while reading");
    }

    // Get head symbol
    let token: LispData = tokens.pop();

    // Make recurrent list
    if token == "(" {
        let L = Vec::new();
        while tokens[0] != ")" {
            L.push(read_from(tokens()));
        }
        let _ = tokens.pop();

        return L;
    }

    // Invalid parenthesis
    if token == ")" {
        panic!("Unexpected parenthesis");
    }

    return atom(token);
}
