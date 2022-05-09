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

//        write!(f, "{} {}", self.typ, self.line)
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
fn take_while<F>(
    data: &[char],
    start_index: usize,
    mut should_continue: F,
) -> Result<String, LoxError>
where
    F: FnMut(char) -> bool,
{
    let mut current_index = start_index;
    let mut buf = String::new();
    while current_index < data.len() && should_continue(data[current_index]) {
        buf.push(data[current_index]);
        current_index += 1;
    }
    Ok(buf.to_string())
}

fn scan_number(data: &[char], start_index: usize) -> Result<(TokenType, usize), LoxError> {
    if let Ok(numstr) = take_while(data, start_index, |ch| ch == '.' || ch.is_digit(10)) {
        match numstr.parse::<f64>() {
            Ok(num) => Ok((TokenType::Number(num), numstr.len())),
            Err(msg) => loxerr!(msg),
        }
    } else {
        loxerr!("Expected number but didn't find one.")
    }
}

fn scan_identifier(data: &[char], start_index: usize) -> Result<TokenType, LoxError> {
    if data[start_index] != '_' && !data[start_index].is_alphabetic() {
        loxerr!("Expected identifier, found number.")
    } else if let Ok(ident) = take_while(data, start_index, |ch| ch == '_' || ch.is_alphanumeric())
    {
        match ident.as_str() {
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
            _ => Ok(TokenType::Identifier(ident.to_string())),
        }
    } else {
        loxerr!("Expected identifer but did not find one.")
    }
}

fn scan_quoted_string(data: &[char], start_index: usize) -> Result<(TokenType, usize), LoxError> {
    let mut line_count = 0;
    let mut prev_ch = '1';
    let tok = take_while(data, start_index, |ch| {
        let result;
        if ch == '\n' {
            line_count += 1
        };
        if ch != '"' {
            result = true;
        } else if prev_ch == '\\' {
            result = true;
        } else {
            result = false;
        }
        prev_ch = ch;
        result
    });
    if let Ok(qstr) = tok {
        if start_index + qstr.len() >= data.len() || '\"' != data[start_index + qstr.len()] {
            // We didn't see a closing double-quote.
            loxerr!(
                "Missing end-quote: idx={}, len={}.",
                start_index + qstr.len(),
                data.len()
            )
        }
        Ok((TokenType::QuotedString(qstr), line_count))
    } else {
        loxerr!("Expected quoted string, did not find one: \"{:?}\".", tok)
    }
}

macro_rules! scanner_test {
    (FAIL: $name:ident, $func:ident, $src:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let func = $func;
            let got = func(&src.chars().collect::<Vec<char>>(), 0);
            assert!(got.is_err(), "{:?} should be an error", got);
        }
    };
    (FROM: $name:ident, $func:ident, $src:expr => ($should_be:expr, $wid:expr)) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let exp_str = $should_be;
            let should_be = TokenType::from(exp_str);
            let func = $func;
            let got = func(&src.chars().collect::<Vec<char>>(), 0).unwrap();
            assert_eq!(got, (should_be, $wid), "Input was {:?}", src);
        }
    };
    (FROM: $name:ident, $func:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let exp_str = $should_be;
            let should_be = TokenType::from(exp_str);
            let func = $func;
            let got = func(&src.chars().collect::<Vec<char>>(), 0).unwrap();
            assert_eq!(got, should_be, "Input was {:?}", src);
        }
    };
    ($name:ident, $func:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let exp_str = $should_be;
            let func = $func;
            let got = func(&src.chars().collect::<Vec<char>>(), 0).unwrap();
            assert_eq!(got, exp_str, "Input was {:?}", src);
        }
    };
}

scanner_test!(FROM: scan_a_single_letter, scan_identifier, "F" => "F");
scanner_test!(FROM: scan_an_identifier, scan_identifier, "Foo" => "Foo");
scanner_test!(FROM: scan_identifier_containing_underscore, scan_identifier, "foo_bar" => "foo_bar");
scanner_test!(
    FAIL: scan_ident_cant_start_with_number,
    scan_identifier,
    "7foo_bar"
);
scanner_test!(
    FAIL: scan_ident_cant_start_with_dot,
    scan_identifier,
    ".foo_bar"
);

scanner_test!(scan_qstring_full, scan_quoted_string, "hello\"" => (TokenType::QuotedString(String::from("hello")),0));
scanner_test!(FAIL: scan_qstring_partial, scan_quoted_string, "hello");
scanner_test!(scan_empty_string, scan_quoted_string, "\"" => (TokenType::QuotedString(String::from("")),0));
scanner_test!(scan_multiline_string,
              scan_quoted_string,
              "a\nb\nc\"" => (TokenType::QuotedString(String::from("a\nb\nc")), 2)
);

scanner_test!(FROM: scan_number_integer, scan_number, "1234" => (1234.0, 4));
scanner_test!(FROM: scan_number_float, scan_number, "1234.5" => (1234.5, 6));
scanner_test!(FAIL: scan_number_two_dots, scan_number, "1234.5.6");
scanner_test!(FROM: scan_number_float_alpha, scan_number, "1234.5ab" => (1234.5, 6));

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
                '"' => match scan_quoted_string(&self.text, self.current_index) {
                    Err(msg) => loxerr!(msg),
                    Ok(toktype) => {
                        if let (TokenType::QuotedString(the_string), line_count) = toktype {
                            self.current_index += the_string.len() + 1;
                            self.line += line_count;
                            Ok(Token::new(TokenType::QuotedString(the_string), line))
                        } else {
                            loxerr!("Something bad happened: {:?}.", toktype)
                        }
                    }
                },
                _ => {
                    if c.is_alphabetic() || c == '_' {
                        match scan_identifier(&self.text, self.current_index - 1) {
                            Err(msg) => loxerr!(msg),
                            Ok(toktype) => {
                                self.current_index += format!("{}", toktype).len() - 1;
                                Ok(Token::new(toktype, line))
                                // if let TokenType::Identifier(the_string) = toktype {
                                //     self.current_index += the_string.len() - 1;
                                //     Ok(Token::new(TokenType::Identifier(the_string), line))
                                // } else {
                                //     loxerr!("Something bad happened getting an identifier: {:?}", toktype)
                                // }
                            }
                        }
                    } else if c.is_numeric() {
                        match scan_number(&self.text, self.current_index - 1) {
                            Err(msg) => loxerr!(msg),
                            Ok((toktype, wid)) => {
                                eprintln!("TOK {} at {}", toktype, wid);
                                //if let TokenType::Number(num) = toktype {
                                self.current_index += wid - 1;
                                Ok(Token::new(toktype, line))
                                //} else {
                                //    loxerr!("Something bad happened")
                                //}
                            }
                        }
                    } else {
                        self.has_error = true;
                        loxerr!("Invalid character on line {}: {}", self.line, c);
                    }
                }
            },
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
        c
    }

    fn advance_line(&mut self) {
        while self.peek() != Some('\n') {
            self.current_index += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().unwrap().is_whitespace() {
            let c = self.advance();
            if let Some(c) = c {
                if c == '\n' {
                    self.line += 1;
                }
            }
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
    scan_comment,
    "// comment\n(",
    TokenType::LeftParen,
    TokenType::Eof
);

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
