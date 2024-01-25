use self::parse::SyntaxTree;

mod parse;

pub fn process(input: &mut impl std::io::BufRead, running: &mut bool) -> Result<(), ()> {
    let mut line = String::new();
    let read = input.read_line(&mut line);
    let tree = SyntaxTree::with_children("test", vec![SyntaxTree::new("more test")]);
    let _ = tree.name();
    if let Err(_) = read {
        eprintln!("Error while reading");
        *running = false;
        return Err(());
    }
    if line.len() <= 2 {
        *running = false;
        println!("Empty line detected, closing...")
    } else {
        print!("Echoing {line}");
    }
    Ok(())
}