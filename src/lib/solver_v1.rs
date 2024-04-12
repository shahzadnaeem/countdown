use crate::combinatorics::{choices, split};
use crate::expr::{eval, Expr, Op};

// Simplest solution - brute force search
// NOTE: Includes optimised validity check

fn combine(l: Expr, r: Expr) -> Vec<Expr> {
    let mut res = Vec::<Expr>::new();

    [Op::Add, Op::Sub, Op::Mul, Op::Div]
        .into_iter()
        .for_each(|op| {
            res.push(Expr::new_expr(op, l.clone(), r.clone()));
        });

    res
}

fn exprs(src: &[i32]) -> Vec<Expr> {
    let mut res = Vec::<Expr>::new();

    if src.len() == 1 {
        res.push(Expr::new_val(src[0].clone()));
    } else {
        split(src).into_iter().for_each(|(l, r)| {
            exprs(&l).into_iter().for_each(|le| {
                exprs(&r).into_iter().for_each(|re| {
                    combine(le.clone(), re.clone()).into_iter().for_each(|e| {
                        res.push(e);
                    });
                });
            });
        });
    }

    res
}

pub fn solutions(input: &[i32], target: i32) -> (Vec<Expr>, usize) {
    let mut total_checked: usize = 0;
    let mut res = Vec::<Expr>::new();

    choices(&input).into_iter().for_each(|choice| {
        let exprs = exprs(&choice);

        exprs.into_iter().for_each(|ex| {
            total_checked += 1;

            if let Some(val) = eval(&ex) {
                if val == target {
                    res.push(ex);
                }
            }
        });
    });

    (res, total_checked)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::expr::eval;

    #[test]
    fn combine_vals() {
        let res = combine(Expr::new_val(0), Expr::new_val(1));
        let mut exp = Vec::<Expr>::new();
        exp.push(Expr::new_expr(Op::Add, Expr::new_val(0), Expr::new_val(1)));
        exp.push(Expr::new_expr(Op::Sub, Expr::new_val(0), Expr::new_val(1)));
        exp.push(Expr::new_expr(Op::Mul, Expr::new_val(0), Expr::new_val(1)));
        exp.push(Expr::new_expr(Op::Div, Expr::new_val(0), Expr::new_val(1)));

        assert_eq!(res, exp);
    }

    #[test]
    fn exprs_two() {
        let res = exprs(&[1, 2]);

        res.iter().for_each(|ex| println!("ex: {ex}"));
    }

    #[test]
    fn exprs_three() {
        let input = [1, 2, 3];
        let res = exprs(&input);

        res.iter()
            .for_each(|ex| println!("ex: {ex} = {:?}", eval(&ex)));

        println!("{} expressions from {input:?}", res.len());
    }

    #[test]
    fn exprs_four() {
        let input = [1, 2, 3, 4];
        let res = exprs(&input);

        res.iter()
            .for_each(|ex| println!("ex: {ex} = {:?}", eval(&ex)));

        println!("{} expressions from {input:?}", res.len());
    }

    #[test]
    fn exprs_three_make_6() {
        let input = [1, 2, 3];

        const TARGET: i32 = 6;

        let solns = solutions(&input, TARGET);

        println!(
            "{} solutions to make {} from {:?} - {} checked",
            solns.0.len(),
            TARGET,
            input,
            solns.1
        );

        solns.0.iter().for_each(|s| {
            println!("ex: {} = {TARGET}", s);
        });
    }

    #[test]
    fn want_608() {
        // Using the original algorithm, this test takes too long!
        if false {
            let input = [50, 25, 75, 100, 4, 1];

            let solns = solutions(&input, TARGET);

            const TARGET: i32 = 608;

            println!(
                "{} solutions to make {} from {:?} - {} checked",
                solns.0.len(),
                TARGET,
                input,
                solns.1
            );

            solns.0.iter().for_each(|s| {
                println!("ex: {} = {TARGET}", s);
            });
        }
    }
}
