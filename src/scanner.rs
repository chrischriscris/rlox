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
            return self.make_token(TokenType::TokenEof);
        }

        let c = self.advance();

        if Self::is_alpha(*c) {
            return self.identifier();
        }
        if Self::is_digit(*c) {
            return self.number();
        }

        match c {
            b'(' => return self.make_token(TokenType::TokenLeftParen),
            b')' => return self.make_token(TokenType::TokenRightParen),
            b'{' => return self.make_token(TokenType::TokenLeftBrace),
            b'}' => return self.make_token(TokenType::TokenRightBrace),
            b';' => return self.make_token(TokenType::TokenSemicolon),
            b',' => return self.make_token(TokenType::TokenComma),
            b'.' => return self.make_token(TokenType::TokenDot),
            b'-' => return self.make_token(TokenType::TokenMinus),
            b'+' => return self.make_token(TokenType::TokenPlus),
            b'/' => return self.make_token(TokenType::TokenSlash),
            b'*' => return self.make_token(TokenType::TokenStar),

            b'!' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::TokenBangEqual)
                } else {
                    self.make_token(TokenType::TokenBang)
                }
            }

            b'=' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::TokenEqualEqual)
                } else {
                    self.make_token(TokenType::TokenEqual)
                }
            }

            b'<' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::TokenLessEqual)
                } else {
                    self.make_token(TokenType::TokenLess)
                }
            }

            b'>' => {
                return if self.match_char(b'=') {
                    self.make_token(TokenType::TokenGreaterEqual)
                } else {
                    self.make_token(TokenType::TokenGreater)
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
            TokenType::TokenError,
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

    fn identifier_type() -> TokenType {
        TokenType::TokenIdentifier
    }

    fn is_alpha(c: u8) -> bool {
        return (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_';
    }

    fn is_digit(c: u8) -> bool {
        c >= b'0' && c <= b'9'
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
        return self.make_token(TokenType::TokenString);
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

        self.make_token(TokenType::TokenNumber)
    }

    fn identifier(&mut self) -> Token {
        while Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(Self::identifier_type())
    }
}
