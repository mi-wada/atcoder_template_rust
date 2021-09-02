fn cin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

fn cin_vec<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    cin()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

fn main() {
    let n: usize = cin().parse().unwrap();
    let s = cin_vec::<i32>();
    let t = cin_vec::<i32>();

    let mut ans = vec![0; n as usize];
    for i in 0..n {
        ans[i] = t[i];
    }

    for i in 0..2*n {
        ans[(i + 1) % n] = std::cmp::min(ans[(i + 1) % n], ans[i % n] + s[i % n]);
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
