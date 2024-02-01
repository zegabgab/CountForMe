use super::{GrammarRule, Symbol};

struct EarleyItem {
    rule: GrammarRule,
    start: usize,
    current: usize,
}

type StateSet = Vec<EarleyItem>;
type EarleyTable = Vec<(Option<String>, StateSet)>;

pub fn earley_recognize(source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> bool {
    let s = earley_table(source, grammar);
    false
}

fn earley_table(mut source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> EarleyTable {
    let mut token = source.next();
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
                Some(Symbol::Terminal(symbol)) => scan(&mut s),
                Some(Symbol::Nonterminal(symbol)) => predict(&mut s),
                None => complete(&mut s),
            }
        }
    }
    s
}

fn complete(s: &[(Option<String>, Vec<EarleyItem>)]) {
    todo!()
}

fn predict(s: &[(Option<String>, Vec<EarleyItem>)]) {
    todo!()
}

fn scan(s: &[(Option<String>, Vec<EarleyItem>)]) {
    todo!()
}

impl EarleyItem {
    pub fn new(rule: &GrammarRule, start: usize) -> EarleyItem {
        EarleyItem {
            rule: rule.clone(), start, current: 0
        }
    }

    pub fn next_unparsed(&self) -> Option<Symbol> {
        self.rule.components.get(self.current).cloned()
    }

    fn advanced(&self) -> EarleyItem {
        EarleyItem { rule: self.rule.clone(), current: self.current + 1, ..*self }
    }
}