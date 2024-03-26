use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Val(i32),
    Expr(Op, Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Val(v) => write!(f, "{}", v),
            Expr::Expr(op, a, b) => write!(f, "({} {} {})", a, op, b),
        }
    }
}

pub fn valid(op: &Op, a: i32, b: i32) -> bool {
    // Optimised checks to remove redundant operations and reversed duplicates
    return match op {
        Op::Add => a <= b,
        Op::Sub => a > b,
        Op::Mul => a != 1 && b != 1 && a <= b,
        Op::Div => b != 1 && a % b == 0,
    };
}

pub fn apply(op: &Op, a: &Expr, b: &Expr) -> Option<i32> {
    if let Some(a) = eval(a) {
        if let Some(b) = eval(b) {
            if valid(&op, a, b) {
                return match op {
                    Op::Add => Some(a + b),
                    Op::Sub => Some(a - b),
                    Op::Mul => Some(a * b),
                    Op::Div => Some(a / b),
                };
            }
        }
    }

    None
}

pub fn eval(expr: &Expr) -> Option<i32> {
    match expr {
        Expr::Val(v) => Some(*v),
        Expr::Expr(op, a, b) => apply(op, a, b),
    }
}

impl Expr {
    pub fn new_val(val: i32) -> Expr {
        Expr::Val(val)
    }

    pub fn new_expr(op: Op, a: Expr, b: Expr) -> Expr {
        Expr::Expr(op, Box::<Expr>::new(a), Box::<Expr>::new(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_val() {
        let expr = Expr::new_val(10);

        let result = eval(&expr);
        println!("expr: {} = {:?}", expr, result);

        assert_eq!(result, Some(10));
    }

    #[test]
    fn simple_add() {
        let expr = Expr::new_expr(Op::Add, Expr::new_val(10), Expr::new_val(0));

        let result = eval(&expr);
        println!("expr: {} = {:?}", expr, result);

        assert_eq!(result, Some(10));
    }

    #[test]
    fn nested_expr() {
        let expr = Expr::new_expr(
            Op::Mul,
            Expr::new_expr(
                Op::Sub,
                Expr::new_expr(Op::Add, Expr::new_val(3), Expr::new_val(4)),
                Expr::new_expr(Op::Div, Expr::new_val(100), Expr::new_val(5)),
            ),
            Expr::new_val(-1),
        );

        let result = eval(&expr);
        println!("expr: {} = {:?}", expr, result);

        assert_eq!(result, Some(13));
    }
}
