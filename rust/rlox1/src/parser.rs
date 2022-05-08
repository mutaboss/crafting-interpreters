use crate::error::LoxError;
use crate::scanner::*;

#[derive(Debug,PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    False,
    True,
    Nil,
    Literal(Token),
    Identifier(Token),
    Grouping(Box<Expr>),
}

pub struct LoxParser {
    tokens: Vec<Token>,
    current: usize,
}

impl LoxParser {

    // Constructor
    pub fn new(toks: &Vec<Token>) -> Self {
        LoxParser {
            tokens: toks.clone(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    // Parse the Input
    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }
    
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.comparison()?;
        while self.match_token(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }
    
    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.term()?;
        while self.match_token(&vec![TokenType::Greater, TokenType::GreaterEqual,
                                      TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.factor()?;
        while self.match_token(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut left = self.unary()?;
        while self.match_token(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let tok = self.advance();
        match tok.typ {
            TokenType::False => Ok(Expr::False),
            TokenType::True => Ok(Expr::True),
            TokenType::Nil => Ok(Expr::Nil),
            TokenType::Number(_) => Ok(Expr::Literal(tok)),
            TokenType::QuotedString(_) => Ok(Expr::Literal(tok)),
            TokenType::Identifier(_) => Ok(Expr::Identifier(tok)),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                Ok(Expr::Grouping(Box::new(expr)))
            },
            _ => loxerr!("Invalid token!")
        }
    }

    // Supporting Methods
    
    fn match_token(&mut self, expected: &Vec<TokenType>) -> bool {
        for exp in expected.iter() {
            if self.check(exp) {
                let _ignored = self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, expected: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().typ == expected
    }
    
    fn advance(&mut self) -> Token {
        if ! self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.peek().typ == TokenType::Eof
    }
    
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    
    fn previous(&self) -> Token {
        self.tokens[self.current-1].clone()
    }
    
}

macro_rules! parser_test {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let src: &str = $src;
            let exp = $should_be;
            let mut scanner = Scanner::new(src);
            let mut parser = LoxParser::new(&scanner.scan_tokens().unwrap().clone());
            let got = parser.parse();
            assert_eq!(got, exp);
        }
    };
}

parser_test!(test_parse_12, "12" => Ok(Expr::Literal(Token::new(TokenType::Number(12.0), 1))));
parser_test!(test_addition, "1 + 2 == 3;" =>
             Ok(Expr::Binary(
                 Box::new(Expr::Binary(
                     Box::new(Expr::Literal(Token::new(TokenType::Number(1.0), 1))),
                     Token::new(TokenType::Plus, 1),
                     Box::new(Expr::Literal(Token::new(TokenType::Number(2.0), 1)))
                 )),
                 Token::new(TokenType::EqualEqual, 1),
                 Box::new(Expr::Literal(Token::new(TokenType::Number(3.0), 1)))
             )
             )
);
