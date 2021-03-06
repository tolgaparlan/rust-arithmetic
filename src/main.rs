use crate::tokenizer::tokenize;
use crate::parser::parse;
use crate::evaluator::eval_expr;
use std::process::exit;

mod tokenizer;
mod parser;
mod evaluator;

fn main() {
    let input = std::io::stdin();
    loop {
        let mut line = String::new();

        if let Err(e) = input.read_line(&mut line) {
            eprintln!("Input Error: {}", e);
            exit(1);
        };

        if line.trim() == "exit" {
            exit(0);
        }

        let tokens = tokenize(line);
        if let Err(err) = tokens {
            eprintln!("{}", err);
            continue;
        }

        let expr = parse(tokens.unwrap());
        if let Err(err) = expr {
            eprintln!("{}", err);
            continue;
        }

        println!("{}", eval_expr(expr.unwrap()));
    }
}