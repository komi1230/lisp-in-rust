extern crate minilisp;

fn main() {
    println!("Hello, world!");

    let s = "   (+ 1 (- 2.2 3))";
    println!("Before : {}", s);

    let mut ss = minilisp::parser::tokenize(s);
    for i in &ss {
        println!("{}", i);
    }

    let x = minilisp::parser::read_from(&mut ss);
    println!("latent expression: {:?}", x);
}
