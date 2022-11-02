use crate::{Location, OpType, PunctType, Token, TokenType};

#[derive(Debug)]
pub struct Tokenizer<'a> {
    data: &'a str,
    col: usize,
    row: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            data,
            col: 0,
            row: 0,
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn delimiter_single_char<'b: 'a, 'c: 'a>(
        &mut self,
        delimiter: char,
        corresponds: TokenType<'b>,
        before: &'c str,
    ) -> Token<'a> {
        let (row, col) = (self.row, self.col);
        let loc = Location::new(row, col);

        if before.is_empty() {
            self.col += delimiter.len_utf8();
            self.data = &self.data[delimiter.len_utf8()..];
            Token::new(corresponds, loc)
        } else {
            self.data = &self.data[before.len()..];
            self.col += before.chars().count();
            // fixme, this shouldnt be an ident but rather something that is decided depending on
            // the content of the `before` variable
            Token::new(TokenType::Indentifier(before), loc)
        }
    }

    // two chars and not in general because dont need it
    fn delimiter_two_chars<'b, 'c, F>(
        &mut self,
        ch: char,
        delimiter: F,
        before: &'c str,
    ) -> Token<'a>
    where
        'c: 'a,
        'b: 'a,
        F: Fn(char) -> TokenType<'b>,
    {
        let loc = Location::new(self.row, self.col);

        let mut iter = self.data[before.len() + ch.len_utf8()..].chars();
        let ch_after = iter.next();
        let corresponds = delimiter(ch_after.unwrap_or('\0')); // just dont have a string
                                                               // with a NUL character in it what could go wrong
        let len = corresponds.width();

        if before.is_empty() {
            self.col += len;
            self.data = &self.data[corresponds.len()..];
            Token::new(corresponds, loc)
        } else {
            self.data = &self.data[before.len()..];
            self.col += before.chars().count();
            // fixme, this shouldnt be an ident but rather something that is decided depending on
            // the content of the `before` variable
            Token::new(TokenType::Indentifier(before), loc)
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.data.char_indices();
        while let Some((i, ch)) = iter.next() {
            let before = &self.data[..i];

            match ch {
                ';' => {
                    return Some(self.delimiter_single_char(
                        ';',
                        TokenType::Punctuation(PunctType::Semicolon),
                        before,
                    ))
                }
                ',' => {
                    return Some(self.delimiter_single_char(
                        ',',
                        TokenType::Punctuation(PunctType::Comma),
                        before,
                    ))
                }
                '.' => {
                    return Some(self.delimiter_single_char(
                        '.',
                        TokenType::Punctuation(PunctType::Dot),
                        before,
                    ))
                }
                '-' => {
                    return Some(self.delimiter_two_chars(
                        '-',
                        |b| match b {
                            '>' => TokenType::Punctuation(PunctType::Arrow),
                            '=' => TokenType::Operator(OpType::SubAssign),
                            _ => TokenType::Operator(OpType::Sub),
                        },
                        before,
                    ))
                }
                '+' => {
                    return Some(self.delimiter_two_chars(
                        '+',
                        |b| match b {
                            '=' => TokenType::Operator(OpType::PlusAssign),
                            _ => TokenType::Operator(OpType::Plus),
                        },
                        before,
                    ))
                }
                '&' => {
                    return Some(self.delimiter_two_chars(
                        '&',
                        |b| match b {
                            '=' => TokenType::Operator(OpType::LAndAssign),
                            '&' => TokenType::Operator(OpType::And),
                            _ => TokenType::Operator(OpType::LAnd),
                        },
                        before,
                    ))
                }

                ' ' => {
                    if before.is_empty() {
                        self.col += 1;
                        self.data = &self.data[' '.len_utf8()..];
                        iter = self.data.char_indices();
                    } else {
                        let loc = Location::new(self.row, self.col);
                        self.data = &self.data[before.len() ..];
                        self.col += before.chars().count();
                        // fixme, this shouldnt be an ident but rather something that is decided depending on
                        // the content of the `before` variable
                        return Some(Token::new(TokenType::Indentifier(before), loc));
                    }

                }

                'a'..='z' | 'A'..='Z' => {}
                x => todo!("hit case {x:?}. this is a problem for future me. if you hit this you are future me."),
            }
        }

        if self.data.is_empty() {
            None
        } else {
            let out = self.data;
            self.data = "";

            // fixme, this shouldnt be an ident but rather something that is decided depending on
            // the content of the `before` variable
            Some(Token::new(
                TokenType::Indentifier(out),
                Location::new(self.row, self.col),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{OpType, PunctType, TokenType, Tokenizer};

    #[test]
    fn only_semicolons() {
        let program = ";;;;;;;";
        let tok = Tokenizer::new(program);
        let tok: Vec<_> = tok.map(|i| i.token_type).collect();

        assert_eq!(tok.len(), 7);
        for i in tok {
            assert_eq!(i, TokenType::Punctuation(crate::PunctType::Semicolon));
        }
    }

    #[test]
    fn semicolons_idents() {
        let program = "abc;def;";
        let tok = Tokenizer::new(program);

        let expected = vec![
            TokenType::Indentifier("abc"),
            TokenType::Punctuation(crate::PunctType::Semicolon),
            TokenType::Indentifier("def"),
            TokenType::Punctuation(crate::PunctType::Semicolon),
        ];

        let mut cur_ind = 0;
        for (i, j) in tok.zip(expected) {
            assert_eq!(i.token_type, j);
            assert_eq!(i.location.line, 0);
            assert_eq!(i.location.col, cur_ind);
            cur_ind += j.width();
        }
    }

    #[test]
    fn multichar() {
        let program = "ab->cd+=-";
        let expected = vec![
            TokenType::Indentifier("ab"),
            TokenType::Punctuation(PunctType::Arrow),
            TokenType::Indentifier("cd"),
            TokenType::Operator(OpType::PlusAssign),
            TokenType::Operator(OpType::Sub),
        ];
        let tok = Tokenizer::new(program);

        let mut cur_ind = 0;
        for (i, j) in tok.zip(expected) {
            assert_eq!(i.token_type, j);
            assert_eq!(i.location.line, 0);
            assert_eq!(i.location.col, cur_ind);
            cur_ind += j.width();
        }
    }
}
