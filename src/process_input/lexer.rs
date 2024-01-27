#[derive(Debug, Copy, Clone)]
enum CharType {
    Whitespace,
    Word,
    Number,
    Special
}

fn kind(character: &char) -> CharType {
    match character {
        ' ' | '\n' => CharType::Whitespace,
        'a'..='z' | 'A'..='Z' | '0'..='9' => CharType::Word,
        _ => CharType::Special
    }
}

pub struct Lexer<'a, T>
where T: Iterator<Item = char> {
    source: &'a mut T,
    current: char
}

impl<'a, T: std::iter::Iterator<Item = char>> Iterator for Lexer<'a, T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}