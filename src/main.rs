mod linked_list;
mod fibonacci;
mod syntax_tree;
mod parse;

use std::env;

use linked_list::ListLinked;
use fibonacci::*;
use syntax_tree::SyntaxTree;

fn main() {
    let input = vec![
        String::from("("),
        String::from("banana"),
        String::from("+"),
        String::from("banana"),
        String::from(")")
    ];
    let input = env::args().skip(1).map(|s| String::from(s)).collect();

    match parse::parse_term(&input) {
        None => println!("no banana"),
        Some(result) => println!("{result}")
    }
}