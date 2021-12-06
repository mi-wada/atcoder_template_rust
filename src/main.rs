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
    let rev_s = cin().chars().rev().collect::<String>();

    let rev_parts: Vec<String> = vec![
        "dream".to_string(),
        "dreamer".to_string(),
        "erase".to_string(),
        "eraser".to_string(),
    ]
    .into_iter()
    .map(|s| s.chars().rev().collect::<String>())
    .collect();

    let can_make = {
        let mut idx = 0;
        while idx < rev_s.len() {
            let (_, remaining) = rev_s.split_at(idx);
            let mut is_contain_part = false;
            for rev_part in rev_parts.iter() {
                if remaining.starts_with(rev_part) {
                    idx += rev_part.len();
                    is_contain_part = true;
                    break;
                }
            }
            if !is_contain_part {
                break;
            }
        }
        idx == rev_s.len()
    };

    println!("{}", if can_make { "YES" } else { "NO" });

    Ok(())
}
