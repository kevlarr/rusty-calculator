extern crate logics;

use std::io;
use std::io::Write;
use logics::types::{Num, OperationResult};

fn main() {
    println!("--Kevin's Quirky Calculator--");
    println!("  Enter expression (eg. 124 + 12) or q to quit");
    println!("  Numbers can span from {} to {}", std::i8::MIN, std::i8::MAX);

    loop {
        match get_input() {
            ref input if input == "q" => std::process::exit(0),
            ref input if input == "hi" => println!("Hello!"),
            input => eval(input),
        }
    }
}

fn get_input() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn eval(input: String) {
    let mut pieces = input.split_whitespace();

    // Should be three pieces: left arg, operator, and right arg
    let left_hand = match parse_integer(pieces.next()) {
        Some(x) => x,
        None => {
            println!("Error: Must supply valid left-hand argument");
            return;
        },
    };
    let operator = match parse_operator(pieces.next()) {
        Some(op) => op,
        None => {
            println!("Error: Must supply valid operator");
            return;
        },
    };
    let right_hand = match parse_integer(pieces.next()) {
        Some(x) => x,
        None => {
            println!("Error: Must supply valid right-hand argument");
            return;
        },
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

fn parse_integer(opt: Option<&str>) -> Option<Num> {
    match opt {
        Some(s) => match Num::from_str_radix(s, 10) {
            Ok(x) => Some(x),
            Err(_) => None,
        },
        None => None,
    }
}

fn parse_operator(opt: Option<&str>) -> Option<fn(Num, Num) -> OperationResult> {
    match opt {
        Some(s) => match s {
            "+" => Some(logics::operations::add),
            "-" => Some(logics::operations::subtract),
            "*" => Some(logics::operations::multiply),
            "/" => Some(logics::operations::divide),
            _ => None,
        },
        None => None,
    }
}
