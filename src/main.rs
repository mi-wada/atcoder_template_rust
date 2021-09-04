use std::collections::HashMap;

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

fn main() {
    let (n, k) = {
        let nk = cin_vec::<usize>();
        (nk[0], nk[1])
    };
    let a = cin_vec::<i32>();
    let a_sorted = {
        let mut res = a.clone();
        res.sort();
        res
    };
    let a_compressed: HashMap<i32, usize> = a_sorted
        .iter()
        .enumerate()
        .map(|(idx, v)| (*v, idx))
        .collect();

    for i in 0..n {
        println!(
            "{}",
            if a_compressed[&a[i]] < k % n {
                (k / n) as i64 + 1
            } else {
                (k / n) as i64
            }
        )
    }
}
