#![cfg_attr(not(any(debug_assertions, test)), no_std)]

use tokenizer::token::Token;

pub mod tokenizer;

pub enum Error {
    ConversionError
}

pub enum CellValue {
    Integer(i32),
    Float(f32),
    Ptr(usize),
    Function,
}

impl<'a> TryFrom<Token<'a>> for CellValue {
    type Error = Error;
    
    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Comment(_) => Err(Error::ConversionError),
            Token::DecInt(value) => {Ok(CellValue::Integer(value.1))},
            Token::HexInt(value) => {Ok(CellValue::Integer(value.1))},
            Token::Float(value) => {Ok(CellValue::Float(value.1))},
            Token::Word(_) => Err(Error::ConversionError),
        }
    }
}

#[derive(Default)]
pub struct Stack {

}
impl Stack {
    pub fn push(&mut self, _number: CellValue) {
        todo!()
    }
}


pub enum DictionaryEntry {}

#[derive(Default)]
pub struct Dictionary {

}
impl Dictionary {
    pub fn lookup<'a>(&self, _name: &'a [u8]) -> Option<DictionaryEntry> {
        None
    }
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
        let mut tokenstream = tokenizer::tokenstream::TokenStream::new(input).peekable();
        
        while let Some(token) =  tokenstream.next(){
            
            /* ignore comments */
            if let tokenizer::token::Token::Comment(_) = token {
                continue;
            }
            /* evaluate words */
            else if let Some(_entry) = self.dictionary.lookup(token.raw()) {

            }
            /* push numners on stack */
            else if let Ok(number) = CellValue::try_from(token) {
                self.stack.push(number);
            } 
            /* this is a syntax error */
            else {

            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::Context;

    #[test]
    fn test_eval() {
        let mut ctx = Context::new();
        ctx.eval(b"4 5 + 6 5 - + ");
    }
}
