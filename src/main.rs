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
    println!("'{}'", input);
}
