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


pub fn tokenize(s: &str) -> Vec<String>{
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");
    
    let tokens: Vec<String> = spreaded.split(" ").map(|item| item.to_string()).collect();

    return tokens;
}


//pub fn read_from(tokens: Vec<String>) 
