extern crate rustycalc;

use rustycalc::{lexer, parser};
use rustycalc::types::Operation;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!(
            "A Rusty command-line calculator

Usage:
    calc -i                # Open interactive prompt
    calc '<expression>'    # Calculate and print result of provided expression"
        );
        return;
    }

    match args[1].as_ref() {
        "-i" => interact(),
        _ => evaluate(&args[1..].join(" ")),
    }
}

fn interact() {
    println!("--Kevin's Rusty Calculator--");

    loop {
        match prompt().as_ref() {
            "q" => std::process::exit(0),
            "hi" => println!("Hello!"),
            input => evaluate(input),
        }
    }
}

fn prompt() -> String {
    print!("\n> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn evaluate(input: &str) {
    let tokens = lexer::lex(input).unwrap();
    let expression = parser::parse(&tokens).unwrap();
    let result = expression.evaluate();

    println!("{:?}", tokens);
    println!("{:?}", expression);
    println!("\n{}", result);

}
