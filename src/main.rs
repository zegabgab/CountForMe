fn main() {
    println!("Input a bracket expression:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading line");

    println!("{}", term(&input.trim().as_bytes(), 0).0);
}

fn term(input: &[u8], index: usize) -> (bool, usize) {
    match index < input.len() {
        true => match parentheses(input, index) {
            (false, _) => (false, index),
            (true, len) => (len == input.len(), len)
        },
        false => (false, index)
    }
}

fn parentheses(input: &[u8], index: usize) -> (bool, usize) {
    let compound = compound(input, index);
    let empty = empty(input, index);
    match compound.0 {
        true => compound,
        false => empty
    }
}

fn empty(input: &[u8], index: usize) -> (bool, usize) {
    (index <= input.len(), index)
}

fn compound(input: &[u8], index: usize) -> (bool, usize) {
    if index >= input.len() || input[index] != b'(' {
        return (false, index);
    }
    let intermediate = parentheses(input, index + 1);
    if !intermediate.0 || intermediate.1 >= input.len() || input[intermediate.1] != b')' {
        (false, index)
    } else {
        parentheses(input, intermediate.1 + 1)
    }
}