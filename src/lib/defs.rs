use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Mod,
}

impl TryFrom<char> for Op {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Op::Add),
            '-' => Ok(Op::Sub),
            '*' => Ok(Op::Mul),
            '/' => Ok(Op::Div),
            '^' => Ok(Op::Exp),
            '%' => Ok(Op::Mod),
            _ => Err("Invalid op - only + - * / ^ are allowed"),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Exp => write!(f, "^"),
            Op::Mod => write!(f, "%"),
        }
    }
}

pub type OpsType = Vec<Op>;

pub fn all_ops() -> Vec<Op> {
    [Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Exp, Op::Mod].to_vec()
}

pub fn std_ops() -> Vec<Op> {
    [Op::Add, Op::Sub, Op::Mul, Op::Div].to_vec()
}

#[derive(Clone, Debug)]
pub enum Expr {
    Val(i32),
    Expr(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn brak_fmt(e: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match e {
            Expr::Val(v) => write!(f, "{}", v),
            Expr::Expr(op, a, b) => {
                write!(f, "(")?;
                Expr::brak_fmt(a, f)?;
                write!(f, " {} ", op)?;
                Expr::brak_fmt(b, f)?;
                write!(f, ")")
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Val(v) => write!(f, "{}", v),
            Expr::Expr(op, a, b) => {
                Expr::brak_fmt(&*a, f)?;
                write!(f, " {} ", op)?;
                Expr::brak_fmt(&*b, f)
            }
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Val(l0), Self::Val(r0)) => l0 == r0,
            (Self::Expr(l0, l1, l2), Self::Expr(r0, r1, r2)) => {
                if l0 == &Op::Add || l0 == &Op::Mul {
                    l0 == r0 && ((l1 == r1 && l2 == r2) || (l1 == r2 && l2 == r1))
                } else {
                    l0 == r0 && l1 == r1 && l2 == r2
                }
            }
            _ => false,
        }
    }
}

pub fn valid(op: &Op, a: i32, b: i32) -> bool {
    // Optimised checks to remove redundant operations and reversed duplicates
    return match op {
        Op::Add => a <= b,
        Op::Sub => a > b,
        Op::Mul => a != 1 && b != 1 && a <= b && a.checked_mul(b).is_some(),
        Op::Div => b > 1 && a % b == 0,
        Op::Exp => a > 1 && b > 1 && a.checked_pow(b as u32).is_some(),
        Op::Mod => a > 0 && b != 0,
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
                    Op::Exp => Some(a.pow(b as u32)),
                    Op::Mod => Some(a % b),
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
    fn read_good_op() {
        let res = Op::try_from('+');

        assert_eq!(res, Ok(Op::Add));
    }

    #[test]
    fn read_bad_op() {
        let res = Op::try_from('~');

        assert!(res.is_err());
    }

    #[test]
    fn simple_val() {
        let expr = Expr::new_val(10);

        let result = eval(&expr);
        println!("expr: {} = {:?}", expr, result);

        assert_eq!(result, Some(10));
    }

    #[test]
    fn simple_add() {
        // NOTE: Must be a 'valid' expression (optimisation)
        let expr = Expr::new_expr(Op::Add, Expr::new_val(10), Expr::new_val(15));

        let result = eval(&expr);
        println!("expr: {} = {:?}", expr, result);

        assert_eq!(result, Some(25));
    }

    #[test]
    fn nested_expr() {
        // NOTE: Must be a 'valid' expression (optimisation)
        let expr = Expr::new_expr(
            Op::Mul,
            Expr::new_expr(
                Op::Sub,
                Expr::new_expr(Op::Add, Expr::new_val(4), Expr::new_val(29)),
                Expr::new_expr(Op::Div, Expr::new_val(100), Expr::new_val(5)),
            ),
            Expr::new_val(30),
        );

        let result = eval(&expr);
        println!("expr: {} = {:?}", expr, result);

        assert_eq!(result, Some(390));
    }

    #[test]
    fn expr_eq_add_vals() {
        let e1 = Expr::new_expr(Op::Add, Expr::new_val(10), Expr::Val(100));
        let e2 = Expr::new_expr(Op::Add, Expr::new_val(100), Expr::Val(10));

        assert_eq!(e1, e2);
    }

    #[test]
    fn expr_eq_mul_vals() {
        let e1 = Expr::new_expr(Op::Mul, Expr::new_val(10), Expr::Val(100));
        let e2 = Expr::new_expr(Op::Mul, Expr::new_val(100), Expr::Val(10));

        assert_eq!(e1, e2);
    }

    #[test]
    fn expr_ne_sub_vals() {
        let e1 = Expr::new_expr(Op::Sub, Expr::new_val(10), Expr::Val(100));
        let e2 = Expr::new_expr(Op::Sub, Expr::new_val(100), Expr::Val(10));

        assert_ne!(e1, e2);
    }

    #[test]
    fn expr_ne_div_vals() {
        let e1 = Expr::new_expr(Op::Div, Expr::new_val(10), Expr::Val(100));
        let e2 = Expr::new_expr(Op::Div, Expr::new_val(100), Expr::Val(10));

        assert_ne!(e1, e2);
    }

    #[test]
    fn expr_eq_add_expr() {
        let e1 = Expr::new_expr(
            Op::Add,
            Expr::new_val(10),
            Expr::new_expr(
                Op::Mul,
                Expr::new_val(10),
                Expr::new_expr(Op::Mul, Expr::new_val(10), Expr::new_val(3)),
            ),
        );
        let e2 = Expr::new_expr(
            Op::Add,
            Expr::new_expr(
                Op::Mul,
                Expr::new_expr(Op::Mul, Expr::new_val(10), Expr::new_val(3)),
                Expr::new_val(10),
            ),
            Expr::new_val(10),
        );

        assert_eq!(e1, e2);

        println!("{} == {} ?", e1, e2);

        assert!(!std::ptr::eq(&e1, &e2));
    }

    #[test]
    fn eval_exp() {
        let expr = Expr::new_expr(Op::Exp, Expr::new_val(4), Expr::new_val(5));

        let res = eval(&expr);

        assert_eq!(Some(1024), res);
    }
}
