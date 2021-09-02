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
    let npq = cin_vec::<String>();
    println!("n: {}, p: {}, q: {}", npq[0], npq[1], npq[2]);
}
