use std::{borrow::Cow, str::FromStr};

use crate::LINE_ENDING;

/// A Parsing Error
#[derive(Debug)]
pub struct ParseError;

/// the location of a token in the program,
/// in a way as it is seen in a text editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// the (0-indexed) line (row) of the token
    pub line: usize,
    /// the (0-indexed) column of the token
    pub col: usize,
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

/// represents an operator type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpType {
    /// the operator `=`
    Assign,
    /// the operator `+`
    Plus,
    /// the operator `-`
    Sub,
    /// the operator `*`
    Mul,
    /// the operator `/`
    Div,
    /// the operator `<<`
    Shl,
    /// the operator `>>`
    Shr,
    /// the operator `%`
    Mod,

    // assigning
    /// the operator `+=`
    PlusAssign,
    /// the operator `-=`
    SubAssign,
    /// the operator `*=`
    MulAssign,
    /// the operator `/=`
    DivAssign,
    /// the operator `<<=`
    ShlAssign,
    /// the operator `>>=`
    ShrAssign,
    /// the operator `%=`
    ModAssign,
    /// the operator `&=`
    LAndAssign,
    /// the operator `|=`
    LOrAssign,
    /// the operator `^=`
    LXorAssign,
    /// the operator `~=`
    LNotAssign,

    // logical
    /// the operator `&`
    LAnd,
    /// the operator `|`
    LOr,
    /// the operator `^`
    LXor,
    /// the operator `~`
    LNot,

    // boolean
    /// the operator `&&`
    And,
    /// the operator `||`
    Or,
    /// the operator `!`
    Not,
    /// the operator `==`
    Eq,
    /// the operator `!=`
    Neq,
}

impl FromStr for OpType {
    type Err = ParseError;

    /// maps a string to the corresponding op type or returns Err otherwise
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OpType::*;
        Ok(match s {
            "=" => Assign,
            "+" => Plus,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            "<<" => Shl,
            ">>" => Shr,
            "%" => Mod,
            "+=" => PlusAssign,
            "-=" => SubAssign,
            "*=" => MulAssign,
            "/=" => DivAssign,
            "<<=" => ShlAssign,
            ">>=" => ShrAssign,
            "%=" => ModAssign,
            "&=" => LAndAssign,
            "|=" => LOrAssign,
            "^=" => LXorAssign,
            "~=" => LNotAssign,
            "&" => LAnd,
            "|" => LOr,
            "^" => LXor,
            "~" => LNot,
            "&&" => And,
            "||" => Or,
            "!" => Not,
            "==" => Eq,
            "!=" => Neq,
            _ => return Err(ParseError),
        })
    }
}

impl OpType {
    /// maps the token to a static string
    pub fn to_str(self) -> &'static str {
        use OpType::*;
        match self {
            Assign => "=",
            Plus => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Shl => "<<",
            Shr => ">>",
            Mod => "%",
            PlusAssign => "+=",
            SubAssign => "-=",
            MulAssign => "*=",
            DivAssign => "/=",
            ShlAssign => "<<=",
            ShrAssign => ">>=",
            ModAssign => "%=",
            LAndAssign => "&=",
            LOrAssign => "|=",
            LXorAssign => "^=",
            LNotAssign => "~=",
            LAnd => "&",
            LOr => "|",
            LXor => "^",
            LNot => "~",
            And => "&&",
            Or => "||",
            Not => "!",
            Eq => "==",
            Neq => "!=",
        }
    }
}

/// Punctuation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PunctType {
    /// the punctuation ,
    Comma,
    /// the punctuation ->
    Arrow,
    /// the punctuation .
    Dot,
    /// the punctuation ;
    Semicolon,
    /// the punctuation ?
    QuestionMark,
    /// the punctuation :
    Colon,
}

impl PunctType {
    fn to_str(self) -> &'static str {
        match self {
            PunctType::Comma => ",",
            PunctType::Arrow => "->",
            PunctType::Dot => ".",
            PunctType::Semicolon => ";",
            PunctType::QuestionMark => "?",
            PunctType::Colon => ":",
        }
    }
}

/// Parentheses etc
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParenType {
    /// open parenthesis (
    LParen,
    /// close parenthesis )
    RParen,
    /// open brace {
    LBrace,
    /// close brace }
    RBrace,
    /// open bracket [
    LBrack,
    /// close bracket ]
    RBrack,
}

/// Type of a token
#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    Keyword(&'a str),
    Operator(OpType),
    /// a string literal; including the quotes
    Str(&'a str),
    /// any other literal value, characters are including the quotes
    Const(&'a str),
    Punctuation(PunctType),
    Indentifier(&'a str),
    Linebreak,
    Comment(&'a str),
}

impl<'a> TokenType<'a> {
    /// returns the number of columns this token spans
    pub fn width(&self) -> usize {
        macro_rules! strlen {
            ($s:expr) => {
                $s.chars().count()
            };
        }
        match self {
            TokenType::Keyword(kword) => strlen!(kword),
            TokenType::Operator(op) => strlen!(op.to_str()),
            TokenType::Str(lit) => strlen!(lit),
            TokenType::Const(cons) => strlen!(cons),
            TokenType::Punctuation(pt) => match pt {
                PunctType::Comma
                | PunctType::Dot
                | PunctType::Semicolon
                | PunctType::QuestionMark
                | PunctType::Colon => 1,
                PunctType::Arrow => 2,
            },
            TokenType::Indentifier(ident) => strlen!(ident),
            TokenType::Comment(com) => strlen!(com) + 2,
            TokenType::Linebreak => strlen!(LINE_ENDING),
        }
    }

    /// returns the number of rows this token spans
    pub const fn height(&self) -> usize {
        1
    }

    /// returns the length of the source text represented by the token in bytes
    pub(crate) fn len(&self) -> usize {
        match self {
            TokenType::Keyword(kword) => kword.len(),
            TokenType::Operator(op) => op.to_str().len(),
            TokenType::Str(lit) => lit.len(),
            TokenType::Const(cons) => cons.len(),
            TokenType::Punctuation(pt) => match pt {
                PunctType::Comma
                | PunctType::Dot
                | PunctType::Semicolon
                | PunctType::QuestionMark
                | PunctType::Colon => 1,
                PunctType::Arrow => 2,
            },
            TokenType::Indentifier(ident) => ident.len(),
            TokenType::Comment(com) => com.len() + 2,
            TokenType::Linebreak => LINE_ENDING.len(),
        }
    }

    pub fn to_str(self) -> Cow<'a, str> {
        match self {
            TokenType::Keyword(kw) => Cow::Borrowed(kw),
            TokenType::Operator(op) => Cow::Borrowed(op.to_str()),
            TokenType::Str(s) => Cow::Borrowed(s),
            TokenType::Const(con) => Cow::Borrowed(con),
            TokenType::Punctuation(pt) => Cow::Borrowed(pt.to_str()),
            TokenType::Indentifier(ident) => Cow::Borrowed(ident),
            TokenType::Linebreak => Cow::Borrowed(LINE_ENDING),
            TokenType::Comment(com) => Cow::Owned(format!("//{com}")),
        }
    }
}

/// a token and its location
#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub(crate) token_type: TokenType<'a>,
    pub(crate) location: Location,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, location: Location) -> Self {
        Self {
            token_type,
            location,
        }
    }
}
