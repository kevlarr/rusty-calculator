# Rusty Calculator

<sub>Or, *A Pointlessly Complex Calculator Prompt*</sub>

Performs basic integer math without using numbers or operations by
simulating logic gates and arithmetic circuits.

## Usage

With Rust 1.31+ and `cargo` installed:

1. `git clone https://github.com/kevlarr/rusty-calc`
2. `cd rusty-calc`
3. `cargo run -- -i` for an interactive prompt, or `cargo run -- '4,000 + 12 * 3'` for a single calculation
4. Make fun of it

## TODO

- [ ] Support float

### Expression parsing

- [x] Parse simple binary arithmetic operations
- [x] Support operator precedence without requiring parentheses
- [x] Support nested expressions via parentheses

### Circuits

- [x] Add
- [x] Subtract
- [x] Multiply
- [x] Divide
- [ ] Exponentiation
- [ ] Modulo
