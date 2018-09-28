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
    let left_hand = match pieces.next() {
        None => {
            println!("Must provide left-hand argument");
            return;
        },
        Some(s) => match isize::from_str_radix(s, 10) {
            Ok(x) => x,
            Err(_) => {
                println!("Left-hand argument must be an integer");
                return;
            },
        },
    };

    let operator = match pieces.next() {
        None => {
            println!("Must provide operator");
            return;
        },
        Some(s) => match operator_from_str(s) {
            Ok(op) => op,
            Err(_) => {
                println!("Must supply valid operator");
                return;
            },
        },
    };

    println!("LH: {}", left_hand);
    println!("Op: {}", operator);
}

fn operator_from_str(s: &str) -> Result<&str, &str> {
    // FIXME
    Ok(s)
}
