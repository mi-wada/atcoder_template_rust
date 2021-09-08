use std::collections::{HashMap, HashSet, VecDeque};

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

fn dfs(g: &Vec<Vec<usize>>, from_idx: usize, current_idx: usize) {
    print!("{} ", current_idx + 1);

    for next_idx in g[current_idx].iter() {
        if *next_idx != from_idx {
            dfs(g, current_idx, *next_idx);
        }
    }
    if !(from_idx == current_idx && from_idx == 0) {
        print!("{} ", from_idx + 1);
    }
}

fn main() {
    let n: usize = cin().parse().unwrap();
    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a_b: Vec<usize> = cin_vec::<usize>().iter().map(|x| x - 1).collect();
        graph[a_b[0]].push(a_b[1]);
        graph[a_b[1]].push(a_b[0]);
    }
    for i in 0..n {
        graph[i].sort();
    }

    dfs(&graph, 0, 0);
    println!("");
}
