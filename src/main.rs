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

enum BinaryTree {
    Nil,
    Node {
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
        value: usize,
    },
}

impl BinaryTree {
    fn new_node(value: usize) -> BinaryTree {
        BinaryTree::Node {
            left: Box::new(BinaryTree::Nil),
            right: Box::new(BinaryTree::Nil),
            value,
        }
    }
}

impl BinaryTree {
    fn insert(&mut self, v: usize) {
        match self {
            BinaryTree::Nil => *self = BinaryTree::new_node(v),
            BinaryTree::Node { left, right, value } => {
                if v < *value {
                    left.insert(v)
                } else {
                    right.insert(v)
                }
            }
        }
    }

    fn upper(&self, v: usize, res: Option<usize>) -> Option<usize> {
        match self {
            BinaryTree::Nil => res,
            BinaryTree::Node { left, right, value } => {
                if *value > v {
                    left.upper(
                        v,
                        Some(std::cmp::min(res.unwrap_or(usize::max_value()), *value)),
                    )
                } else {
                    right.upper(v, res)
                }
            }
        }
    }

    fn lower(&self, v: usize, res: Option<usize>) -> Option<usize> {
        match self {
            BinaryTree::Nil => res,
            BinaryTree::Node { left, right, value } => {
                if *value < v {
                    right.lower(
                        v,
                        Some(std::cmp::max(res.unwrap_or(usize::min_value()), *value)),
                    )
                } else {
                    left.lower(v, res)
                }
            }
        }
    }
}

fn main() {
    let (l, q) = {
        let l_q = cin_vec::<usize>();
        (l_q[0], l_q[1])
    };

    let (mut c, mut x) = (vec![], vec![]);

    for _ in 0..q {
        let c_x = cin_vec::<usize>();
        c.push(c_x[0]);
        x.push(c_x[1]);
    }

    let mut tree = BinaryTree::Nil;
    tree.insert(0);
    tree.insert(l);

    for i in 0..q {
        match c[i] {
            1 => {
                tree.insert(x[i]);
            }
            2 | _ => {
                println!(
                    "{}",
                    tree.upper(x[i], None).unwrap() - tree.lower(x[i], None).unwrap()
                );
            }
        }
    }
}
