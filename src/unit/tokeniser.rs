use std::str::Chars;
use std::iter::Peekable;

pub struct Tokeniser<'a>(pub Peekable<Chars<'a>>);

impl<'a> Tokeniser<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokeniser(s.chars().peekable())
    }
}

enum Class {
    Token,
    Power,
    Symbol,
    Whitespace
}

#[inline(always)]
fn classify(c: char) -> Class {
    use self::Class::*;
    match c {
        '+'|'-'|'/'|'^'|'*'|'('|')'|'=' => Token,
        '⁻'|'⁰'|'¹'|'²'|'³'|'⁴'|'⁵'|'⁶'|'⁷'|'⁸'|'⁹' => Power,
        _ if c.is_whitespace() => Whitespace,
        _ => Symbol
    }
}

impl<'a> Iterator for Tokeniser<'a>{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        use self::Class::*;
        let c = self.0.next()?;

        match classify(c) {
            Whitespace => self.next(),
            Token => Some(c.to_string()),
            Power => {
                let mut s = c.to_string();
                while let Some(&c) = self.0.peek() {
                    match classify(c) {
                        Power => s.push(self.0.next().unwrap()),
                        _ => return Some(s)
                    }
                }
                Some(s)
            }
            Symbol => {
                let mut s = c.to_string();
                while let Some(&c) = self.0.peek() {
                    match classify(c) {
                        Token|Power => return Some(s),
                        Whitespace => {
                            self.0.next().unwrap();
                            return Some(s)
                        }
                        _ => s.push(self.0.next().unwrap())
                    }
                }
                Some(s)
            }
        }
    }
}
