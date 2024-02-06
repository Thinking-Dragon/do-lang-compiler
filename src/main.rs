mod token;
mod tokenizer;

mod ast;
mod ast_parser;

use std::env;
use std::fs;

use crate::tokenizer::tokenize;
use crate::ast_parser::parse_ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_code = fs::read_to_string(args.get(1).unwrap()).unwrap();
    
    let tokens = tokenize(source_code);
    println!("Tokens:\n{:#?}", tokens);
}
