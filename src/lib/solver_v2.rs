use crate::combinatorics::{choices, split};
use crate::expr::{eval, valid, Expr, OpsType};

// First optimisation

pub type Result = (Expr, i32);

pub fn combine2(l: Result, r: Result, ops: &OpsType) -> Vec<Result> {
    let mut res = Vec::<Result>::new();

    ops.iter().for_each(|op| {
        // Check if each expression is valid and keep it and its value
        if valid(&op, l.1, r.1) {
            let expr = Expr::new_expr(op.clone(), l.0.clone(), r.0.clone());
            let val = eval(&expr).unwrap();
            res.push((expr, val));
        }
    });

    res
}

pub fn results(src: &[i32], ops: &OpsType) -> Vec<Result> {
    let mut res = Vec::<Result>::new();

    if src.len() == 1 {
        res.push((Expr::new_val(src[0].clone()), src[0]));
    } else {
        split(src).into_iter().for_each(|(l, r)| {
            results(&l, ops).into_iter().for_each(|le| {
                results(&r, ops).into_iter().for_each(|re| {
                    combine2(le.clone(), re.clone(), ops)
                        .into_iter()
                        .for_each(|e| {
                            res.push(e);
                        });
                });
            });
        });
    }

    res
}

pub fn solutions2(input: &[i32], target: i32, ops: &OpsType) -> (Vec<Result>, usize) {
    let mut total_checked: usize = 0;
    let mut res = Vec::<Result>::new();

    choices(&input).into_iter().for_each(|choice| {
        let results = results(&choice, ops);

        results.into_iter().for_each(|result| {
            total_checked += 1;

            if result.1 == target {
                res.push(result);
            }
        });
    });

    (res, total_checked)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::expr::std_ops;

    #[test]
    fn want_608_v2() {
        let input = [50, 25, 75, 100, 4, 1];

        let solns = solutions2(&input, TARGET, &std_ops());

        const TARGET: i32 = 608;

        println!(
            "{} solutions to make {} from {:?} - {} checked",
            solns.0.len(),
            TARGET,
            input,
            solns.1
        );

        solns.0.iter().for_each(|s| {
            println!("ex: {} = {}", s.0, s.1);
        });
    }
}
