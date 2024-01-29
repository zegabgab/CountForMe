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

pub fn earley_parse(source: impl Iterator<Item = String>) -> SyntaxTree {
    SyntaxTree::with_children("Not implemented yet", source.map(|s| SyntaxTree::new(&s)).collect())
}

#[derive(Debug, Clone)]
enum Symbol {
    Terminal(String),
    Nonterminal(String)
}

struct EarleyItem<'a> {
    rule: &'a GrammarRule,
    start: usize,
    big_fat_dot: usize,
}

impl<'a> EarleyItem<'a> {
    pub fn new(rule: &GrammarRule, start: usize) -> EarleyItem {
        EarleyItem {
            rule, start, big_fat_dot: 0
        }
    }

    pub fn next_unparsed(&self) -> Option<&String> {
        self.rule.components.get(self.big_fat_dot)
    }

    fn advanced(&self) -> EarleyItem {
        EarleyItem { big_fat_dot: self.big_fat_dot + 1, ..*self }
    }
}

struct GrammarRule {
    name: String,
    components: Vec<String>
}