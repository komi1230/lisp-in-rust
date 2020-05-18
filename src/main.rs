extern crate minilisp;

fn main() {
    println!("Hello, world!");

    let s = "   (+ 1 (- 2.2 3))";
    println!("Before : {}", s);

    let ss = minilisp::parser::tokenize(s);
    println!("Tokens: {:?}", ss);
}
