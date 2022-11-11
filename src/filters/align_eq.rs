use crate::Token;

// super lazy implementation, change later
pub fn align_eq(mut tokens: &mut [Token]) {
    let mut lines = vec![];

    while !tokens.is_empty() {
        let tmp;
        (tmp, tokens) = tokens.split_at_mut(1);

        lines.push(tmp);
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::Tokenizer;

    #[test]
    fn hi() {
        let program = "abc;\n       ed;";
        let tokenizer = Tokenizer::new(program);



    }
}

