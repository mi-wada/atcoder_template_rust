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

fn main() {
    let s: Vec<char> = cin().chars().collect();
    let MOD = 1e9 as i32 + 7;
    let chars = ['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
    let mut dp: HashMap<String, Vec<i32>> = HashMap::new();
    let chokudai = "chokudai";
    for i in 0..chokudai.len() + 1 {
        dp.insert(
            chokudai.get(0..i).unwrap().to_string(),
            if i == 0 { vec![1; s.len() + 1]} else { vec![0; s.len() + 1] }
        );
    }

    for (c_i, c) in chars.iter().enumerate() {
        for i in 0..s.len() {
            let now_str = chokudai.get(0..c_i + 1).unwrap();
            dp.get_mut(now_str).unwrap()[i + 1] = dp[now_str][i];
            if s[i] == *c {
                let prev_str = chokudai.get(0..c_i).unwrap();
                dp.get_mut(now_str).unwrap()[i + 1] = (dp[now_str][i + 1] + dp.get(prev_str).unwrap()[i]) % MOD;
            }
        }
    }

    // println!("{:#?}", dp);

    println!("{}", dp.get("chokudai").unwrap()[s.len()] % MOD);
}
