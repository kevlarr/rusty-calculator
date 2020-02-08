extern crate rustycalc;

use rustycalc::lexer;
use rustycalc::types::Operation;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage:
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
    println!("  Enter expression (eg. 124 + 12) or q to quit");
    println!(
        "  Numbers can span from {} to {}",
        std::i64::MIN,
        std::i64::MAX
    );

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
    lexer::lex(input).unwrap();

    let mut pieces = input.split_whitespace();
    let mut piece;

    // Should be three pieces: left arg, operator, and right arg
    piece = pieces.next();
    let left_hand = match parse_integer(piece) {
        Some(x) => x,
        None => {
            eprintln!("Error: Must supply valid left-hand argument");
            return;
        }
    };

    piece = pieces.next();
    let operator = match parse_operator(piece) {
        Some(op) => op,
        None => {
            eprintln!("Error: '{:?}' not a valid operator", piece);
            return;
        }
    };

    piece = pieces.next();
    let right_hand = match parse_integer(piece) {
        Some(x) => x,
        None => {
            eprintln!("Error: Must supply valid right-hand argument");
            return;
        }
    };

    // ... and to be a proper expression there shouldn't be anything left
    if let Some(_) = pieces.next() {
        println!("Error: Invalid expression following operator");
        return;
    }

    match operator(left_hand, right_hand) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("Error: {}", e),
    };
}

fn parse_integer(opt: Option<&str>) -> Option<i64> {
    match opt {
        Some(s) => match i64::from_str_radix(s, 10) {
            Ok(x) => Some(x),
            Err(_) => None,
        },
        None => None,
    }
}

fn parse_operator(opt: Option<&str>) -> Option<Operation> {
    match opt {
        Some(s) => match s {
            "+" => Some(rustycalc::add),
            "-" => Some(rustycalc::subtract),
            "*" => Some(rustycalc::multiply),
            "/" => Some(rustycalc::divide),
            _ => None,
        },
        None => None,
    }
}
