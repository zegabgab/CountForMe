mod linked_list;
mod fibonacci;
mod syntax_tree;

use linked_list::ListLinked;
use fibonacci::*;
use syntax_tree::SyntaxTree;

fn main() {
    println!("Input a natural number:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading line");
    let n: u32 = input.trim().parse().expect("Please enter a number, thank you <3");

    let tree = SyntaxTree::Twig(
        "sentence",
        vec![
            Box::new(SyntaxTree::Leaf("i")),
            Box::new(SyntaxTree::Leaf("am")),
            Box::new(SyntaxTree::Leaf("trying")),
            Box::new(SyntaxTree::Leaf("here")),
            Box::new(SyntaxTree::Leaf("!"))
        ]
    );
}