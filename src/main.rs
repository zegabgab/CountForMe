mod linked_list;
mod fibonacci;

use linked_list::ListLinked;
use fibonacci::*;

fn main() {
    println!("Input a natural number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading line");
    let n: u32 = input.trim().parse().expect("Please enter a number, thank you <3");

    let mut list = ListLinked::<u32>::new();

    for i in 0..=n {
        list.add(fibonacci(i), i);
    }

    for i in (0..=n).rev() {
        match list.get(i) {
            Ok(value) => println!("F({i}) = {}", value),
            Err(msg) => println!("Error: {msg}")
        }
    }
}