#[derive(Debug, Copy, Clone)]
enum CharType {
    Whitespace,
    Word,
    Special
}

fn kind(character: &char) -> CharType {
    match character {
        ' ' | '\n' | '\r' => CharType::Whitespace,
        'a'..='z' | 'A'..='Z' | '0'..='9' => CharType::Word,
        _ => CharType::Special
    }
}

pub struct Lexer<T>
where T: Iterator<Item = char> {
    source: std::iter::Peekable<T>
}

impl<T: std::iter::Iterator<Item = char>> Lexer<T> {
    pub fn new(source: T) -> Lexer<T> {
        Lexer { source: source.peekable() }
    }
}

impl<T: std::iter::Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::new();
        let mut current = self.source.next();
        while let Some(c) = current {
            if push(c) {
                result.push(c);
            }
            if eject(c, self.source.peek()) {
                break;
            }
            current = self.source.next();
        }
        match result.len() {
            0 => None,
            _ => Some(result)
        }
    }
}

fn eject(c: char, peek: Option<&char>) -> bool {
    if peek == None {
        return true;
    }

    match (kind(&c), kind(peek.unwrap())) {
        (CharType::Whitespace, _) => false,
        (CharType::Word, CharType::Word) => false,
        (CharType::Word, _) => true,
        (CharType::Special, _) => true
    }
}

fn push(c: char) -> bool {
    match kind(&c) {
        CharType::Whitespace => false,
        CharType::Word => true,
        CharType::Special => true,
    }
}