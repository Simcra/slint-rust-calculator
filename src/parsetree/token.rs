pub trait FromToken: Sized {
    type Err;
    fn from_token(token: &Token) -> Result<Self, Self::Err>;
}

pub trait Precedence {
    fn precedence(&self) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    LeftParen,
    RightParen,
    Modulo,
    Pi,
    Divide,
    SquareRoot,
    Multiply,
    Subtract,
    Power,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseTokenError;

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(value) => value.to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Modulo => "%".to_string(),
            Token::Pi => "π".to_string(),
            Token::Divide => "/".to_string(),
            Token::SquareRoot => "√".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Subtract => "-".to_string(),
            Token::Power => "^".to_string(),
            Token::Add => "+".to_string(),
        }
    }
}

impl From<Token> for String {
    fn from(value: Token) -> Self {
        value.to_string()
    }
}

impl Precedence for Token {
    fn precedence(&self) -> u8 {
        match self {
            Token::LeftParen => 6,
            Token::RightParen => 6,
            Token::Modulo => 5,
            Token::Divide => 4,
            Token::SquareRoot => 5,
            Token::Multiply => 3,
            Token::Subtract => 1,
            Token::Power => 5,
            Token::Add => 2,
            _ => 0,
        }
    }
}
