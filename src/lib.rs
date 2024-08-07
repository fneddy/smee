#![cfg_attr(not(any(debug_assertions, test)), no_std)]

pub mod tokenizer;


#[derive(Default)]
pub struct Stack {

}

#[derive(Default)]
pub struct Dictionary {

}

#[derive(Default)]
pub struct Context {
    stack: Stack,
    dictionary: Dictionary,
}
impl Context {
    pub fn new() -> Context {
        Context::default()
    }

    pub fn eval(&mut self, input: &[u8]) {
        let mut tokenstream = tokenizer::tokenstream::TokenStream::new(input);
        while let Some(token) =  tokenstream.next(){
            println!("{:#?}", token);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenstream;

    use super::Context;

    #[test]
    fn test_eval() {
        let mut ctx = Context::new();
        ctx.eval(b"4 5 + 6 5 - + ");
    }
}
