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

pub fn earley_parse(source: impl Iterator<Item = String>, grammar: Vec<GrammarRule>) -> Option<SyntaxTree> {
    if grammar.is_empty() { return None; }
    let mut sets = Vec::new();
    let mut source = source.enumerate().peekable();
    sets.push(Vec::new());
    for r in grammar.iter().filter(|r| r.name == grammar[0].name) {
        sets[0].push(EarleyItem::new(r, 0));
    }

    for (i, token) in source {
        let set = &mut sets[i];
        for item in set {
            set.push(item.advanced());
            match item.next_unparsed() {
                None => complete(),
                Some(item) => {
                    match item {
                        Symbol::Terminal(symbol) => scan(),
                        Symbol::Nonterminal(symbol) => predict()
                    }
                }
            }
        }
    }
    None
}

fn predict() {
    todo!()
}

fn scan() {
    todo!()
}

fn complete() {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Symbol {
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

#[derive(PartialEq, Eq)]
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

    pub fn next_unparsed(&self) -> Option<&Symbol> {
        self.rule.components.get(self.big_fat_dot)
    }

    fn advanced(&self) -> EarleyItem {
        EarleyItem { big_fat_dot: self.big_fat_dot + 1, ..*self }
    }
}

#[derive(PartialEq, Eq)]
struct GrammarRule {
    name: String,
    components: Vec<Symbol>
}

type StateSet<'a> = Vec<EarleyItem<'a>>;