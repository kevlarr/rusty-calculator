use std::io;
use std::io::Write;

fn main() {
    println!("--Kevin's Quirky Calculator--\nEnter expression (eg. 124 + 12) or q to quit");

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

    println!("LH: {}", left_hand);
    println!("Op: {}", operator);
    println!("RH: {}", right_hand);
}

fn parse_integer(opt: Option<&str>) -> Option<i32> {
    match opt {
        Some(s) => match i32::from_str_radix(s, 10) {
            Ok(x) => Some(x),
            Err(_) => None,
        },
        None => None,
    }
}

fn parse_operator(opt: Option<&str>) -> Option<&str> {
    match opt {
        // FIXME
        Some(s) => Some(s),
        None => None,
    }
}
