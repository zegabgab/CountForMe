#[derive(Debug, PartialEq)]
pub struct Token {
    position: usize,
    content: String,
    kind: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Unimportant,
    Float(f64),
    Int(i64),
    String(String),
}


pub struct Scanner<T: Iterator<Item = char>> {
    source: T,
}

impl<T: Iterator<Item = char>> Iterator for Scanner<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Token { position: 0, content: self.source.next()?.to_string(), kind: TokenType::Unimportant })
    }
}

