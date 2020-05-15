extern crate minilisp;

use std::io;

fn main() {
    println!("Hello, world!");

    let s = "(+ 1 2)";
    println!("Before : {}", s);

    let ss = minilisp::parser::tokenize(s);
    for i in ss{
        println!("{}", i);
    };
}
