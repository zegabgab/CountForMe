mod linked_list;
mod fibonacci;
mod syntax_tree;

use linked_list::ListLinked;
use fibonacci::*;
use syntax_tree::SyntaxTree;

fn main() {
    let tree = SyntaxTree::new_with_children(
        "term",
        vec![
            SyntaxTree::new("("),
            SyntaxTree::new("5"),
            SyntaxTree::new("/"),
            SyntaxTree::new("0"),
            SyntaxTree::new(")")]
    );
    println!("{tree}");
}