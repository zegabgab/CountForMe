use self::earley::earley_parse;

pub mod scan;
mod earley;

pub fn parse(source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> Option<SyntaxTree> {
    earley_parse(source, grammar)
}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    name: String,
    children: Vec<Self>,
}

impl SyntaxTree {
    pub fn new(name: &str) -> SyntaxTree {
        SyntaxTree {
            name: String::from(name),
            children: Vec::new(),
        }
    }

    pub fn with_children(name: &str, children: Vec<SyntaxTree>) -> SyntaxTree {
        SyntaxTree {
            name: String::from(name),
            children,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const INDENT: &str = "  ";

        fn format(
            s: &SyntaxTree,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            for _ in 0..depth {
                write!(f, "{INDENT}")?;
            }
            write!(f, "{}", s.name)?;
            let first = s.children.get(0);
            match first {
                None => Ok(()),
                Some(child) => {
                    write!(f, ":\n")?;
                    format(child, f, depth + 1)?;
                    for child in s.children.iter().skip(1) {
                        write!(f, "\n")?;
                        format(child, f, depth + 1)?;
                    }
                    Ok(())
                }
            }
        }

        format(self, f, 0)
        /*
        write!(f, "{}", self.name)?;
        let mut iter = self.children.iter();
        match iter.next() {
            None => Ok(()),
            Some(child) => {
                write!(f, "[{}", child)?;
                for child in iter  {
                    write!(f, ", {child}")?;
                }
                write!(f, "]")
            }
        }
        */
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Terminal(String),
    Nonterminal(String),
}

impl Symbol {
    fn matches(&self, token: &str) -> bool {
        match self {
            Symbol::Terminal(s) => s == token,
            Symbol::Nonterminal(s) => s == token,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub components: Vec<Symbol>,
}

impl GrammarRule {
    pub fn new(name: &str, components: &[Symbol]) -> GrammarRule {
        GrammarRule {
            name: name.to_string(),
            components: components.to_vec(),
        }
    }
}
