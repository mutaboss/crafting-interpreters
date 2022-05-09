use crate::error::LoxError;
use crate::parser::Expr;
use crate::scanner::*;

fn get_number(n: &Expr) -> Result<f64, LoxError> {
    if let Expr::Literal(n) = n {
        if let TokenType::Number(n) = n.typ {
            return Ok(n);
        }
    }
    loxerr!("A number was expected but not found.")
}

macro_rules! math_op {
    ($left:expr, $right:expr, $exec:block) => {{
        let nl = get_number($left)?;
        let nr = get_number($right)?;
        Ok(Expr::Literal(Token::new(
            TokenType::Number($exec(nl, nr)),
            0,
        )))
    }};
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expr: &Expr) -> Result<Expr, LoxError> {
        match expr {
            Expr::False => Ok(Expr::False),
            Expr::True => Ok(Expr::True),
            Expr::Nil => Ok(Expr::Nil),
            Expr::Binary(left, oper, right) => self.evaluate_binary(left, oper, right),
            Expr::Unary(tok, right) => self.evaluate_unary(tok, right),
            Expr::Literal(tok) => {
                if let TokenType::Number(n) = tok.typ {
                    Ok(Expr::Literal(Token::new(TokenType::Number(n), tok.line)))
                } else if let TokenType::QuotedString(s) = &tok.typ {
                    Ok(Expr::Literal(Token::new(
                        TokenType::QuotedString(s.clone()),
                        tok.line,
                    )))
                } else {
                    loxerr!("Unrecognized literal: {}", tok)
                }
            }
            Expr::Identifier(_tok) => {
                loxerr!("Identifier not implemented")
            }
            Expr::Grouping(expr) => self.interpret(expr),
        }
    }

    fn evaluate_binary(&self, left: &Expr, oper: &Token, right: &Expr) -> Result<Expr, LoxError> {
        let left = self.interpret(&left)?;
        let right = self.interpret(&right)?;
        match oper.typ {
            TokenType::Plus => math_op!(&left, &right, { |x, y| x + y }),
            TokenType::Star => math_op!(&left, &right, { |x, y| x * y }),
            TokenType::Minus => math_op!(&left, &right, { |x, y| x - y }),
            TokenType::Slash => math_op!(&left, &right, { |x, y| x / y }),
            _ => {
                loxerr!("Token not recognized")
            }
        }
    }

    fn evaluate_unary(&self, tok: &Token, expr: &Expr) -> Result<Expr, LoxError> {
        match tok.typ {
            TokenType::Minus => match self.interpret(expr)? {
                Expr::Literal(tok) => match tok.typ {
                    TokenType::Number(n) => Ok(Expr::Literal(Token::new(TokenType::Number(-n), 0))),
                    _ => loxerr!("Unsupported target token of unary minus: {}", tok),
                },
                _ => loxerr!("Unsupported target expression of unary minus: {}", expr),
            },
            TokenType::Bang => {
                loxerr!("Unary ! is not yet supported.")
                // match self.interpret(expr)? {
                //     Expr::Identifier(ident) => {
                //         match ident.typ {
                //             _ => loxerr!("Unsupported target token of unary !: {}", ident),
                //         }
                //     },
                //     _ => loxerr!("Unsupported target expression of unary !: {}", expr),
                // }
            }
            _ => loxerr!("Invalid unary operator: {}", tok),
        }
    }
}
