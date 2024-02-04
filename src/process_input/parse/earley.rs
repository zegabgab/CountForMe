use super::{GrammarRule, Symbol};

#[derive(PartialEq, Eq)]
struct EarleyItem<'a> {
    rule: &'a GrammarRule,
    start: usize,
    current: usize,
}

type StateSet<'a> = Vec<EarleyItem<'a>>;
type EarleyTable<'a> = Vec<(Option<String>, StateSet<'a>)>;

pub fn earley_recognize(source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> bool {
    let table = earley_table(source, grammar);
    match table.last() {
        None => false,
        Some((_, items)) => {
            items.iter().any(|item| item.rule.name == grammar[0].name && item.start == 0 && item.next_unparsed() == None)
        }
    }
}

fn earley_table<'a>(mut source: impl Iterator<Item = String>, grammar: &'a [GrammarRule]) -> EarleyTable<'a> {
    let token = source.next();
    let mut s = EarleyTable::new();
    if token == None || grammar.is_empty() { return s; }
    s.push(
        (
            token,
            grammar.iter()
            .filter(|r| r.name == grammar[0].name)
            .map(|r| EarleyItem::new(r, 0))
            .collect()
        )
    );


    for i in 0.. {
        if i >= s.len() { break; }
        if s[i].0 != None { s.push((source.next(), StateSet::new())); }
        for j in 0.. {
            if j >= s[i].1.len() { break; }
            match s[i].1[j].next_unparsed() {
                Some(Symbol::Terminal(symbol)) => scan(&mut s, symbol, i, j),
                Some(Symbol::Nonterminal(symbol)) => predict(&mut s, symbol, i, grammar),
                None => complete(&mut s, i, j),
            }
        }
    }
    s
}

fn scan(s: &mut [(Option<String>, Vec<EarleyItem>)], symbol: String, i: usize, j: usize) {
    match &s[i].0 {
        None => return,
        Some(token) => if Symbol::Terminal(symbol).matches(&token) {
            let item = s[i].1[j].advanced();
            push_unique(&mut s[i + 1].1, item);
        }
    }
}

fn predict<'a>(s: &mut [(Option<String>, Vec<EarleyItem<'a>>)], symbol: String, i: usize, grammar: &'a [GrammarRule]) {
    for rule in grammar.iter().filter(|r| r.name == symbol) {
        push_unique(&mut s[i].1, EarleyItem::new(rule, i));
    }
}

fn complete<'a>(s: &mut [(Option<String>, Vec<EarleyItem<'a>>)], i: usize, j: usize) {
    let start = s[i].1[j].start;
    let candidates = std::mem::take(&mut s[start].1);
    let name = s[i].1[j].rule.name.clone();
    for item in candidates.iter()
    .filter(|item| item.next_unparsed() == Some(Symbol::Nonterminal(name.clone()))) {
        push_unique(&mut s[i].1, item.advanced());
    }
    let _ = std::mem::replace(&mut s[start].1, candidates);
}

fn push_unique<'a>(set: &mut StateSet<'a>, item: EarleyItem<'a>) {
    if !set.contains(&item) {
        set.push(item);
    }
}

impl<'a> EarleyItem<'a> {
    pub fn new(rule: &'a GrammarRule, start: usize) -> EarleyItem {
        EarleyItem {
            rule, start, current: 0
        }
    }

    pub fn next_unparsed(&self) -> Option<Symbol> {
        self.rule.components.get(self.current).cloned()
    }

    fn advanced(&self) -> EarleyItem<'a> {
        EarleyItem { current: self.current + 1, ..*self }
    }
}