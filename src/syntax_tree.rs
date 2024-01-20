pub enum SyntaxTree<'a> {
    Twig(&'a str, Vec<Box<SyntaxTree<'a>>>),
    Leaf(&'a str)
}