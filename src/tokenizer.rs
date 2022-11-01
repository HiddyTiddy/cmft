use crate::{Location, PunctType, Token, TokenType};

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
            Token::new(TokenType::Indentifier(before), loc)
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for (i, ch) in self.data.char_indices() {
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

                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{TokenType, Tokenizer};

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
            dbg!(cur_ind);
            dbg!(&i);
            assert_eq!(i.token_type, j);
            assert_eq!(i.location.line, 0);
            assert_eq!(i.location.col, cur_ind);
            cur_ind += j.width();
        }
    }
}
