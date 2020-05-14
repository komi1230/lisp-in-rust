fn tokenize(s: &str) -> Vec<&str>{
    s.replace("(", " ( ").replace(")", " ) ").split(" ")
}


