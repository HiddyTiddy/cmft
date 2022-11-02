#![warn(missing_debug_implementations)]
mod reconstruct;
mod token;
mod tokenizer;

pub(crate) const LINE_ENDING: &str = "\n";

pub use token::*;
pub use tokenizer::Tokenizer;

pub fn format_string(s: String) -> String {
    let tokenizer = Tokenizer::new(&s);
    // TODO modify tokens
    reconstruct::reconstruct(tokenizer)
}
