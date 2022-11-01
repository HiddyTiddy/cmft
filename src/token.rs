#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpType {
    Eq,
    CmpEq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PunctType {
    Comma,
    Arrow,
    Dot,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    Keyword(&'a str),
    Operator(OpType),
    Str(&'a str),
    Const(&'a str),
    Punctuation(PunctType),
    Indentifier(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    location: Location,
}
