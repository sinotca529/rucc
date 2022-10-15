use super::{Annot, CodeFragment, CodePoint, Loc};

pub struct CharStream<'a> {
    remine: &'a str,
    current_loc: Loc,
}

impl<'a> CharStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            remine: input,
            current_loc: Loc { line: 1, column: 1 },
        }
    }

    pub fn current_loc(&self) -> Loc {
        self.current_loc
    }

    pub fn next(&mut self) -> Option<CodePoint> {
        self.take_n(1).map(|s| Annot::<char> {
            value: s.value.chars().next().unwrap(),
            loc: s.loc,
        })
    }

    pub fn next_if_eq(&mut self, s: &str) -> Option<CodeFragment<'a>> {
        if &self.remine[0..s.bytes().len()] == s {
            self.take_n(s.len())
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<CodePoint> {
        self.peek_n(1).map(|s| Annot::<char> {
            value: s.value.chars().next().unwrap(),
            loc: s.loc,
        })
    }

    pub fn peek_n(&self, len: usize) -> Option<CodeFragment<'a>> {
        if self.remine.len() < len {
            return None;
        }
        let loc = self.current_loc;
        let f = self.remine.split_at(len).0;
        Some(CodeFragment { value: f, loc })
    }

    pub fn take_n(&mut self, len: usize) -> Option<CodeFragment<'a>> {
        if self.remine.len() < len {
            return None;
        }

        let loc = self.current_loc;
        let (f, remine) = self.remine.split_at(len);
        self.remine = remine;
        for c in f.chars() {
            if c == '\n' {
                self.current_loc.line += 1;
                self.current_loc.column = 1;
            } else {
                self.current_loc.column += 1;
            }
        }

        Some(CodeFragment { value: f, loc })
    }

    pub fn take_while(&mut self, cond: impl Fn(&char) -> bool) -> CodeFragment<'a> {
        let last_idx = self
            .remine
            .char_indices()
            .take_while(|(_, c)| cond(c))
            .last()
            .map(|(i, _)| i + 1)
            .unwrap_or(0);

        self.take_n(last_idx).unwrap()
    }
}
