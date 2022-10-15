use std::fmt::Display;

mod char_stream;
pub mod token_stream;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TokenKind<'a> {
    Number(u64),
    StringLiteral(&'a str),
    CharLiteral(char),
    Id(&'a str),
    /// char which is one of "!%&()*+,-/;<=>@[]{|}"
    Punct(char),
    /// !=, %=, &=, +=, -=, /=, <=, ==, >=, |=
    PunctEq(char),
    /// int
    Int,
    /// char
    Char,
    /// void
    Void,
    /// if
    If,
    /// else
    Else,
    /// for
    For,
    /// while
    While,
    /// return
    Return,
    /// sizeof
    Sizeof,
}

impl<'a> TokenKind<'a> {
    fn is_punct(c: char) -> bool {
        "!%&()*+,-/;<=>@[]{|}".contains(c)
    }

    fn is_punct_eq_candidate(c: char) -> bool {
        "!%&*+-/<=>|".contains(c)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Loc {
    line: u32,
    column: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Annot<T> {
    value: T,
    loc: Loc,
}
impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

type Token<'a> = Annot<TokenKind<'a>>;

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ln {}, col {}: {:?}",
            self.loc.line, self.loc.column, self.value
        )
    }
}

impl<'a> Token<'a> {
    fn number(num: u64, loc: Loc) -> Token<'a> {
        Self {
            value: TokenKind::Number(num),
            loc,
        }
    }

    fn string_literal(s: &'a str, loc: Loc) -> Token<'a> {
        Self {
            value: TokenKind::StringLiteral(s),
            loc,
        }
    }

    fn char_literal(c: char, loc: Loc) -> Token<'a> {
        Self {
            value: TokenKind::CharLiteral(c),
            loc,
        }
    }

    fn punct_eq(c: char, loc: Loc) -> Token<'a> {
        Token {
            value: TokenKind::PunctEq(c),
            loc,
        }
    }

    fn punct(c: char, loc: Loc) -> Token<'a> {
        Token {
            value: TokenKind::Punct(c),
            loc,
        }
    }
}

type CodePoint = Annot<char>;
type CodeFragment<'a> = Annot<&'a str>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;
impl LexError {
    fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }

    fn unexpected_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::UnexpectedChar(c), loc)
    }
}
