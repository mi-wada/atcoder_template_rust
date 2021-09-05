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
    let n: usize = cin().parse().unwrap();
    let a: Vec<usize> = cin_vec();

    let mut appearances_table: HashMap<usize, usize> = HashMap::new();

    for v in a.iter() {
        *appearances_table.entry(*v).or_insert(0) += 1;
    }

    let mut ans = 0;

    for i in 0..n {
        ans += (n - i) - appearances_table[&a[i]];
        *appearances_table.get_mut(&a[i]).unwrap() -= 1;
    }

    println!("{}", ans);
}
