use std::collections::{HashMap, HashSet};

fn cin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

fn cin_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    cin()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn cin_vertical<T>(n: usize) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut res = Vec::new();
    for _ in 0..n {
        res.push(cin().parse().unwrap());
    }
    res
}

fn tuple_2<T>(v: Vec<T>) -> (T, T)
where
    T: Copy,
{
    (v[0], v[1])
}

fn main() {
    let n: usize = cin().parse().unwrap();
    let (x, y): (Vec<i32>, Vec<i32>) = (0..n).map(|_| tuple_2(cin_vec::<i32>())).unzip();
    let mut x_with_idx: Vec<(usize, i32)> = x.into_iter().enumerate().collect();
    let mut y_with_idx: Vec<(usize, i32)> = y.into_iter().enumerate().collect();
    x_with_idx.sort_by(|(_a_idx, a), (_b_idx, b)| a.cmp(b));
    y_with_idx.sort_by(|(_a_idx, a), (_b_idx, b)| a.cmp(b));
    let mut answer_candidate = vec![
        x_with_idx[n - 1].1 - x_with_idx[0].1,
        x_with_idx[n - 1].1 - x_with_idx[1].1,
        x_with_idx[n - 2].1 - x_with_idx[0].1,
        y_with_idx[n - 1].1 - y_with_idx[0].1,
        y_with_idx[n - 1].1 - y_with_idx[1].1,
        y_with_idx[n - 2].1 - y_with_idx[0].1,
    ];
    answer_candidate.sort();
    if (x_with_idx[0].0, x_with_idx[n - 1].0) == (y_with_idx[0].0, y_with_idx[n - 1].0) {
        println!("{}", answer_candidate[6 - 3]);
    } else {
        println!("{}", answer_candidate[6 - 2]);
    }
}
