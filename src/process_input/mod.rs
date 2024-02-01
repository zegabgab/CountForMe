use self::parse::{Symbol, SyntaxTree};

mod parse;
mod lexer;

pub fn process(input: &mut impl std::io::BufRead, running: &mut bool) -> Result<(), ()> {
    let mut line = String::new();
    let read = input.read_line(&mut line);
    let tree = SyntaxTree::with_children("test", vec![SyntaxTree::new("more test")]);
    let _ = tree.name();

    let lexer = lexer::Lexer::new(line.trim().chars());
    let grammar = vec![
        parse::GrammarRule {
            name : "Parens".to_string(), 
            components: vec![Symbol::Terminal("(".to_string()), Symbol::Terminal(")".to_string())]
        },
        parse::GrammarRule {
            name: "Parens".to_string(),
            components: vec![Symbol::Terminal("(".to_string()), Symbol::Nonterminal("Parens".to_string()), Symbol::Terminal(")".to_string())]
        },
        parse::GrammarRule {
            name: "Parens".to_string(),
            components: vec![Symbol::Nonterminal("Parens".to_string()), Symbol::Nonterminal("Parens".to_string()), ]
        },
        parse::GrammarRule {
            name: "Parens".to_string(),
            components: vec![Symbol::Terminal("(".to_string()), Symbol::Terminal("banana".to_string()), Symbol::Terminal(")".to_string())]
        },
        parse::GrammarRule {
            name: "Parens".to_string(),
            components: vec![Symbol::Terminal("(".to_string()), Symbol::Terminal("ananab".to_string()), Symbol::Terminal(")".to_string())]
        }
    ];

    if let Err(_) = read {
        eprintln!("Error while reading");
        *running = false;
        return Err(());
    }
    if parse::recognize(lexer, &grammar) {
        println!("Banana expression found!");
    } else {
        println!("No valid expression found");
    }
    Ok(())
}