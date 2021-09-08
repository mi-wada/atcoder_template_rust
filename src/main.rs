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

fn get_nearest_idx(s: &Vec<bool>, target: bool) -> Option<usize> {
    let mut res = None;
    for i in 0..s.len() {
        if s[i] == target {
            res = match res {
                None => Some(i),
                Some(n) => Some(std::cmp::min(n, i)),
            };
        }
        let j = (s.len() - i) % s.len();
        if s[j] == target {
            res = match res {
                None => Some(i),
                Some(n) => Some(std::cmp::min(n, i)),
            };
        }
    }
    res
}

fn main() {
    let (_n, _m) = {
        let n_m = cin_vec::<usize>();
        (n_m[0], n_m[1])
    };
    let s = cin_vec::<u8>().iter().map(|x| *x == 1).collect();
    let t: Vec<bool> = cin_vec::<u8>().iter().map(|x| *x == 1).collect();

    let mut pos = vec![None; 2];
    pos[false as usize] = get_nearest_idx(&s, false);
    pos[true as usize] = get_nearest_idx(&s, true);

    let mut ans = 0;
    let mut is_seeked = false;
    let mut current_value = s[0];

    for x in t {
        if current_value == x {
            ans += 1;
        } else {
            if !is_seeked {
                match pos[x as usize] {
                    None => {
                        ans = -1;
                        break;
                    }
                    Some(n) => {
                        ans += n as i32 + 1;
                    }
                }
                is_seeked = true;
                current_value = x;
            } else {
                ans += 2;
                current_value = !current_value;
            }
        }
    }

    println!("{}", ans);
}
