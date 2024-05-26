pub enum TokenType {
    // Single-character tokens
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,

    // One or two character tokens
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    // Literals
    TokenIdentifier,
    TokenString,
    TokenNumber,

    // Keywords
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFun,
    TokenFor,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenSuper,
    TokenThis,
    TokenTrue,
    TokenVar,
    TokenWhile,

    TokenError,
    TokenEof,
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
