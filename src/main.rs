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
    let n: usize = cin().parse()?;
    let mut ans = 0;
    // let m = (n as f64).powf(1.0 / 3.0).floor() as usize;
    // // root^3(n)以下の数を走査すれば十分
    // for i in 1..=m {
    //     // root^2(n / i)以下の数を走査すればよい
    //     for j in i..=(n as f64 / i as f64).powf(0.5).floor() as usize {
    //         let k_max = (n as f64 / (i as f64 * j as f64)).floor() as usize;
    //         if k_max < j {
    //             continue;
    //         }
    //         ans += k_max - j + 1;
    //     }
    // }

    // 上の書き方だと小数の誤差によりWA

    for i in 1..=n {
        if i * i * i > n {
            break;
        }
        for j in i..=n {
            if j * j * i > n {
                break;
            }
            let k_max = (n as f64 / (i as f64 * j as f64)).floor() as usize;
            if k_max < j {
                continue;
            }
            ans += k_max - j + 1;
        }
    }

    println!("{}", ans);
    Ok(())
}
