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

#[derive(PartialEq, Eq, Clone)]
enum Flag {
    NotVisited,
    Visited { color: bool },
}

impl Flag {
    fn reverse(&self) -> Flag {
        match self {
            Flag::NotVisited => Flag::NotVisited,
            Flag::Visited { color } => Flag::Visited { color: !color },
        }
    }
}
// Visitedの中にblackとwhiteがある

fn main() {
    let (n, q) = {
        let n_q = cin_vec::<usize>();
        (n_q[0], n_q[1])
    };
    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a_b = cin_vec::<usize>();
        graph[a_b[0] - 1].push(a_b[1] - 1);
        graph[a_b[1] - 1].push(a_b[0] - 1);
    }
    let mut c = vec![];
    let mut d = vec![];
    for _ in 0..q {
        let c_d = cin_vec::<usize>();
        c.push(c_d[0] - 1);
        d.push(c_d[1] - 1);
    }

    let mut colors = vec![Flag::NotVisited; n];
    colors[0] = Flag::Visited { color: true };
    let mut que = std::collections::VecDeque::new();
    que.push_back(0);

    while let Some(current_i) = que.pop_back() {
        for next_i in graph[current_i].iter() {
            match colors[*next_i] {
                Flag::NotVisited => {
                    colors[*next_i] = colors[current_i].reverse();
                }
                Flag::Visited { color: _ } => continue,
            }

            que.push_back(*next_i);
        }
    }

    for i in 0..q {
        if colors[c[i]] == colors[d[i]] {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}
