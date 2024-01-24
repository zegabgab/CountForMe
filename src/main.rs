mod linked_list;
mod fibonacci;
mod syntax_tree;
mod parse;

use linked_list::ListLinked;
use fibonacci::*;
use syntax_tree::SyntaxTree;

fn main() {
    let input = vec![
        String::from("banana"),
        String::from("+"),
        String::from("banana")
    ];

    match parse::parse_term(&input) {
        None => println!("no banana"),
        Some(result) => println!("{result}")
    }
}