pub fn tokenize(s: &str) -> Vec<String>{
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");
    
    let tokens: Vec<String> = a.split(" ").map(|item| item.to_string()).collect();

    return tokens;
}
