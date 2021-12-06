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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, y) = {
        let ny = cin_vec::<usize>();
        (ny[0], ny[1])
    };

    let mut ans = None;

    for bil_1000 in 0..=n {
        for bil_5000 in 0..=(n - bil_1000) {
            let bil_10000 = n - bil_1000 - bil_5000;

            if bil_1000 * 1_000 + bil_5000 * 5_000 + bil_10000 * 10_000 == y {
                ans = Some((bil_10000, bil_5000, bil_1000));
                break;
            }
        }
    }

    match ans {
        Some(ans) => {
            println!("{} {} {}", ans.0, ans.1, ans.2);
        }
        None => {
            println!("{} {} {}", -1, -1, -1);
        }
    }

    Ok(())
}
