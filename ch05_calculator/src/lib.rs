#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Square(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidInput,
    MissingOperand,
    TooManyOperand,
    EmptyInput,
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Box<Expr>> = Vec::new();
    for word in input.split_ascii_whitespace() {
        let expr = match word {
            "sqr" => {
                let operand = stack.pop().unwrap();
                Expr::Square(operand)
            }
            "+" => {
                let operand1 = stack.pop().ok_or(ParseError::MissingOperand)?;
                let operand2 = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Add(operand1, operand2)
            }
            "-" => {
                let operand1 = stack.pop().ok_or(ParseError::MissingOperand)?;
                let operand2 = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Sub(operand1, operand2)
            }
            "*" => {
                let operand1 = stack.pop().ok_or(ParseError::MissingOperand)?;
                let operand2 = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Mul(operand1, operand2)
            }
            "/" => {
                let operand2 = stack.pop().ok_or(ParseError::MissingOperand)?;
                let operand1 = stack.pop().ok_or(ParseError::MissingOperand)?;
                Expr::Div(operand1, operand2)
            }
            otherwise => {
                let res = otherwise
                    .parse::<i64>()
                    .map_err(|_| ParseError::InvalidInput)?;
                Expr::Number(res)
            }
        };
        stack.push(Box::new(expr))
    }
    let res = stack.pop();
    match res {
        Some(expr) if stack.is_empty() => Ok(*expr),
        Some(_) => Err(ParseError::TooManyOperand),
        None => Err(ParseError::EmptyInput),
    }
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ZeroDivision,
    InvalidExpression,
    Overflow,
    Underflow,
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Add(exp1, exp2) => {
            let res1 = eval(exp1)?;
            let res2 = eval(exp2)?;
            let res = res1.checked_add(res2).ok_or(EvalError::Overflow)?;
            Ok(res)
        }
        Expr::Sub(exp1, exp2) => {
            let res1 = eval(exp1)?;
            let res2 = eval(exp2)?;
            let res = res1.checked_sub(res2).ok_or(EvalError::Underflow)?;
            Ok(res)
        }
        Expr::Mul(exp1, exp2) => {
            let res1 = eval(exp1)?;
            let res2 = eval(exp2)?;
            Ok(res1 * res2)
        }
        Expr::Div(exp1, exp2) => {
            let res1 = eval(exp1)?;
            let res2 = eval(exp2)?;
            if res2 == 0 {
                return Err(EvalError::ZeroDivision);
            }
            Ok(res1 / res2)
        }
        Expr::Square(exp1) => {
            let res1 = eval(exp1)?;
            Ok(res1 * res1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "42";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn overflow() {
        let input = format!("{} 1 +", i64::MAX);
        let expr = parse(&input).unwrap();
        let res = eval(&expr).unwrap_err();
        assert_eq!(res, EvalError::Overflow);
    }

    #[test]
    fn underflow() {
        let input = format!("{} 1 -", i64::MIN);
        let expr = parse(&input).unwrap();
        let res = eval(&expr).unwrap_err();
        assert_eq!(res, EvalError::Underflow);
    }

    #[test]
    fn too_many_operands() {
        let input = "42 42 42 +";
        let err = parse(input).unwrap_err();
        assert_eq!(err, ParseError::TooManyOperand);
    }

    #[test]
    fn empty_input() {
        let input = "       ";
        let err = parse(input).unwrap_err();
        assert_eq!(err, ParseError::EmptyInput);
    }

    #[test]
    fn div() {
        let input = "84 2 /";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn divide_zero() {
        let input = "42 0 /";
        let expr = parse(input).unwrap();
        let err = eval(&expr).unwrap_err();
        assert_eq!(err, EvalError::ZeroDivision);
    }
}
