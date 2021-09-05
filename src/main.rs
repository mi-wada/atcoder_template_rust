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
    let (a, b, c) = {
        let abc = cin_vec::<i32>();
        (abc[0], abc[1], abc[2])
    };
    let ans = if c % 2 == 0 {
        if a.abs() == b.abs() {
            '='
        } else if a.abs() < b.abs() {
            '<'
        } else {
            '>'
        }
    } else {
        if a == b {
            '='
        } else if a < b {
            '<'
        } else {
            '>'
        }
    };

    println!("{}", ans);
}
