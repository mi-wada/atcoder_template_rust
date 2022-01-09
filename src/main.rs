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

fn is_overflow(a: usize, b: usize) -> bool {
    if b == 0 {
        return false;
    }

    a > usize::max_value() / b
}

fn solve(
    num_of_bags: usize,
    desired: usize,
    balls_by_bag: &HashMap<usize, Vec<usize>>,
    bag_i: usize,
    acc: usize,
) -> usize {
    if bag_i == num_of_bags {
        if desired == acc {
            1
        } else {
            0
        }
    } else {
        let mut ret = 0;
        for ball in balls_by_bag.get(&bag_i).unwrap() {
            ret += if is_overflow(acc, *ball) {
                0
            } else {
                solve(num_of_bags, desired, balls_by_bag, bag_i + 1, acc * *ball)
            };
        }
        ret
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, x) = cin_vec::<usize>().to_2();
    let balls_by_bag: HashMap<usize, Vec<usize>> = (0..n)
        .map(|i| (i, cin_vec::<usize>().into_iter().skip(1).collect()))
        .collect();

    println!("{}", solve(n, x, &balls_by_bag, 0, 1));
    Ok(())
}
