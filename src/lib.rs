#![cfg_attr(not(any(debug_assertions, test)), no_std)]

extern crate alloc;
use tokenizer::token::Token;
pub mod tokenizer;

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Error<'a> {
    ConversionError,
    SyntaxError(&'a [u8])
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq)]
pub enum CellValue {
    Integer(i32),
    Float(f32),
    Ptr(usize),
    Function,
}

impl<'a> TryFrom<Token<'a>> for CellValue {
    type Error = Error<'a>;
    
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
    // as for now we use the std vec ...
    data: alloc::vec::Vec<CellValue>
}
impl Stack {
    pub fn push(&mut self, value: CellValue) {
        self.data.push(value);
    }
    pub fn pop(&mut self) -> Option<CellValue> {
        self.data.pop()
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

    pub fn eval<'a>(&mut self, input: &'a [u8]) -> Result<(),Error<'a>>{
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
                return Err(Error::SyntaxError(token.raw()));
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::Context;
    use super::CellValue;

    #[test]
    fn test_eval() {
        let mut ctx = Context::new();
        if let Err(error) = ctx.eval(b"4 5 6 5 ") {
            debug_assert!(false, "{}", format!("{:#?}", error))
        }
        assert_eq!(ctx.stack.data[0], CellValue::Integer(4));
    }
}
