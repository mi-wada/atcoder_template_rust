use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

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

// TODO: ToTupleトレイトにしてvecに実装しよう
fn tuple_2<T>(v: Vec<T>) -> (T, T)
where
    T: Copy,
{
    (v[0], v[1])
}

fn prime_factors(mut x: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 2..=((x as f64).sqrt() as usize + 1) {
        if x % i == 0 {
            res.push(i);
            while x % i == 0 {
                x /= i;
            }
        }
    }
    if x != 1 {
        res.push(x);
    }
    res
}

fn main() {
    let (n, m) = tuple_2(cin_vec::<usize>());
    let a = cin_vec::<usize>();

    let mut ans: HashSet<usize> = (1..=m).collect();

    for _a in a {
        for prime_factor in prime_factors(_a) {
            if ans.contains(&prime_factor) {
                let mut mult = 1;
                while prime_factor * mult <= m {
                    ans.remove(&(prime_factor * mult));
                    mult += 1;
                }
            }
        }
    }

    println!("{}", ans.len());
    let mut ans: Vec<usize> = ans.into_iter().collect();
    ans.sort();
    ans.iter().for_each(|x| println!("{}", x));
}
