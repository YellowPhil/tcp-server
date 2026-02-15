use logos::Logos;

pub struct SimpleError(pub String);

#[derive(Debug, Logos)]
#[logos(skip r"[ \t\n\r]+")]
pub enum Token<'s> {
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
    #[regex(r"[0-9]+", priority = 2)]
    Integer,
    #[regex(r"[0-9]+\.[0-9]+", priority = 2)]
    Float,
    #[regex(r"[a-zA-Z0-9]+", priority = 3)]
    String(&'s str),
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),
}
