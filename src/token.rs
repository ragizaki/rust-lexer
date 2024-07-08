use std::fmt;

#[derive(Clone)]
pub struct Token<'a> {
    typ: TokenType,
    lexeme: &'a str,
    literal: Literal<'a>,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{} {} {}", self.typ, self.lexeme, self.literal);
        write!(f, "{output}")
    }
}

impl<'a> Token<'a> {
    pub fn new(typ: TokenType, lexeme: &'a str, literal: Literal<'a>) -> Self {
        Token {
            typ,
            lexeme,
            literal,
        }
    }
}

#[derive(Clone)]
pub enum Literal<'a> {
    Null,
    Str(&'a str),
    Number(f64),
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Null => write!(f, "null"),
            Literal::Str(s) => write!(f, "{s}"),
            Literal::Number(num) => {
                // force whole numbers to be formatted with 1 decimal (i.e. 1234 => 1234.0)
                if num.fract() == 0.0 {
                    write!(f, "{:.1}", num)
                } else {
                    write!(f, "{num}")
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum TokenType {
    Eof,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Equal,
    DoubleEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    String,
    Number,
    Identifier,
    And,
    Class,
    Else,
    True,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            TokenType::Eof => "EOF",
            TokenType::Lparen => "LEFT_PAREN",
            TokenType::Rparen => "RIGHT_PAREN",
            TokenType::Lbrace => "LEFT_BRACE",
            TokenType::Rbrace => "RIGHT_BRACE",
            TokenType::Star => "STAR",
            TokenType::Plus => "PLUS",
            TokenType::Dot => "DOT",
            TokenType::Comma => "COMMA",
            TokenType::Minus => "MINUS",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Slash => "SLASH",
            TokenType::Equal => "EQUAL",
            TokenType::DoubleEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::If => "IF",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::For => "FOR",
            TokenType::Fun => "FUN",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",
        };

        write!(f, "{output}")
    }
}
