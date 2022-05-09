use crate::error::LoxError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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

    // One Or Two Character Tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier(String),
    QuotedString(String),
    Number(f64),

    // keywords
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

    Eof,
}

impl From<String> for TokenType {
    fn from(other: String) -> TokenType {
        TokenType::Identifier(other)
    }
}

impl<'a> From<&'a str> for TokenType {
    fn from(other: &'a str) -> TokenType {
        TokenType::Identifier(other.to_string())
    }
}

impl From<f64> for TokenType {
    fn from(other: f64) -> TokenType {
        TokenType::Number(other)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // literals
            TokenType::Identifier(ident) => write!(f, "{}", ident),
            TokenType::QuotedString(strng) => write!(f, "\"{}\"", strng),
            TokenType::Number(num) => write!(f, "{}", num),

            // Single-character tokens
            TokenType::LeftParen => write!(f, "{}", "("),
            TokenType::RightParen => write!(f, "{}", ")"),
            TokenType::LeftBrace => write!(f, "{}", "{"),
            TokenType::RightBrace => write!(f, "{}", "}"),
            TokenType::Comma => write!(f, "{}", ","),
            TokenType::Dot => write!(f, "{}", "."),
            TokenType::Minus => write!(f, "{}", "-"),
            TokenType::Plus => write!(f, "{}", "+"),
            TokenType::Semicolon => write!(f, "{}", ";"),
            TokenType::Slash => write!(f, "{}", "/"),
            TokenType::Star => write!(f, "{}", "*"),

            // One Or Two Character Tokens
            TokenType::Bang => write!(f, "{}", "!"),
            TokenType::BangEqual => write!(f, "{}", "!="),
            TokenType::Equal => write!(f, "{}", "="),
            TokenType::EqualEqual => write!(f, "{}", "=="),
            TokenType::Greater => write!(f, "{}", ">"),
            TokenType::GreaterEqual => write!(f, "{}", ">="),
            TokenType::Less => write!(f, "{}", "<"),
            TokenType::LessEqual => write!(f, "{}", "<="),

            // keywords
            TokenType::And => write!(f, "{}", "and"),
            TokenType::Class => write!(f, "{}", "class"),
            TokenType::Else => write!(f, "{}", "else"),
            TokenType::False => write!(f, "{}", "false"),
            TokenType::Fun => write!(f, "{}", "fun"),
            TokenType::For => write!(f, "{}", "for"),
            TokenType::If => write!(f, "{}", "if"),
            TokenType::Nil => write!(f, "{}", "nil"),
            TokenType::Or => write!(f, "{}", "or"),
            TokenType::Print => write!(f, "{}", "print"),
            TokenType::Return => write!(f, "{}", "return"),
            TokenType::Super => write!(f, "{}", "super"),
            TokenType::This => write!(f, "{}", "this"),
            TokenType::True => write!(f, "{}", "true"),
            TokenType::Var => write!(f, "{}", "var"),
            TokenType::While => write!(f, "{}", "while"),

            TokenType::Eof => write!(f, "{}", "EOF"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(typ: TokenType, line: usize) -> Self {
        Token { typ, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.typ)
    }
}

#[derive(Clone)]
pub struct Scanner {
    text: Vec<char>,
    current_index: usize,
    line: usize,
    has_error: bool,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Scanner {
            text: input.chars().collect::<Vec<char>>(),
            current_index: 0,
            line: 1,
            has_error: false,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        loop {
            match self.scan_token() {
                Err(msg) => loxerr!(msg),
                Ok(tok) => {
                    if tok.typ == TokenType::Eof {
                        break;
                    } else {
                        self.tokens.push(tok);
                    }
                }
            }
        }
        self.tokens.push(Token {
            typ: TokenType::Eof,
            line: self.line,
        });
        if self.has_error {
            loxerr!("{}", "Invalid input.")
        } else {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<Token, LoxError> {
        self.skip_whitespace();
        let line = self.line;
        let c = self.advance();
        match c {
            None => Ok(Token::new(TokenType::Eof, line)),
            Some(c) => match c {
                '(' => Ok(Token::new(TokenType::LeftParen, line)),
                ')' => Ok(Token::new(TokenType::RightParen, line)),
                '{' => Ok(Token::new(TokenType::LeftBrace, line)),
                '}' => Ok(Token::new(TokenType::RightBrace, line)),
                ',' => Ok(Token::new(TokenType::Comma, line)),
                '.' => Ok(Token::new(TokenType::Dot, line)),
                '-' => Ok(Token::new(TokenType::Minus, line)),
                '+' => Ok(Token::new(TokenType::Plus, line)),
                ';' => Ok(Token::new(TokenType::Semicolon, line)),
                '*' => Ok(Token::new(TokenType::Star, line)),
                '!' => {
                    if self.match_advance('=') {
                        Ok(Token::new(TokenType::BangEqual, line))
                    } else {
                        Ok(Token::new(TokenType::Bang, line))
                    }
                }
                '=' => {
                    if self.match_advance('=') {
                        Ok(Token::new(TokenType::EqualEqual, line))
                    } else {
                        Ok(Token::new(TokenType::Equal, line))
                    }
                }
                '<' => {
                    if self.match_advance('=') {
                        Ok(Token::new(TokenType::LessEqual, line))
                    } else {
                        Ok(Token::new(TokenType::Less, line))
                    }
                }
                '>' => {
                    if self.match_advance('=') {
                        Ok(Token::new(TokenType::GreaterEqual, line))
                    } else {
                        Ok(Token::new(TokenType::Greater, line))
                    }
                }
                '/' => {
                    if self.match_advance('/') {
                        self.advance_line();
                        self.scan_token()
                    } else {
                        Ok(Token::new(TokenType::Slash, line))
                    }
                }
                '"' => {
                    self.current_index -= 1;
                    match self.scan_quoted_string() {
                        Err(msg) => loxerr!(msg),
                        Ok(toktype) => {
                            if let TokenType::QuotedString(the_string) = toktype {
                                Ok(Token::new(TokenType::QuotedString(the_string), line))
                            } else {
                                loxerr!("Something bad happened: {:?}.", toktype)
                            }
                        }
                    }
                }
                _ => {
                    if c.is_alphabetic() || c == '_' {
                        self.current_index -= 1;
                        match self.scan_identifier() {
                            Err(msg) => loxerr!(msg),
                            Ok(toktype) => Ok(Token::new(toktype, line)),
                        }
                    } else if c.is_numeric() {
                        self.current_index -= 1;
                        match self.scan_number() {
                            Err(msg) => loxerr!(msg),
                            Ok(toktype) => Ok(Token::new(toktype, line)),
                        }
                    } else {
                        self.has_error = true;
                        loxerr!("Invalid character on line {}: {}", self.line, c);
                    }
                }
            },
        }
    }

    fn scan_number(&mut self) -> Result<TokenType, LoxError> {
        if let Some(strng) = self.advance_fn(|ch, _prev| ch == '.' || ch.is_digit(10)) {
            match strng.parse::<f64>() {
                Ok(num) => Ok(TokenType::Number(num)),
                Err(msg) => loxerr!(msg),
            }
        } else {
            loxerr!("Expected number but didn't find one.")
        }
    }

    fn scan_identifier(&mut self) -> Result<TokenType, LoxError> {
        if let Some(strng) = self.advance_fn(|ch, _prev| ch == '_' || ch.is_alphanumeric()) {
            match strng.as_str() {
                "and" => Ok(TokenType::And),
                "class" => Ok(TokenType::Class),
                "else" => Ok(TokenType::Else),
                "false" => Ok(TokenType::False),
                "for" => Ok(TokenType::For),
                "fun" => Ok(TokenType::Fun),
                "if" => Ok(TokenType::If),
                "nil" => Ok(TokenType::Nil),
                "or" => Ok(TokenType::Or),
                "print" => Ok(TokenType::Print),
                "return" => Ok(TokenType::Return),
                "super" => Ok(TokenType::Super),
                "this" => Ok(TokenType::This),
                "true" => Ok(TokenType::True),
                "var" => Ok(TokenType::Var),
                "while" => Ok(TokenType::While),
                _ => Ok(TokenType::Identifier(strng.to_string())),
            }
        } else {
            loxerr!("Expected identifer but did not find one.")
        }
    }

    fn scan_quoted_string(&mut self) -> Result<TokenType, LoxError> {
        if !self.match_advance('"') {
            loxerr!("Expected quoted string to start with \"");
        }
        let beginning_index = self.current_index;
        let beginning_line = self.line;
        let poss = self.advance_fn(|ch, prev| !(ch == '"' && prev != '\\'));
        if let Some(strng) = poss {
            if !self.match_advance('"') {
                loxerr!(
                    "Unterminated quoted string starting on line {}: \"{}",
                    beginning_line,
                    self.text[beginning_index..beginning_index + 10]
                        .into_iter()
                        .collect::<String>()
                        .to_string()
                );
            } else {
                Ok(TokenType::QuotedString(strng))
            }
        } else {
            loxerr!("We had trouble parsing a quoted string.")
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.text.len()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.text[self.current_index])
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.current_index += 1;
        if let Some(c) = c {
            if c == '\n' {
                self.line += 1;
            }
        }
        c
    }

    fn advance_fn<F>(&mut self, fnc: F) -> Option<String>
    where
        F: Fn(char, char) -> bool,
    {
        let mut buffer = String::new();
        let mut prev = '\0';
        while fnc(self.peek().unwrap_or('\0'), prev) {
            let c = self.advance().unwrap();
            prev = c;
            buffer.push(c);
        }
        Some(buffer.to_string())
    }

    fn advance_line(&mut self) {
        while self.peek() != Some('\n') {
            self.current_index += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().unwrap().is_whitespace() {
            let _c = self.advance();
        }
    }

    fn match_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.text[self.current_index] != expected {
            false
        } else {
            self.current_index += 1;
            true
        }
    }
}

macro_rules! scanner_test_tokens {
    ( $name:ident, $src:expr, $( $toktyp:expr ),+ ) => {
        #[cfg(test)]
        #[test]
        fn $name() -> Result<(), LoxError> {
            let src: &str = $src;
            let mut typs = Vec::new();
            $(
                eprintln!("E TokenType:: {:?}", $toktyp);
                typs.push($toktyp);
            )+
            let mut scanner = Scanner::new(&String::from(src));
            let tokens = scanner.scan_tokens()?;
            assert_eq!(typs.len(), tokens.len() );
            for i in 0..tokens.len() {
                eprintln!("O TokenType:: {:?}", tokens[i].typ);
                assert_eq!(typs[i], tokens[i].typ);
            }
            Ok(())
        }
    };
}

scanner_test_tokens!(
    scan_one_char_tokens,
    "(){},.-+;/*",
    TokenType::LeftParen,
    TokenType::RightParen,
    TokenType::LeftBrace,
    TokenType::RightBrace,
    TokenType::Comma,
    TokenType::Dot,
    TokenType::Minus,
    TokenType::Plus,
    TokenType::Semicolon,
    TokenType::Slash,
    TokenType::Star,
    TokenType::Eof
);

scanner_test_tokens!(
    scan_two_char_tokens,
    "! != == = < <= > >= /",
    TokenType::Bang,
    TokenType::BangEqual,
    TokenType::EqualEqual,
    TokenType::Equal,
    TokenType::Less,
    TokenType::LessEqual,
    TokenType::Greater,
    TokenType::GreaterEqual,
    TokenType::Slash,
    TokenType::Eof
);

scanner_test_tokens!(
    scan_keywords,
    "and class else false fun for if nil or print return super this true var while",
    TokenType::And,
    TokenType::Class,
    TokenType::Else,
    TokenType::False,
    TokenType::Fun,
    TokenType::For,
    TokenType::If,
    TokenType::Nil,
    TokenType::Or,
    TokenType::Print,
    TokenType::Return,
    TokenType::Super,
    TokenType::This,
    TokenType::True,
    TokenType::Var,
    TokenType::While,
    TokenType::Eof
);

scanner_test_tokens!(
    scan_comment,
    "// comment\n(",
    TokenType::LeftParen,
    TokenType::Eof
);

scanner_test_tokens! {
    scan_quoted,
    "\"hello\" \"world\"",
    TokenType::QuotedString("hello".to_string()),
    TokenType::QuotedString("world".to_string()),
    TokenType::Eof
}

scanner_test_tokens!(
    scan_abc_and_12,
    "abc 12;",
    TokenType::Identifier("abc".to_string()),
    TokenType::Number(12.0),
    TokenType::Semicolon,
    TokenType::Eof
);

scanner_test_tokens!(
    test_scan_quoted_string,
    "myvar = \"round bear\";",
    TokenType::Identifier("myvar".to_string()),
    TokenType::Equal,
    TokenType::QuotedString("round bear".to_string()),
    TokenType::Semicolon,
    TokenType::Eof
);

scanner_test_tokens!(
    test_single_number,
    "12",
    TokenType::Number(12.0),
    TokenType::Eof
);
