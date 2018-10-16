use super::{
    circuits, conversions,
    types::{Circuit, Num, OperationResult}
};

pub fn add(x: Num, y: Num) -> OperationResult {
    run_circuit(circuits::binary_adder, x, y)
}

pub fn subtract(x: Num, y: Num) -> OperationResult {
    run_circuit(circuits::binary_subtractor, x, y)
}

pub fn multiply(x: Num, y: Num) -> OperationResult {
    run_circuit(circuits::binary_multiplier, x, y)
}

pub fn divide(x: Num, y: Num) ->  OperationResult {
    Ok(x / y)
}

fn run_circuit(circuit: Circuit, x: Num, y: Num) -> OperationResult {
    let xb = conversions::to_bit_array(x);
    let yb = conversions::to_bit_array(y);

    println!("\nx: {}\n{:#010b}\n{:?}", x, x, xb);
    println!("\ny: {}\n{:#010b}\n{:?}", y, y, yb);

    let result = circuit(xb, yb);
    println!("\nr: {:?}", result);

    conversions::from_bit_array(result)
}

#[cfg(test)]
mod tests {
    use super::{add};

    #[test]
    fn add_works_for_positives() {
        let args = [
            (0, 1), (0, 100), (0, 500),
            (1, 2), (1, 200), (1, 200000),
            (123321, 192392), (98498239, 1238723),
            (0, ::std::i8::MAX)
        ];
        for (x, y) in args.iter() {
            assert_eq!(add(*x, *y), Ok(x + y));
        }
    }

    #[test]
    fn add_works_for_negatives() {
        let args = [
            (0, -1), (0, -100), (0, -500),
            (1, -2), (1, -200), (1, -200000),
            (-1, -2), (-1, -200), (-1, -200000),
            (-123321, -192392), (-98498239, 1238723),
            (0, ::std::i8::MIN), (1, ::std::i8::MIN),
        ];
        for (x, y) in args.iter() {
            assert_eq!(add(*x, *y), Ok(x + y), "\nTesting: {} + {}", x, y);
        }
    }
}
