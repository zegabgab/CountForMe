mod linked_list;
mod fibonacci;
mod syntax_tree;

use linked_list::ListLinked;
use fibonacci::*;
use syntax_tree::SyntaxTree;

fn main() {
    let mut tree = SyntaxTree::new_with_children(
        "another_sentence",
        vec![
            SyntaxTree::new("we"),
            SyntaxTree::new("are"),
            SyntaxTree::new("getting"),
            SyntaxTree::new("there")]
    );
    tree.child_as_mutable(0).add_child(SyntaxTree::new("by which i mean, i"));
    println!("{tree}");
}