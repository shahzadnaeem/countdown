fn subs<T>(src: &[T]) -> Vec<Vec<T>>
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

fn interleave<T>(item: T, with: &[T]) -> Vec<Vec<T>>
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

fn perms<T>(src: &[T]) -> Vec<Vec<T>>
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

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;

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
}
