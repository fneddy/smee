use super::token::*;

pub struct TokenStream<'a> {
    data: &'a [u8],
    rest: &'a [u8]
}
impl<'a> TokenStream<'a> {
    pub fn new(data: &'a [u8]) -> TokenStream<'a>{
        TokenStream {
            data: data,
            rest: data
        }
    }
    pub fn reset(&mut self) {
        self.rest = self.data;
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok((rest, token)) = parse_next(&self.rest) {
            self.rest = rest;
            return Some(token);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::token::Token;

    use super::TokenStream;

    #[test]
    fn test_tokenstream_iterator() {
        let mut ts = TokenStream::new(b"4 5 + ");

        assert_eq!(ts.next(), Some(Token::DecInt((b"4",4))));
        assert_eq!(ts.next(), Some(Token::DecInt((b"5",5))));
        assert_eq!(ts.next(), Some(Token::Word(b"+")));
        assert_eq!(ts.next(), None);
        ts.reset();
        assert_eq!(ts.next(), Some(Token::DecInt((b"4",4))));
        assert_eq!(ts.next(), Some(Token::DecInt((b"5",5))));
        assert_eq!(ts.next(), Some(Token::Word(b"+")));
        assert_eq!(ts.next(), None);
    }
}
