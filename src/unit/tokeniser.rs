use std::str::CharIndices;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    Numbers,
    Letters,
    Symbols,
    Nothing
}

pub struct Tokeniser<'a> {
    string: &'a str,
    chars: CharIndices<'a>,
    start_index: usize,
    cur_mode: Mode
}

impl<'a> Tokeniser<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokeniser {
            start_index: 0,
            string: s,
            chars: s.char_indices(),
            cur_mode: Mode::Nothing
        }
    }
    fn char_mode(&mut self, c: char) {
        if c.is_digit(10) {
            self.cur_mode = Mode::Numbers;
        } else if c.is_alphabetic() {
            self.cur_mode = Mode::Letters;
        } else if c.is_whitespace() {
            self.cur_mode = Mode::Nothing;
        } else {
            self.cur_mode = Mode::Symbols;
        }
    }
}

// TODO Broken tokeniser

impl<'a> Iterator for Tokeniser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let last_mode = self.cur_mode;
        let (mut i, mut next_char) = self.chars.next()?;

        self.char_mode(next_char);

        loop {
            self.char_mode(next_char);
            if let Some((j, c)) = self.chars.next() {
                i = j;
                next_char = c;
            } else {
                return Some(&self.string[self.start_index..])
            }

            if let Mode::Nothing = self.cur_mode {
                self.start_index = i;
                continue;
            }

            if last_mode != self.cur_mode {
                break;
            }
        }

        let start = self.start_index;
        self.start_index = i+next_char.len_utf8();

        Some(&self.string[start..i])
    }
}
