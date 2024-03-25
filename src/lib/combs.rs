use crate::defs::{Expr, Op};

pub fn subs<T>(src: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if src.is_empty() {
        vec![vec![]]
    } else {
        let mut res = Vec::<Vec<T>>::new();

        let elem = &src[0];

        let rest = if src.len() > 1 {
            &src[1..]
        } else {
            &[] as &[T]
        };

        let mut next = subs(rest);

        res.append(&mut next);

        subs(rest).iter_mut().for_each(|v| {
            v.insert(0, elem.clone());
            res.push(v.to_vec());
        });

        res
    }
}

pub fn interleave<T>(item: T, with: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    if with.is_empty() {
        vec![vec![item]]
    } else {
        let mut res = Vec::<Vec<T>>::new();

        let mut vec = vec![item.clone()];
        vec.append(&mut with.to_vec());

        res.push(vec);

        let elem = &with[0];

        let rest = if with.len() > 1 {
            &with[1..]
        } else {
            &[] as &[T]
        };

        interleave(item, rest).iter_mut().for_each(|v| {
            v.insert(0, elem.clone());
            res.push(v.to_vec());
        });

        res
    }
}

pub fn perms<T>(src: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut res = Vec::<Vec<T>>::new();

    if src.len() > 0 {
        let elem = &src[0];

        let rest = if src.len() > 1 {
            &src[1..]
        } else {
            &[] as &[T]
        };

        perms(rest).iter_mut().for_each(|v| {
            interleave(elem.clone(), v).into_iter().for_each(|v| {
                res.push(v);
            });
        });
    } else {
        res.push(Vec::<T>::new());
    }

    res
}

pub fn choices<T>(src: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut res = Vec::<Vec<T>>::new();

    subs(&src).into_iter().for_each(|v| {
        perms(&v).into_iter().for_each(|r| {
            if !r.is_empty() {
                res.push(r);
            }
        });
    });

    res
}

pub fn split<T>(src: &[T]) -> Vec<(Vec<T>, Vec<T>)>
where
    T: Clone,
{
    let mut res = Vec::<(Vec<T>, Vec<T>)>::new();

    if src.len() > 1 {
        let elem = &src[0];

        let rest = &src[1..];

        res.push((vec![elem.clone()], rest.to_vec()));

        split(rest).into_iter().for_each(|mut pair| {
            pair.0.insert(0, elem.clone());
            res.push(pair);
        });
    }

    res
}

pub fn combine(l: Expr, r: Expr) -> Vec<Expr> {
    let mut res = Vec::<Expr>::new();

    [Op::Add, Op::Sub, Op::Mul, Op::Div]
        .into_iter()
        .for_each(|op| {
            res.push(Expr::new_expr(op, l.clone(), r.clone()));
        });

    res
}

pub fn exprs(src: &[i32]) -> Vec<Expr> {
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

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use crate::defs::eval;

    use super::*;

    #[test]
    fn empty() {
        let res = subs::<usize>(&[]);
        assert_eq!(res, vec![vec![]]);
    }

    #[test]
    fn three() {
        let res = subs(&[1, 2, 3]);

        assert_eq!(
            res,
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![2, 3],
                vec![1],
                vec![1, 3],
                vec![1, 2],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn three_gen_exp() {
        let res = subs(&[1, 2, 3]).into_iter().sorted().collect::<Vec<_>>();
        let exp = (1..4).powerset().sorted().collect::<Vec<_>>();

        assert_eq!(res, exp);
    }

    #[test]
    fn interleave_one() {
        let res = interleave(3, &[1, 2, 4]);

        println!("res: {:?}", res);
    }

    #[test]
    fn perms_three() {
        let res = perms(&[1, 2, 3]);

        println!("res: {:?}", res);
    }

    #[test]
    fn split_empty() {
        let res = split::<usize>(&[]);
        let exp = Vec::<(Vec<usize>, Vec<usize>)>::new();

        assert_eq!(res, exp);
    }

    #[test]
    fn split_single() {
        let res = split::<usize>(&[1]);
        let exp = Vec::<(Vec<usize>, Vec<usize>)>::new();

        assert_eq!(res, exp);
    }

    #[test]
    fn split_two() {
        let res = split::<usize>(&[1, 2]);
        let mut exp = Vec::<(Vec<usize>, Vec<usize>)>::new();
        exp.push((vec![1], vec![2]));

        assert_eq!(res, exp);
    }

    #[test]
    fn split_three() {
        let res = split::<usize>(&[1, 2, 3]);
        let mut exp = Vec::<(Vec<usize>, Vec<usize>)>::new();
        exp.push((vec![1], vec![2, 3]));
        exp.push((vec![1, 2], vec![3]));

        assert_eq!(res, exp);
    }

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

        choices(&input).into_iter().for_each(|choice| {
            let res = exprs(&choice);

            const WANT: i32 = 6;

            res.iter().for_each(|ex| {
                if let Some(res) = eval(&ex) {
                    if res == WANT {
                        println!("ex: {ex} = {}", res)
                    }
                }
            });

            // println!("{} expressions from {input:?}", res.len());
        });
    }

    #[test]
    fn want_608() {
        let input = [50, 25, 75, 100, 4, 1];

        let mut tot_results = 0;
        let mut tot_exprs = 0;

        choices(&input).into_iter().for_each(|choice| {
            let res = exprs(&choice);

            const WANT: i32 = 608;

            res.iter().for_each(|ex| {
                tot_exprs += 1;

                if let Some(res) = eval(&ex) {
                    if res == WANT {
                        tot_results += 1;
                        println!("#{}: {ex} = {}", tot_results, res);
                    }
                }
            });
        });

        println!("{}/{} expressions from {input:?}", tot_results, tot_exprs);
    }
}
