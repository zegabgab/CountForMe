use super::{GrammarRule, Symbol, SyntaxTree};

#[derive(PartialEq, Eq, Clone, Copy)]
struct EarleyItem<'a> {
    rule: &'a GrammarRule,
    start: usize,
    current: usize,
}

type StateSet<'a> = Vec<EarleyItem<'a>>;
type EarleyTable<'a> = Vec<(Option<String>, StateSet<'a>)>;

pub fn earley_parse(
    source: impl Iterator<Item = String>,
    grammar: &[GrammarRule],
) -> Option<SyntaxTree> {
    let table = earley_table(source, grammar);
    let target = &grammar.first()?.name;
    let table = reverse(&table);
    if recognize(&table, target) {
        construct_tree(&reverse(&table), target, 0, table.len() - 1)
    } else {
        None
    }
}

fn construct_tree(
    table: &[(Option<String>, Vec<EarleyItem<'_>>)],
    target: &str,
    from: usize,
    to: usize,
) -> Option<SyntaxTree> {
    table[from]
        .1
        .iter()
        .filter(|item| item.rule.name == target && item.start == to)
        .map(|item| from_item(table, item, from, to))
        .next()?
}

fn recognize(table: &[(Option<String>, Vec<EarleyItem<'_>>)], target: &str) -> bool {
    table
        .first()
        .unwrap_or(&(None, Vec::new()))
        .1
        .iter()
        .any(|item| item.rule.name == target && item.start == table.len() - 1)
}

fn from_item(
    table: &[(Option<String>, Vec<EarleyItem<'_>>)],
    item: &EarleyItem<'_>,
    mut from: usize,
    to: usize,
) -> Option<SyntaxTree> {
    let subdivision = subdivide(table, item, from, to)?;
    let mut vec = Vec::new();
    let name = &item.rule.name;
    for (i, &element) in subdivision.iter().enumerate() {
        let tree = match item.rule.components[i] {
            Symbol::Terminal(_) => SyntaxTree::new(table[from].0.as_ref()?),
            Symbol::Nonterminal(_) => {
                let next_item = table[from]
                    .1
                    .iter()
                    .find(|it| {
                        item.rule.components[i].matches(&it.rule.name) && it.start == element
                    })?;
                from_item(table, next_item, from, element)?
            }
        };
        from = element;
        vec.push(tree);
    }
    Some(SyntaxTree::with_children(name, vec))
}

fn subdivide(
    table: &[(Option<String>, Vec<EarleyItem<'_>>)],
    item: &EarleyItem<'_>,
    from: usize,
    to: usize,
) -> Option<Vec<usize>> {
    let mut result = subdivide_continue(table, &item.rule.components, from, to)?;
    result.reverse();
    Some(result)
}

fn subdivide_continue(
    table: &[(Option<String>, Vec<EarleyItem<'_>>)],
    symbols: &[Symbol],
    from: usize,
    to: usize,
) -> Option<Vec<usize>> {
    match (from.cmp(&to), symbols.first()) {
        (std::cmp::Ordering::Greater, _) => None,
        (std::cmp::Ordering::Less, None) => None,
        (std::cmp::Ordering::Equal, Some(_)) => None,
        (std::cmp::Ordering::Equal, None) => Some(Vec::new()),
        (std::cmp::Ordering::Less, Some(symbol)) => match symbol {
            Symbol::Terminal(_) => {
                if !symbol.matches(table[from].0.as_ref()?) {
                    return None;
                }
                let mut result = subdivide_continue(table, &symbols[1..], from + 1, to)?;
                result.push(from + 1);
                Some(result)
            }
            Symbol::Nonterminal(_) => {
                for item in table[from]
                    .1
                    .iter()
                    .filter(|i| symbol.matches(&i.rule.name))
                {
                    let result = subdivide_continue(table, &symbols[1..], item.start, to);
                    if let Some(mut vec) = result {
                        vec.push(item.start);
                        return Some(vec);
                    }
                }
                None
            }
        },
    }
}

fn earley_table(mut source: impl Iterator<Item = String>, grammar: &[GrammarRule]) -> EarleyTable {
    let token = source.next();
    let mut s = EarleyTable::new();
    if token.is_none() || grammar.is_empty() {
        return s;
    }
    s.push((
        token,
        grammar
            .iter()
            .filter(|r| r.name == grammar[0].name)
            .map(|r| EarleyItem::new(r, 0))
            .collect(),
    ));

    for i in 0.. {
        if i >= s.len() {
            break;
        }
        if s[i].0.is_some() {
            s.push((source.next(), StateSet::new()));
        }
        for j in 0.. {
            if j >= s[i].1.len() {
                break;
            }
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
        None => (),
        Some(token) => {
            if Symbol::Terminal(symbol).matches(token) {
                let item = s[i].1[j].advanced();
                push_unique(&mut s[i + 1].1, item);
            }
        }
    }
}

fn predict<'a>(
    s: &mut [(Option<String>, Vec<EarleyItem<'a>>)],
    symbol: String,
    i: usize,
    grammar: &'a [GrammarRule],
) {
    for rule in grammar.iter().filter(|r| r.name == symbol) {
        push_unique(&mut s[i].1, EarleyItem::new(rule, i));
    }
}

fn complete(s: &mut [(Option<String>, Vec<EarleyItem<'_>>)], i: usize, j: usize) {
    let start = s[i].1[j].start;
    let candidates = std::mem::take(&mut s[start].1);
    let name = s[i].1[j].rule.name.clone();
    for item in candidates
        .iter()
        .filter(|item| item.next_unparsed() == Some(Symbol::Nonterminal(name.clone())))
    {
        push_unique(&mut s[i].1, item.advanced());
    }
    let _ = std::mem::replace(&mut s[start].1, candidates);
}

fn push_unique<'a>(set: &mut StateSet<'a>, item: EarleyItem<'a>) {
    if !set.contains(&item) {
        set.push(item);
    }
}

fn reverse<'a>(table: &EarleyTable<'a>) -> EarleyTable<'a> {
    let mut result = vec![(None, Vec::new()); table.len()];
    for (i, set) in table.iter().enumerate() {
        result[i].0 = set.0.clone();
        for item in &set.1 {
            if item.next_unparsed().is_none() {
                result[item.start].1.push(EarleyItem { start: i, ..*item })
            }
        }
    }
    for (_, set) in result.iter_mut() {
        set.sort_by(|a, b| a.start.cmp(&b.start));
    }
    result
}

impl<'a> EarleyItem<'a> {
    pub fn new(rule: &'a GrammarRule, start: usize) -> EarleyItem {
        EarleyItem {
            rule,
            start,
            current: 0,
        }
    }

    pub fn next_unparsed(&self) -> Option<Symbol> {
        self.rule.components.get(self.current).cloned()
    }

    fn advanced(&self) -> EarleyItem<'a> {
        EarleyItem {
            current: self.current + 1,
            ..*self
        }
    }
}
