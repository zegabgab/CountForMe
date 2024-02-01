mod earley;

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    name: String,
    children: Vec<Self>
}

impl SyntaxTree {
    pub fn new(name: &str) -> SyntaxTree {
        SyntaxTree {
            name: String::from(name),
            children: Vec::new()
        }
    }

    pub fn with_children(name: &str, children: Vec<SyntaxTree>) -> SyntaxTree {
        SyntaxTree {
            name: String::from(name),
            children
        }
    }
    
    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn child(&self, index: usize) -> &SyntaxTree {
        &self.children[index]
    }
}

impl std::fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Terminal(String),
    Nonterminal(String)
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
    pub components: Vec<Symbol>
}

impl GrammarRule {
    fn new(name: &str, components: &[Symbol]) -> GrammarRule {
        GrammarRule {
            name: name.to_string(), components: components.to_vec()
        }
    }
}