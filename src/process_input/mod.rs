use crate::process_input::parse::parse;

use self::parse::{Symbol, SyntaxTree};

mod parse;
mod lexer;

pub fn process(input: &mut impl std::io::BufRead, running: &mut bool) -> Result<(), ()> {
    let mut line = String::new();
    let read = input.read_line(&mut line);
    let tree = SyntaxTree::with_children("test", vec![SyntaxTree::new("more test")]);
    let _ = tree.name();

    let lexer = lexer::Lexer::new(line.trim().chars());
    use parse::GrammarRule;
    /*let grammar = vec![
        GrammarRule::new("Parens", &[Symbol::Terminal("(".to_string()), Symbol::Terminal(")".to_string())]),
        GrammarRule::new("Parens", &[Symbol::Terminal("(".to_string()), Symbol::Nonterminal("Parens".to_string()), Symbol::Terminal(")".to_string())]),
        GrammarRule::new("Parens", &[Symbol::Nonterminal("Parens".to_string()), Symbol::Nonterminal("Parens".to_string())]),
        GrammarRule::new("Parens", &[Symbol::Terminal("(".to_string()), Symbol::Terminal("banana".to_string()), Symbol::Terminal(")".to_string())]),
        GrammarRule::new("Parens", &[Symbol::Terminal("(".to_string()), Symbol::Terminal("ananab".to_string()), Symbol::Terminal(")".to_string())]),
    ];*/
    let grammar = vec![
        GrammarRule::new("Sum", &[Symbol::Nonterminal("Sum".to_string()), Symbol::Terminal("+".to_string()), Symbol::Nonterminal("Product".to_string())]),
        GrammarRule::new("Sum", &[Symbol::Nonterminal("Product".to_string())]),
        GrammarRule::new("Factor", &[Symbol::Terminal("(".to_string()), Symbol::Nonterminal("Sum".to_string()), Symbol::Terminal(")".to_string())]),
        GrammarRule::new("Factor", &[Symbol::Terminal("var".to_string())]),
        GrammarRule::new("Factor", &[Symbol::Terminal("num".to_string())]),
        GrammarRule::new("Product", &[Symbol::Nonterminal("Product".to_string()), Symbol::Terminal("*".to_string()), Symbol::Nonterminal("Factor".to_string())]),
        GrammarRule::new("Product", &[Symbol::Nonterminal("Factor".to_string())])
    ];

    if let Err(_) = read {
        eprintln!("Error while reading");
        *running = false;
        return Err(());
    }
    let tree = parse(lexer, &grammar);
    match tree {
        None => println!("no banana"),
        Some(tree) => println!("{tree}"),
    }
    Ok(())
}