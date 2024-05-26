use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source_str: &'a str) -> Self {
        let source = source_str.as_bytes();
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if Self::is_alpha(*c) {
            return self.identifier();
        }
        if Self::is_digit(*c) {
            return self.number();
        }

        match c {
            b'(' => return self.make_token(TokenType::LeftParen),
            b')' => return self.make_token(TokenType::RightParen),
            b'{' => return self.make_token(TokenType::LeftBrace),
            b'}' => return self.make_token(TokenType::RightBrace),
            b';' => return self.make_token(TokenType::Semicolon),
            b',' => return self.make_token(TokenType::Comma),
            b'.' => return self.make_token(TokenType::Dot),
            b'-' => return self.make_token(TokenType::Minus),
            b'+' => return self.make_token(TokenType::Plus),
            b'/' => return self.make_token(TokenType::Slash),
            b'*' => return self.make_token(TokenType::Star),

            b'!' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }

            b'=' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }

            b'<' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }

            b'>' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }

            b'"' => {
                return self.string();
            }

            _ => {}
        }

        self.error_token("Unexpected character.")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.start,
            self.current - self.start,
            &self.source[self.start..self.current],
            self.line,
        )
    }

    fn error_token(&self, message: &'a str) -> Token {
        // What to do with this? How to store the error message?
        Token::new(
            TokenType::Error,
            0,
            message.len(),
            message.as_bytes(),
            self.line,
        )
    }

    fn advance(&mut self) -> &u8 {
        let c = &self.source[self.current];
        self.current += 1;

        c
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        self.source[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }

        self.source[self.current + 1]
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            match c {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }

                b'\n' => {
                    self.line += 1;
                    self.advance();
                }

                b'/' => {
                    if self.peek_next() == b'/' {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }

                _ => return,
            }
        }
    }

    fn identifier_type(&self) -> TokenType {
        return match &self.source[self.start] {
            b'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            b'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            b'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            b'f' => {
                if self.current - self.start > 1 {
                    match &self.source[self.start + 1] {
                        b'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        b'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        b'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            b'i' => self.check_keyword(1, 1, "f", TokenType::If),
            b'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            b'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            b'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            b'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            b's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            b't' => {
                if self.current - self.start > 1 {
                    match &self.source[self.start + 1] {
                        b'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        b'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            b'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            b'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        };
    }

    fn is_alpha(c: u8) -> bool {
        return (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_';
    }
    fn is_digit(c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && &self.source[self.start + start..self.start + start + length] == rest.as_bytes()
        {
            return token_type;
        }

        TokenType::Identifier
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // Consume the closing ".
        self.advance();
        return self.make_token(TokenType::String);
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }
}
