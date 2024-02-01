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

pub fn earley_parse(source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> Option<SyntaxTree> {
    if grammar.is_empty() { return None; }
    let target = &grammar[0].name;
    let table = earley_table(source, grammar);
    construct_tree(table, target)
}

fn construct_tree(table: Vec<StateSet>, target: &str) -> Option<SyntaxTree> {
    match recognized(&table, target) {
        false => None,
        true => Some(SyntaxTree::new("Success!"))
    }
}

pub fn recognized(table: &[StateSet], target: &str) -> bool {
    match table.last() {
        None => false,
        Some(set) => set.iter()
            .any(|item| item.next_unparsed() == None && item.rule.name == target)
    }
}

#[test]
fn recognized_test() {
    let grammar = vec![GrammarRule { name: "yes".to_string(), 
    components: vec![Symbol::Nonterminal("lol".to_string()), Symbol::Terminal("badada".to_string())]}];
    let first = EarleyItem::new(&grammar[0], 0);
    let second = first.advanced();
    let table = vec![vec![second.advanced(), EarleyItem::new(&grammar[0], 2)]];
    assert!(recognized(&table, "yes"));
    assert!(!recognized(&table, "no"));
}

pub fn earley_table(source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> Vec<StateSet> {
    let mut sets = Vec::new();
    let mut source = source.enumerate().peekable();
    sets.push(Vec::new());
    for r in grammar.iter().filter(|r| r.name == grammar[0].name) {
        let first = &mut sets[0];
        first.push(EarleyItem::new(r, 0));
    }

    loop {
        if source.peek() == None { break; }
        let (i, token) = source.next().unwrap();
        if source.peek() != None { sets.push(Vec::new()); }
        for j in 0.. {
            let len = sets[i].len();
            if j >= len { break; }
            match sets[i][j].next_unparsed() {
                None => {
                    let name = sets[i][j].rule.name.clone();
                    complete(&mut sets, i, j, name);
                },
                Some(Symbol::Nonterminal(symbol)) => {
                    let k = sets[i][j].big_fat_dot;
                    predict(&mut sets, &symbol, i, k, grammar)
                },
                Some(Symbol::Terminal(symbol)) => scan(&mut sets, &symbol, i, &token, j)
            }
        }
    }
    sets
}

fn predict(sets: &mut Vec<StateSet>, symbol: &str, i: usize, k: usize, grammar: &[GrammarRule]) {
    for item in grammar.iter().filter(|r| r.components.len() > k && r.components[k].matches(symbol))
    .map(|r| EarleyItem::new(r, i)) {
        if !sets[i].contains(&item) {
            sets[i].push(item);
        }
    }
}

fn scan(sets: &mut Vec<StateSet>, symbol: &str, i: usize, token: &str, j: usize) {
    if symbol == token && i < sets.len() - 1 {
        let set = sets[i][j].advanced();
        if !sets[i + 1].contains(&set) {
            sets[i + 1].push(set)
        }
    }
}

fn complete(sets: &mut Vec<StateSet>, i: usize, j: usize, name: String) {
    let start = sets[i][j].start;
    for k in 0..sets[start].len() {
        if sets[start][k].next_unparsed() == Some(Symbol::Nonterminal(name.clone())) {
            let item = sets[start][k].advanced();
            if !sets[i].contains(&item) { sets[i].push(item); }
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

#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub struct EarleyItem {
    rule: GrammarRule,
    start: usize,
    big_fat_dot: usize,
}

impl EarleyItem {
    pub fn new(rule: &GrammarRule, start: usize) -> EarleyItem {
        EarleyItem {
            rule: rule.clone(), start, big_fat_dot: 0
        }
    }

    pub fn next_unparsed(&self) -> Option<Symbol> {
        self.rule.components.get(self.big_fat_dot).cloned()
    }

    fn advanced(&self) -> EarleyItem {
        EarleyItem { rule: self.rule.clone(), big_fat_dot: self.big_fat_dot + 1, ..*self }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub components: Vec<Symbol>
}

type StateSet = Vec<EarleyItem>;