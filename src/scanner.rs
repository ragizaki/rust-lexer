use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    input: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
    pub error_found: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            input,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            error_found: false,
        }
    }

    fn parse_keyword(&self, lexeme: &str) -> Option<TokenType> {
        let token_type = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "fun" => TokenType::Fun,
            _ => return None,
        };

        Some(token_type)
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_token = Token::new(TokenType::Eof, "", Literal::Null);
        self.tokens.push(eof_token);

        self.tokens.clone()
    }

    fn add_token(&mut self, typ: TokenType, literal: Literal<'a>) {
        let text = &self.input[self.start..self.current];
        self.tokens.push(Token::new(typ, text, literal));
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::Lparen, Literal::Null),
            ')' => self.add_token(TokenType::Rparen, Literal::Null),
            '{' => self.add_token(TokenType::Lbrace, Literal::Null),
            '}' => self.add_token(TokenType::Rbrace, Literal::Null),
            ',' => self.add_token(TokenType::Comma, Literal::Null),
            '.' => self.add_token(TokenType::Dot, Literal::Null),
            '+' => self.add_token(TokenType::Plus, Literal::Null),
            '-' => self.add_token(TokenType::Minus, Literal::Null),
            ';' => self.add_token(TokenType::Semicolon, Literal::Null),
            '*' => self.add_token(TokenType::Star, Literal::Null),
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::DoubleEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, Literal::Null);
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, Literal::Null);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, Literal::Null);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, Literal::Null);
            }
            '/' => match self.match_char('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        // ignore comment
                        let _ = self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash, Literal::Null),
            },
            '"' => self.scan_string_literal(),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            other => {
                if other.is_digit(10) {
                    self.scan_number_literal();
                } else if self.valid_lox_identifier(c) {
                    self.scan_identifier();
                } else {
                    self.report_error(other);
                    self.error_found = true;
                }
            }
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn report_error(&self, text: char) {
        eprintln!("[line {}] Error: Unexpected character: {}", self.line, text);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.input.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    // looks at the next character
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.current).unwrap()
        }
    }

    // looks at the next next character
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.current + 1).unwrap()
        }
    }

    fn scan_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.error_found = true;
            return;
        }

        // The closing "
        self.advance();

        let string_literal = &self.input[self.start + 1..self.current - 1];

        self.add_token(TokenType::String, Literal::Str(string_literal));
    }

    fn scan_number_literal(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume the '.'
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let number_literal = self.input[self.start..self.current]
            .parse::<f64>()
            .expect("Invalid f64");

        self.add_token(TokenType::Number, Literal::Number(number_literal));
    }

    fn scan_identifier(&mut self) {
        while self.valid_lox_identifier(self.peek()) {
            self.advance();
        }
        let text = &self.input[self.start..self.current];

        let token_type = match self.parse_keyword(text) {
            Some(tt) => tt,
            None => TokenType::Identifier,
        };

        self.add_token(token_type, Literal::Null);
    }

    fn advance(&mut self) -> char {
        let char = self.input.chars().nth(self.current).unwrap();
        self.current += 1;

        char
    }

    /// a char in an identifier must be alphanumeric or contain an underscore in LOX
    fn valid_lox_identifier(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}
