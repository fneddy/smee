use core::fmt::Debug;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::{digit1, hex_digit1, space0, space1};
use nom::character::{ is_newline, is_space};
use nom::combinator::{eof, peek, recognize};
use nom::error::{Error, ParseError};
use nom::sequence::{ preceded, terminated};
use nom::IResult;

#[derive(PartialEq)]
pub enum Token<'a> {
    Comment(&'a [u8]),
    DecInt((&'a [u8],i32)),
    HexInt((&'a [u8],i32)),
    Float((&'a [u8],f32)),
    Word(&'a [u8]),
}
impl<'a> Token<'a> {
    pub fn raw(&self) -> &[u8]{
        match self {
            Token::Comment(data) => data,
            Token::DecInt((data,_)) => data,
            Token::HexInt((data,_))=> data,
            Token::Float((data,_)) => data,
            Token::Word(data) => data,
        }
    }
}

#[cfg(debug_assertions)]
impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Comment(arg0) => f.debug_tuple("Comment").field(&String::from_utf8_lossy(*arg0)).finish(),
            Self::DecInt(arg0) => f.debug_tuple("DecInt").field(arg0).finish(),
            Self::HexInt(arg0) => f.debug_tuple("HexInt").field(arg0).finish(),
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Word(arg0) => f.debug_tuple("Word").field(&String::from_utf8_lossy(*arg0)).finish(),
        }
    }
}


fn parse_comment(input: &[u8])->IResult<&[u8], Token> {
    let (rest, value) = recognize(preceded(tag(b"\\"), take_while1(|c| c != b'\n')))(input)?;
    Ok((rest,Token::Comment(value)))
}


fn parse_decint(input: &[u8])->IResult<&[u8], Token> {
    let (rest,result) =
        alt(( 
            recognize(terminated(digit1, peek(space1))),
            recognize(terminated(digit1, peek(eof)))
        ))(input)?;
    let (_,number) = nom::character::complete::i32(result)?;
    Ok((rest,Token::DecInt((result,number))))
}


fn hex_to_i32(input: &[u8]) -> IResult<&[u8],i32> {
    let mut result: i32 = 0;
    for &byte in input {
        let value = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return Err(nom::Err::Error(Error::from_char(input, byte as char))),
        };
        result = result * 16 + value as i32;
    }
    Ok((input, result))
}

fn parse_hexint(input: &[u8])->IResult<&[u8], Token> {
    let (rest, result) = 
        alt((
            recognize(terminated(preceded(alt((tag(b"0x"),tag(b"0x"))), hex_digit1),peek(space1))),
            recognize(terminated(preceded(alt((tag(b"0x"),tag(b"0x"))), hex_digit1), peek(eof)))
        ))
        (input)?;

    let (_,number) = hex_to_i32(&result[2..])?;
    Ok((rest,Token::HexInt((result,number))))
}


fn parse_word(input: &[u8])->IResult<&[u8], Token> {
    let word:IResult<&[u8], &[u8]> = take_till1( |c| is_space(c) || is_newline(c) )(input);
    word.map(|(rest, result)| 
        (rest, Token::Word(result))
    )
}


fn _parse_float(_input: &[u8])->IResult<&[u8], Token> {
    todo!()
}


pub fn parse_next(input: &[u8])->IResult<&[u8], Token> {
    preceded(
        space0,
            alt((
                parse_comment,
                parse_decint,
                parse_hexint,
                // todo _parse_float, 
                parse_word,

            )))
    (input)
}


#[cfg(test)]
mod tests {
    use super::{parse_comment, parse_decint, parse_hexint, parse_next, parse_word, Token};

    #[test]
    fn test_parse_comment() {
        // positive
        if let Ok(parsed_token) = parse_comment(b"\\ fooo comment\n ") {
            assert_eq!(parsed_token.1, Token::Comment(b"\\ fooo comment"));
            assert_eq!(parsed_token.0, b"\n ");
        }
        else {
            assert!(false);
        }

        // negative
        if let Err(error) = parse_comment(b" \\ fooo comment\n ") {
            error.map( |e|
                assert_eq!(e.input, b" \\ fooo comment\n ")
            );
        }
        else {
            assert!(false);
        }

    }

    #[test]
    fn test_parse_decint() {
        // positive
        if let Ok(parsed_token) = parse_decint(b"42") {
            assert_eq!(parsed_token.1, Token::DecInt((b"42",42)));
            assert_eq!(parsed_token.0, b"");
        }
        else {
            assert!(false);
        }
        if let Ok(parsed_token) = parse_decint(b"23 ") {
            assert_eq!(parsed_token.1, Token::DecInt((b"23",23)));
            assert_eq!(parsed_token.0, b" ");
        }
        else {
            assert!(false);
        }


        // negative
        if let Err(error) = parse_decint(b" 432 ") {
            error.map( |e|
                assert_eq!(e.input, b" 432 ")
            );
        }
        else {
            assert!(false);
        }

        // negative
        if let Err(error) = parse_decint(b"0x32") {
            error.map( |e| {
                assert_eq!(e.input, b"x32")
            }
            );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_hexint() {
        // positive
        if let Ok(parsed_token) = parse_hexint(b"0x42") {
            assert_eq!(parsed_token.1, Token::HexInt((b"0x42", 0x42)));
            assert_eq!(parsed_token.0, b"");
        }
        else {
            assert!(false);
        }
        if let Ok(parsed_token) = parse_hexint(b"0x23 ") {
            assert_eq!(parsed_token.1, Token::HexInt((b"0x23",0x23)));
            assert_eq!(parsed_token.0, b" ");
        }
        else {
            assert!(false);
        }


        // negative
        if let Err(error) = parse_hexint(b" 0x23 ") {
            error.map( |e|
                assert_eq!(e.input, b" 0x23 ")
            );
        }
        else {
            assert!(false);
        }

        // negative
        if let Err(error) = parse_hexint(b"0xk32") {
            error.map( |e| {
                assert_eq!(e.input, b"k32")
            }
            );
        }
        else {
            assert!(false);
        }


    }

    #[test]
    fn test_parse_word() {
        if let Ok((rest,word)) = parse_word(b"foo") {
            assert_eq!(word, Token::Word(b"foo"));
            assert_eq!(rest, b"");
        }
        else {
            assert!(false)
        }

        if let Ok((rest,word)) = parse_word(b"baar ") {
            assert_eq!(word, Token::Word(b"baar"));
            assert_eq!(rest, b" ");
        }
        else {
            assert!(false)
        }

        if let Ok((rest,word)) = parse_word(b"baar\n") {
            assert_eq!(word, Token::Word(b"baar"));
            assert_eq!(rest, b"\n");
        }
        else {
            assert!(false)
        }

        if let Ok((rest,word)) = parse_word(b"baz kaz ") {
            assert_eq!(word, Token::Word(b"baz"));
            assert_eq!(rest, b" kaz ");
        }
        else {
            assert!(false)
        }

        // negative
        if let Err(error) = parse_word(b" baz ") {
            error.map( |e|
                assert_eq!(e.input, b" baz ")
            );
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_next() {
        if let Ok((rest, token)) = parse_next(b" 4 5 +") {
            assert_eq!(token, Token::DecInt((b"4",4)));
            if let Ok((rest, token)) =  parse_next(rest){
                assert_eq!(token, Token::DecInt((b"5",5)));
                if let Ok((rest, token)) =  parse_next(rest){
                    assert_eq!(token, Token::Word(b"+"));
                    assert_eq!(rest, b"")
                }
                else { assert!(false); }
            }
            else { assert!(false); }
        }
        else { assert!(false); }
    }
}