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

struct HashMapForSolve {
    v: HashMap<i32, i32>
}

impl HashMapForSolve {
    fn new() -> HashMapForSolve {
        HashMapForSolve {
            v: HashMap::<i32, i32>::new()
        }
    }

    fn insert(&mut self, k: i32) -> () {
        *self.v.entry(k).or_insert(0) += 1;
    }

    fn remove(&mut self, k: &i32) -> () {
        match self.v.remove(k).unwrap() {
            1 => (),
            v => { self.v.insert(*k, v - 1); }
        };
    }

    fn len(&self) -> usize {
        self.v.len()
    }
}

fn main() {
    let (n, k) = {
        let nk = cin_vec::<usize>();
        (nk[0], nk[1])
    };
    let c = cin_vec::<i32>();

    let mut color_set = HashMapForSolve::new();
    for i in 0..k {
        color_set.insert(c[i]);
    }

    let mut ans = color_set.len();

    for i in k..n {
        color_set.insert(c[i]);
        color_set.remove(&c[i - k]);
        ans = std::cmp::max(ans, color_set.len());
    }

    println!("{}", ans);
}
