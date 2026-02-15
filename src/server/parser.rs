use std::mem::Discriminant;

use logos::{Lexer, Logos};

#[derive(Debug, PartialEq)]
pub struct SimpleError(pub String);

#[derive(Debug, Logos)]
#[logos(skip r"[ \t\n\r]+")]
pub(super) enum Token<'s> {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token(":")]
    Colon,
    #[token("_")]
    Underscore,
    #[token("#")]
    Hash,
    #[token(",")]
    Comma,
    #[token("*")]
    Star,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap(), priority = 3)]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap(), priority = 3)]
    Float(f64),
    #[regex(r"[a-zA-Z0-9]+", priority = 2)]
    String(&'s str),
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'s> {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(&'s str),
    Array(Vec<Value<'s>>),
}

// TODO: add support for subarray parsing
// NOTE: called after initial [ token is consumed
pub(super) fn parse_array<'s>(lexer: &mut Lexer<'s, Token<'s>>) -> Result<Value<'s>, SimpleError> {
    let mut array = Vec::new();
    let mut expected_type: Option<Discriminant<Value<'_>>> = None;
    let mut awaiting_value = true;

    let span = lexer.span();

    let mut check_add_value =
        |value: Value<'s>, awaiting_value: &mut bool| -> Result<(), SimpleError> {
            if !*awaiting_value {
                return Err(SimpleError(format!(
                    "Expected value after comma at {}..{}",
                    span.start, span.end
                )));
            }
            match expected_type {
                Some(expected_type) => {
                    if expected_type != std::mem::discriminant(&value) {
                        return Err(SimpleError(format!(
                            "Expected value of type {:?} at {}..{}",
                            expected_type, span.start, span.end
                        )));
                    }
                }
                None => {
                    expected_type = Some(std::mem::discriminant(&value));
                }
            }
            array.push(value);
            *awaiting_value = false;
            Ok(())
        };

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::LeftBracket) => {
                let value = parse_array(lexer)?;
                check_add_value(value, &mut awaiting_value)?;
            }
            Ok(Token::RightBracket) => {
                if awaiting_value && !array.is_empty() {
                    return Err(SimpleError(format!(
                        "Expected value after comma at {}..{}",
                        span.start, span.end
                    )));
                }
                return Ok(Value::Array(array));
            }
            Ok(Token::Comma) => {
                if awaiting_value {
                    return Err(SimpleError(format!(
                        "Unexpected comma at {}..{}",
                        span.start, span.end
                    )));
                }
                awaiting_value = true;
            }

            Ok(Token::Integer(i)) => check_add_value(Value::Integer(i), &mut awaiting_value)?,
            Ok(Token::Float(f)) => check_add_value(Value::Float(f), &mut awaiting_value)?,
            Ok(Token::String(s)) => check_add_value(Value::String(s), &mut awaiting_value)?,
            Ok(Token::Bool(b)) => check_add_value(Value::Bool(b), &mut awaiting_value)?,
            _ => {
                return Err(SimpleError(format!(
                    "unexpected token at {}..{} inside of array. {:?}",
                    span.start, span.end, token
                )));
            }
        }
    }

    Err(SimpleError(format!(
        "Unclosed array starting at {}..{}",
        span.start, span.end
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array() {
        let mut lexer = Token::lexer("1,2,3]");
        let result = parse_array(&mut lexer);
        assert_eq!(
            result,
            Ok(Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ]))
        );
    }
    #[test]
    fn test_parse_nested_array() {
        let mut lexer = Token::lexer("[1],[2],[3]]");
        let result = parse_array(&mut lexer);
        assert_eq!(
            result,
            Ok(Value::Array(vec![
                Value::Array(vec![Value::Integer(1)]),
                Value::Array(vec![Value::Integer(2)]),
                Value::Array(vec![Value::Integer(3)])
            ]))
        );
    }
}
