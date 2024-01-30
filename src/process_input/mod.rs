use self::parse::SyntaxTree;

mod parse;
mod lexer;

pub fn process(input: &mut impl std::io::BufRead, running: &mut bool) -> Result<(), ()> {
    let mut line = String::new();
    let read = input.read_line(&mut line);
    let tree = SyntaxTree::with_children("test", vec![SyntaxTree::new("more test")]);
    let _ = tree.name();

    let lexer = lexer::Lexer::new(line.chars());

    if let Err(_) = read {
        eprintln!("Error while reading");
        *running = false;
        return Err(());
    }
    let tokens: Vec<String> = lexer.collect();
    if tokens.is_empty() {
        println!("Blank line entered, closing...");
        *running = false;
    } else {
        println!("{:?}", tokens);
    }
    Ok(())
}