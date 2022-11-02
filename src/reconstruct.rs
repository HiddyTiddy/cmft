use std::borrow::Cow;

use crate::{Token, LINE_ENDING};

pub fn reconstruct<'a, I>(tokens: I) -> String
where
    I: Iterator<Item = Token<'a>>,
{
    let mut out = String::new();
    let mut col = 0;
    let mut line = 0;
    for token in tokens {
        assert!(col <= token.location.col);
        assert!(line <= token.location.line);
        if line < token.location.line {
            out.push_str(LINE_ENDING.repeat(token.location.line - line).as_str());
            col = 0;
        }
        if col < token.location.col {
            out.push_str(" ".repeat(token.location.col - col).as_str());
        }
        col = token.location.col + token.token_type.width();
        match token.token_type.to_str() {
            Cow::Borrowed(s) => out.push_str(s),
            Cow::Owned(s) => out.push_str(s.as_str()),
        }

        line = token.location.line;
    }
    // todo!("{out:?}")
    out
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;

    use super::reconstruct;

    #[test]
    fn simple() {
        let program = "abc;ed";
        let tokenizer = Tokenizer::new(program);

        let reconstructed = reconstruct(tokenizer);
        assert_eq!(program, reconstructed);
    }

    #[test]
    fn spaces() {
        let program = "abc;\n       ed;";
        let tokenizer = Tokenizer::new(program);

        let reconstructed = reconstruct(tokenizer);
        assert_eq!(program, reconstructed);
    }
}
