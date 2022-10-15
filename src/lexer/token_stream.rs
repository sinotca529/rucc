use super::{char_stream::CharStream, LexError, Token, TokenKind};

pub struct TokenStream<'a> {
    cs: CharStream<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cs: CharStream::new(input),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token<'a>>, LexError> {
        let mut tokens = Vec::new();

        while let Some(c) = self.cs.peek() {
            match c.value {
                // skip white space ('\n' also skipped)
                space if space.is_ascii_whitespace() => {
                    self.cs.next();
                }
                '\'' => {
                    tokens.push(self.lex_char_literal()?);
                }
                '"' => {
                    tokens.push(self.lex_string_literal()?);
                }
                '0'..='9' => {
                    tokens.push(self.lex_number()?);
                }
                c if TokenKind::is_punct(c) => {
                    tokens.push(self.lex_punct_or_punct_eq()?);
                }
                _ => {
                    tokens.push(self.lex_keyword_or_id()?);
                }
            }
        }
        Ok(tokens)
    }
}

impl<'a> TokenStream<'a> {
    fn skip_char(&mut self, c: char) -> Result<(), LexError> {
        match self.cs.next() {
            Some(g) if g.value == c => Ok(()),
            Some(g) => Err(LexError::unexpected_char(g.value, g.loc)),
            None => Err(LexError::eof(self.cs.current_loc())),
        }
    }

    fn lex_number(&mut self) -> Result<Token<'a>, LexError> {
        let loc = self.cs.current_loc();
        let num = match self.cs.peek().unwrap().value {
            '0' => {
                if self
                    .cs
                    .next_if_eq("0x")
                    .or_else(|| self.cs.next_if_eq("0X"))
                    .is_some()
                {
                    let literal = self.cs.take_while(|c| c.is_ascii_hexdigit()).value;
                    u64::from_str_radix(literal, 16).unwrap()
                } else {
                    let literal = self.cs.take_while(|c| "01234567".contains(*c)).value;
                    u64::from_str_radix(literal, 8).unwrap()
                }
            }
            _ => {
                let literal = self.cs.take_while(|c| c.is_ascii_digit()).value;
                literal.parse::<u64>().unwrap()
            }
        };

        Ok(Token::number(num, loc))
    }

    fn lex_char_literal(&mut self) -> Result<Token<'a>, LexError> {
        let loc = self.cs.current_loc();
        self.skip_char('\'')?;
        let c = self.cs.next().ok_or_else(|| LexError::eof(loc))?;

        if c.value == '\'' {
            return Err(LexError::unexpected_char(c.value, c.loc));
        }
        self.skip_char('\'')?;

        Ok(Token::char_literal(c.value, loc))
    }

    fn lex_string_literal(&mut self) -> Result<Token<'a>, LexError> {
        let loc = self.cs.current_loc();
        self.skip_char('"')?;
        let s = self.cs.take_while(|c| c != &'"');
        self.skip_char('"')?;

        Ok(Token::string_literal(s.value, loc))
    }

    fn lex_punct_or_punct_eq(&mut self) -> Result<Token<'a>, LexError> {
        let punct = self.cs.next().unwrap();

        if TokenKind::is_punct_eq_candidate(punct.value) && self.cs.next_if_eq("=").is_some() {
            return Ok(Token::punct_eq(punct.value, punct.loc));
        }

        Ok(Token::punct(punct.value, punct.loc))
    }

    fn lex_keyword_or_id(&mut self) -> Result<Token<'a>, LexError> {
        let loc = self.cs.current_loc();

        let token_kind = if self.cs.next_if_eq("return").is_some() {
            TokenKind::Return
        } else if self.cs.next_if_eq("if").is_some() {
            TokenKind::If
        } else if self.cs.next_if_eq("else").is_some() {
            TokenKind::Else
        } else if self.cs.next_if_eq("while").is_some() {
            TokenKind::While
        } else if self.cs.next_if_eq("for").is_some() {
            TokenKind::For
        } else if self.cs.next_if_eq("void").is_some() {
            TokenKind::Void
        } else if self.cs.next_if_eq("int").is_some() {
            TokenKind::Int
        } else if self.cs.next_if_eq("char").is_some() {
            TokenKind::Char
        } else if self.cs.next_if_eq("sizeof").is_some() {
            TokenKind::Sizeof
        } else if self.cs.peek().is_some() {
            let id = self
                .cs
                .take_while(|c| !c.is_whitespace() && !TokenKind::is_punct(*c));
            TokenKind::Id(id.value)
        } else {
            return Err(LexError::eof(loc));
        };

        Ok(Token {
            value: token_kind,
            loc,
        })
    }
}
