use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

#[allow(dead_code)]
fn cin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

trait ToTuple<T> {
    fn to_2(self) -> (T, T);
    fn to_3(self) -> (T, T, T);
    fn to_4(self) -> (T, T, T, T);
}

impl<T> ToTuple<T> for Vec<T>
where
    T: Clone,
{
    fn to_2(self) -> (T, T) {
        (self.get(0).unwrap().clone(), self.get(1).unwrap().clone())
    }

    fn to_3(self) -> (T, T, T) {
        (
            self.get(0).unwrap().clone(),
            self.get(1).unwrap().clone(),
            self.get(2).unwrap().clone(),
        )
    }

    fn to_4(self) -> (T, T, T, T) {
        (
            self.get(0).unwrap().clone(),
            self.get(1).unwrap().clone(),
            self.get(2).unwrap().clone(),
            self.get(3).unwrap().clone(),
        )
    }
}

fn solve(k: usize) -> String {
    let mut ret = String::new();
    let mut k = k;
    while k > 0 {
        ret.push(if k % 2 == 0 { '0' } else { '2' });

        k = k >> 1;
    }
    ret.chars().rev().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let k = cin().parse::<usize>().unwrap();
    println!("{}", solve(k));

    Ok(())
}
