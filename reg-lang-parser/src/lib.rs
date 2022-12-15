#![allow(unused)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
struct MyParser;

pub fn tralala(hehe: &str) -> &str {
    let tralalalala = MyParser::parse(Rule::file, hehe)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    print!("{:?}", tralalalala);

    return "tralala";
}