use crate::combinatorics::{choices, split};
use crate::expr::{eval, valid, Expr, OpsType};

// First optimisation

pub type Result = (Expr, i32);

fn combine2(l: Result, r: Result, ops: &OpsType) -> Vec<Result> {
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

fn results(src: &[i32], ops: &OpsType) -> Vec<Result> {
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

pub fn dedup(all_solns: &Vec<Result>) -> (Vec<Result>, Vec<String>) {
    let mut deduped = Vec::<Result>::new();
    let mut dups_log = Vec::<String>::new();

    all_solns.clone().into_iter().for_each(|s| {
        if let Some(dup) = deduped.iter().find(|&dr| dr == &s) {
            dups_log.push(format!("{} == {}", s.0, dup.0));
        } else {
            deduped.push(s);
        }
    });

    (deduped, dups_log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{std_ops, Op};

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

    #[test]
    fn add_dups() {
        let input = [1, 2, 5];
        let ops = [Op::Add, Op::Mul].to_vec();

        let mut total: usize = 0;
        let mut res = Vec::<Result>::new();

        choices(&input).into_iter().for_each(|choice| {
            let results = results(&choice, &ops);

            results.into_iter().for_each(|result| {
                total += 1;

                res.push(result);
            });
        });

        let expected_res = 13;
        assert_eq!(res.len(), expected_res);

        // println!("Potential results: {total}");

        let (deduped, dups_log) = dedup(&res);
        let expected_dups = 2;

        assert_eq!(dups_log.len(), expected_dups);
        assert_eq!(deduped.len(), expected_res - expected_dups);

        // if !dups_log.is_empty() {
        //     println!("Duplicates: {} found", dups_log.len());

        //     dups_log.iter().for_each(|s| println!("  {s}"));
        // }

        // println!("Potential results: {}", deduped.len());
        // deduped.iter().for_each(|s| println!("  {} = {}", s.0, s.1));
    }
}
