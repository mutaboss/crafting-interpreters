use crate::error::LoxError;
use crate::parser::Expr;
use crate::parser::*;
use crate::scanner::*;

fn get_number(n: &Expr) -> Result<f64, LoxError> {
    if let Expr::Literal(n) = n {
        if let TokenType::Number(n) = n.typ {
            return Ok(n);
        }
    }
    loxerr!("A number was expected but not found.")
}

fn get_string(e: &Expr) -> Result<String, LoxError> {
    if let Expr::Literal(e) = e {
        if let TokenType::QuotedString(qs) = &e.typ {
            return Ok(qs.to_string());
        }
    }
    loxerr!("A String was expected but not found.")
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

macro_rules! compare_op {
    // FIXME: Add support for Strings
    ($left:expr, $right:expr, $exec:block) => {{
        if let Ok(nl) = get_number($left) {
            if let Ok(nr) = get_number($right) {
                if $exec(nl, nr) {
                    return Ok(Expr::True);
                } else {
                    return Ok(Expr::False);
                }
            }
        }
        loxerr!("Invalid inputs to comparison.")
    }};
}

pub struct Interpreter {}

macro_rules! tt_to_expr {
    ($exprname:ident, $ttname:ident) => {
        Expr::$exprname(Token::new(TokenType::$ttname, 0))
    };
    ($exprname:ident, $ttname:ident, $ttval:expr) => {
        Expr::$exprname(Token::new(TokenType::$ttname($ttval), 0))
    };
}

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
            Expr::Grouping(expr) => self.interpret(expr),
            Expr::Identifier(_tok) => {
                loxerr!("Identifier not implemented")
            }
        }
    }

    fn evaluate_binary(&self, left: &Expr, oper: &Token, right: &Expr) -> Result<Expr, LoxError> {
        let left = self.interpret(&left)?;
        let right = self.interpret(&right)?;
        match oper.typ {
            TokenType::Plus => {
                if get_number(&left).is_ok() && get_number(&right).is_ok() {
                    math_op!(&left, &right, { |x, y| x + y })
                } else if get_string(&left).is_ok() && get_string(&right).is_ok() {
                    let mut s1 = get_string(&left).unwrap().clone();
                    let s2 = get_string(&right).unwrap();
                    s1.push_str(&s2);
                    Ok(Expr::Literal(Token::new(
                        TokenType::QuotedString(s1.to_string()),
                        0,
                    )))
                } else {
                    loxerr!("Invalid arguments to '+'.")
                }
            }
            TokenType::Star => math_op!(&left, &right, { |x, y| x * y }),
            TokenType::Minus => math_op!(&left, &right, { |x, y| x - y }),
            TokenType::Slash => math_op!(&left, &right, { |x, y| x / y }),
            TokenType::Greater => compare_op!(&left, &right, { |x, y| x > y }),
            TokenType::GreaterEqual => compare_op!(&left, &right, { |x, y| x >= y }),
            TokenType::Less => compare_op!(&left, &right, { |x, y| x < y }),
            TokenType::LessEqual => compare_op!(&left, &right, { |x, y| x <= y }),
            TokenType::EqualEqual => compare_op!(&left, &right, { |x, y| x == y }),
            TokenType::BangEqual => compare_op!(&left, &right, { |x, y| x != y }),
            _ => {
                loxerr!("Token not recognized")
            }
        }
    }

    fn evaluate_unary(&self, tok: &Token, expr: &Expr) -> Result<Expr, LoxError> {
        match tok.typ {
            TokenType::Minus => match self.interpret(expr)? {
                Expr::Literal(tok) => match tok.typ {
                    TokenType::Number(n) => Ok(tt_to_expr!(Literal, Number, -n)),
                    _ => loxerr!("Unsupported target token of unary minus: {}", tok),
                },
                _ => loxerr!("Unsupported target expression of unary minus: {}", expr),
            },
            TokenType::Bang => match self.interpret(expr)? {
                Expr::False => Ok(Expr::True),
                Expr::Nil => Ok(Expr::True),
                _ => Ok(Expr::False),
            },
            _ => loxerr!("Invalid unary operator: {}", tok),
        }
    }
}

#[cfg(test)]
fn interpret_input(input: &str) -> Result<Expr, LoxError> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(&tokens.clone());
    let tree = parser.parse()?;
    let interp = Interpreter::new();
    interp.interpret(&tree)
}

macro_rules! test_interpreter {
    ( UNARY: $ident:ident, $strng:expr, $exp:expr ) => {
        #[cfg(test)]
        #[test]
        fn $ident() -> Result<(), LoxError> {
            let result = interpret_input($strng);
            if let Ok(res) = result {
                assert_eq!($exp, res);
                Ok(())
            } else {
                loxerr!("Bad use of unary operator: {:?}", result);
            }
        }
    };
    ( BINARY_MATH: $ident:ident, $strng:expr, $exp:expr ) => {
        #[cfg(test)]
        #[test]
        fn $ident() -> Result<(), LoxError> {
            let result = interpret_input($strng);
            let exp = Expr::Literal(Token::new(TokenType::Number($exp as f64), 0));
            if let Ok(res) = result {
                assert_eq!(exp, res);
                Ok(())
            } else {
                loxerr!("Bad use of binary math operator: {:?}", result);
            }
        }
    };
    ( BINARY_STRING: $ident:ident, $strng:expr, $exp:expr ) => {
        #[cfg(test)]
        #[test]
        fn $ident() -> Result<(), LoxError> {
            let result = interpret_input($strng);
            let exp = Expr::Literal(Token::new(TokenType::QuotedString($exp), 0));
            if let Ok(res) = result {
                assert_eq!(exp, res);
                Ok(())
            } else {
                loxerr!("Bad use of binary string operator: {:?}", result);
            }
        }
    };
    ( BINARY_COMPARE: $ident:ident, $strng:expr, $exp:expr ) => {
        #[cfg(test)]
        #[test]
        fn $ident() -> Result<(), LoxError> {
            let result = interpret_input($strng);
            let exp = match $exp {
                true => Expr::True,
                false => Expr::False,
            };
            if let Ok(res) = result {
                assert_eq!(exp, res);
                Ok(())
            } else {
                loxerr!("Bad use of binary comparison operator: {:?}", result);
            }
        }
    };
}

test_interpreter!(UNARY: test_unary_false, "!false", Expr::True);
test_interpreter!(UNARY: test_unary_true, "!true", Expr::False);
test_interpreter!(UNARY: test_unary_nil, "!nil", Expr::True);
test_interpreter!(
    UNARY: test_unary_minus_neg,
    "-5",
    tt_to_expr!(Literal, Number, -5.0)
);
test_interpreter!(
    UNARY: test_unary_minus_pos,
    "-(-5)",
    tt_to_expr!(Literal, Number, 5.0)
);

test_interpreter!(BINARY_MATH: test_binary_addition, "3 + 2", 5.0);
test_interpreter!(BINARY_MATH: test_binary_subtraction, "15 - 3", 12);
test_interpreter!(BINARY_MATH: test_binary_multiplication, "15 * 4", 60);
test_interpreter!(BINARY_MATH: test_binary_division, "15 / 3", 5);
test_interpreter!(BINARY_MATH: test_grouping, "(15 / 3) * 4", 20);
test_interpreter!(BINARY_MATH: test_grouping_spaces, "( 15 / 3 ) * 4", 20);
test_interpreter!(
    BINARY_STRING: test_concat,
    "\"abc\" + \"def\"",
    "abcdef".to_string()
);
test_interpreter!(BINARY_COMPARE: test_greater_than, "5 + 12 > 15", true);
test_interpreter!(BINARY_COMPARE: test_greater_than_2, "5 +5 > 15", false);
test_interpreter!(BINARY_COMPARE: test_less_than, "5 + 12 < 20", true);
test_interpreter!(BINARY_COMPARE: test_less_than_2, "5 + 5 < 3", false);
test_interpreter!(BINARY_COMPARE: test_ge, "15 >= 15", true);
test_interpreter!(BINARY_COMPARE: test_ge_2, "15 >= 12", true);
test_interpreter!(BINARY_COMPARE: test_ge_3, "15 >= 20", false);
test_interpreter!(BINARY_COMPARE: test_le, "15 <= 15", true);
test_interpreter!(BINARY_COMPARE: test_le_2, "15 <= 16", true);
test_interpreter!(BINARY_COMPARE: test_le_3, "15 <= 5", false);
test_interpreter!(BINARY_COMPARE: test_eq_success, "5 == 5", true);
test_interpreter!(BINARY_COMPARE: test_eq_failure, "5 == 6", false);
test_interpreter!(BINARY_COMPARE: test_ne_success, "5 != 6", true);
test_interpreter!(BINARY_COMPARE: test_ne_failure, "5 != 5", false);
