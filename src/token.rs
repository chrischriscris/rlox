pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

pub struct Token<'a> {
    token_type: TokenType,

    // Think about replacing this with a string slice
    start: usize,
    length: usize,
    lexeme: &'a [u8],

    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        start: usize,
        length: usize,
        lexeme: &'a [u8],
        line: usize,
    ) -> Self {
        Token {
            token_type,
            start,
            length,
            lexeme,
            line,
        }
    }
}
